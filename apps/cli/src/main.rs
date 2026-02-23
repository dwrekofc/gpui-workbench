use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use registry::plan::{
    ApplyFailureReport, DefaultLayout, FileMutation, FileAction, MutationStrategy, PlanContract,
    generate_plan,
};

// ---------------------------------------------------------------------------
// CLI output envelope (shared by all commands, FR-003)
// ---------------------------------------------------------------------------

/// Shared JSON output envelope for all CLI commands.
///
/// Ensures schema consistency across `add --plan`, `plan`, `apply`, and
/// future Phase 1 commands (`list`, `doctor`, `update`, `remove`).
#[derive(Debug, Serialize, Deserialize)]
pub struct CliOutput<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub errors: Vec<CliError>,
}

/// A structured error in CLI output.
#[derive(Debug, Serialize, Deserialize)]
pub struct CliError {
    pub code: String,
    pub message: String,
}

impl<T: Serialize> CliOutput<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            errors: Vec::new(),
        }
    }

    fn failure(data: T, errors: Vec<CliError>) -> Self {
        Self {
            success: false,
            data,
            errors,
        }
    }

    fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed to serialize CLI output")
    }
}

// ---------------------------------------------------------------------------
// CLI argument parsing
// ---------------------------------------------------------------------------

/// GPUI component toolkit CLI
#[derive(Parser)]
#[command(name = "gpui", version, about = "GPUI component toolkit - install, plan, and manage UI components")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a component to your project
    Add {
        /// Component name (e.g. dialog, select, tabs)
        component: String,
        /// Output the mutation plan as JSON instead of applying
        #[arg(long)]
        plan: bool,
        /// Target project directory (defaults to current directory)
        #[arg(long, short = 'd')]
        target_dir: Option<PathBuf>,
    },
    /// Generate a mutation plan for a component (alias for `add --plan`)
    Plan {
        /// Component name (e.g. dialog, select, tabs)
        component: String,
        /// Target project directory (defaults to current directory)
        #[arg(long, short = 'd')]
        target_dir: Option<PathBuf>,
    },
    /// Apply a previously generated mutation plan
    Apply {
        /// Path to the plan JSON file
        plan_file: PathBuf,
        /// Target project directory (defaults to current directory)
        #[arg(long, short = 'd')]
        target_dir: Option<PathBuf>,
    },
}

// ---------------------------------------------------------------------------
// Command implementations
// ---------------------------------------------------------------------------

/// Generate a plan for a component installation.
fn cmd_plan(component: &str, target_dir: &Path) -> Result<()> {
    let index = registry::generate_registry();
    let entry = index.get(component).with_context(|| {
        let available = index.names().join(", ");
        format!(
            "Component '{}' not found in registry. Available: {}",
            component, available
        )
    })?;

    let layout = DefaultLayout::new(target_dir);

    // Detect existing files for conflict checking
    let existing_files = scan_existing_files(target_dir, &entry.name);

    let plan = generate_plan(entry, &layout, &existing_files);
    let output = CliOutput::success(plan);
    println!("{}", output.to_json()?);
    Ok(())
}

/// Add a component to the target project.
fn cmd_add(component: &str, target_dir: &Path) -> Result<()> {
    let index = registry::generate_registry();
    let entry = index.get(component).with_context(|| {
        let available = index.names().join(", ");
        format!(
            "Component '{}' not found in registry. Available: {}",
            component, available
        )
    })?;

    let layout = DefaultLayout::new(target_dir);
    let existing_files = scan_existing_files(target_dir, &entry.name);
    let plan = generate_plan(entry, &layout, &existing_files);

    if plan.has_conflicts() {
        let conflict_msgs: Vec<String> = plan
            .conflicts
            .iter()
            .map(|c| format!("{}: {}", c.file_path.display(), c.reason))
            .collect();

        let errors: Vec<CliError> = plan
            .conflicts
            .iter()
            .map(|c| CliError {
                code: "CONFLICT".to_string(),
                message: format!("{}: {}", c.file_path.display(), c.reason),
            })
            .collect();

        eprintln!(
            "Conflicts detected for component '{}'. Use --plan to review.",
            component
        );
        for msg in &conflict_msgs {
            eprintln!("  - {}", msg);
        }

        let output = CliOutput::failure(plan, errors);
        println!("{}", output.to_json()?);
        return Ok(());
    }

    // Apply the plan
    match apply_plan(&plan, target_dir) {
        Ok(()) => {
            let output = CliOutput::success(plan);
            println!("{}", output.to_json()?);
            Ok(())
        }
        Err(boxed) => {
            let (failed_index, error, plan_clone) = *boxed;
            let report = ApplyFailureReport {
                plan: plan_clone.clone(),
                failed_at_index: failed_index,
                error: error.to_string(),
                completed_mutations: plan_clone.mutations[..failed_index].to_vec(),
                remaining_mutations: plan_clone.mutations[failed_index..].to_vec(),
            };

            let errors = vec![CliError {
                code: "APPLY_FAILED".to_string(),
                message: error.to_string(),
            }];

            let output = CliOutput::failure(report, errors);
            println!("{}", output.to_json()?);
            bail!("Apply failed at mutation {}: {}", failed_index, error)
        }
    }
}

/// Apply a plan from a JSON file.
fn cmd_apply(plan_file: &Path, target_dir: &Path) -> Result<()> {
    let json = std::fs::read_to_string(plan_file)
        .with_context(|| format!("Failed to read plan file: {}", plan_file.display()))?;

    // Parse the plan -- it may be wrapped in a CliOutput envelope or be a raw PlanContract
    let plan: PlanContract = if let Ok(envelope) = serde_json::from_str::<CliOutput<PlanContract>>(&json) {
        envelope.data
    } else {
        PlanContract::from_json(&json)
            .context("Failed to parse plan JSON. Expected PlanContract or CliOutput<PlanContract>")?
    };

    match apply_plan(&plan, target_dir) {
        Ok(()) => {
            let output = CliOutput::success(&plan);
            println!("{}", output.to_json()?);
            Ok(())
        }
        Err(boxed) => {
            let (failed_index, error, _) = *boxed;
            let report = ApplyFailureReport {
                plan: plan.clone(),
                failed_at_index: failed_index,
                error: error.to_string(),
                completed_mutations: plan.mutations[..failed_index].to_vec(),
                remaining_mutations: plan.mutations[failed_index..].to_vec(),
            };

            let errors = vec![CliError {
                code: "APPLY_FAILED".to_string(),
                message: error.to_string(),
            }];

            let output = CliOutput::failure(report, errors);
            println!("{}", output.to_json()?);
            bail!("Apply failed at mutation {}: {}", failed_index, error)
        }
    }
}

// ---------------------------------------------------------------------------
// Plan execution (apply)
// ---------------------------------------------------------------------------

/// Execute a plan's mutations against the filesystem.
///
/// Returns Ok(()) on success, or Err with the failed mutation index and error.
fn apply_plan(
    plan: &PlanContract,
    _target_dir: &std::path::Path,
) -> std::result::Result<(), Box<(usize, String, PlanContract)>> {
    for (i, mutation) in plan.mutations.iter().enumerate() {
        if let Err(e) = apply_mutation(mutation) {
            return Err(Box::new((i, e.to_string(), plan.clone())));
        }
    }

    // Write provenance metadata
    for pa in &plan.provenance_actions {
        let provenance_path = pa.file_path.with_extension("provenance.json");
        let provenance = serde_json::json!({
            "source": pa.source,
            "license": pa.license,
            "modifications": pa.modifications,
            "installed_by": "gpui-cli",
        });
        if let Ok(json) = serde_json::to_string_pretty(&provenance) {
            // Best-effort provenance write -- don't fail the install if this fails
            let _ = std::fs::write(&provenance_path, json);
        }
    }

    Ok(())
}

/// Apply a single file mutation.
fn apply_mutation(mutation: &FileMutation) -> Result<()> {
    match mutation.action {
        FileAction::Create => {
            // Ensure parent directory exists
            if let Some(parent) = mutation.file_path.parent() {
                std::fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create directory: {}", parent.display())
                })?;
            }
            std::fs::write(&mutation.file_path, &mutation.content).with_context(|| {
                format!("Failed to write file: {}", mutation.file_path.display())
            })?;
        }
        FileAction::Modify => match mutation.strategy {
            MutationStrategy::AppendExport => {
                let existing = if mutation.file_path.exists() {
                    std::fs::read_to_string(&mutation.file_path).with_context(|| {
                        format!("Failed to read file: {}", mutation.file_path.display())
                    })?
                } else {
                    // Create the file if it doesn't exist
                    if let Some(parent) = mutation.file_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    String::new()
                };

                // Check if export already exists (idempotent)
                if !existing.contains(&mutation.content) {
                    let new_content = if existing.is_empty() {
                        format!("{}\n", mutation.content)
                    } else if existing.ends_with('\n') {
                        format!("{}{}\n", existing, mutation.content)
                    } else {
                        format!("{}\n{}\n", existing, mutation.content)
                    };
                    std::fs::write(&mutation.file_path, new_content).with_context(|| {
                        format!("Failed to modify file: {}", mutation.file_path.display())
                    })?;
                }
            }
            MutationStrategy::InsertUse => {
                let existing = std::fs::read_to_string(&mutation.file_path).with_context(|| {
                    format!("Failed to read file: {}", mutation.file_path.display())
                })?;

                if !existing.contains(&mutation.content) {
                    let new_content = format!("{}\n{}", mutation.content, existing);
                    std::fs::write(&mutation.file_path, new_content)?;
                }
            }
            _ => {
                // WriteFile, ReplaceSection, DeleteFile handled elsewhere
                std::fs::write(&mutation.file_path, &mutation.content)?;
            }
        },
        FileAction::Delete => {
            if mutation.file_path.exists() {
                std::fs::remove_file(&mutation.file_path).with_context(|| {
                    format!("Failed to delete file: {}", mutation.file_path.display())
                })?;
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

/// Scan for existing files that would conflict with a component installation.
fn scan_existing_files(target_dir: &std::path::Path, component_name: &str) -> Vec<PathBuf> {
    let component_dir = target_dir
        .join("src/shared/ui")
        .join(component_name.to_lowercase());

    let mut existing = Vec::new();
    if component_dir.exists()
        && let Ok(entries) = std::fs::read_dir(&component_dir)
    {
        for entry in entries.flatten() {
            existing.push(entry.path());
        }
    }
    existing
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().context("Failed to get current directory")?;

    match cli.command {
        Commands::Add {
            component,
            plan,
            target_dir,
        } => {
            let dir = target_dir.unwrap_or_else(|| cwd.clone());
            if plan {
                cmd_plan(&component, &dir)
            } else {
                cmd_add(&component, &dir)
            }
        }
        Commands::Plan {
            component,
            target_dir,
        } => {
            let dir = target_dir.unwrap_or_else(|| cwd.clone());
            cmd_plan(&component, &dir)
        }
        Commands::Apply {
            plan_file,
            target_dir,
        } => {
            let dir = target_dir.unwrap_or_else(|| cwd.clone());
            cmd_apply(&plan_file, &dir)
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_dir() -> PathBuf {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "gpui-cli-test-{}-{}",
            std::process::id(),
            id
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn cleanup(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    // -- Plan generation tests --

    #[test]
    fn plan_dialog_produces_valid_json() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("dialog").unwrap();
        let layout = DefaultLayout::new(&dir);
        let plan = generate_plan(entry, &layout, &[]);

        let output = CliOutput::success(plan);
        let json = output.to_json().unwrap();

        assert!(json.contains("\"success\": true"));
        assert!(json.contains("\"Dialog\""));
        assert!(json.contains("\"mutations\""));

        // Verify parseable
        let parsed: CliOutput<PlanContract> = serde_json::from_str(&json).unwrap();
        assert!(parsed.success);
        assert_eq!(parsed.data.component_name, "Dialog");

        cleanup(&dir);
    }

    #[test]
    fn plan_select_produces_valid_json() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("select").unwrap();
        let layout = DefaultLayout::new(&dir);
        let plan = generate_plan(entry, &layout, &[]);

        let json = serde_json::to_string_pretty(&plan).unwrap();
        assert!(json.contains("\"Select\""));

        cleanup(&dir);
    }

    #[test]
    fn plan_tabs_produces_valid_json() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("tabs").unwrap();
        let layout = DefaultLayout::new(&dir);
        let plan = generate_plan(entry, &layout, &[]);

        let json = serde_json::to_string_pretty(&plan).unwrap();
        assert!(json.contains("\"Tabs\""));

        cleanup(&dir);
    }

    // -- Apply tests --

    #[test]
    fn apply_creates_component_files() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("dialog").unwrap();
        let layout = DefaultLayout::new(&dir);
        let plan = generate_plan(entry, &layout, &[]);

        let result = apply_plan(&plan, &dir);
        assert!(result.is_ok(), "Apply should succeed: {:?}", result.err());

        // Verify files were created
        let component_dir = dir.join("src/shared/ui/dialog");
        assert!(component_dir.exists(), "Component directory should exist");

        let mod_file = component_dir.join("mod.rs");
        assert!(mod_file.exists(), "mod.rs should exist");

        cleanup(&dir);
    }

    #[test]
    fn apply_creates_parent_module() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("dialog").unwrap();
        let layout = DefaultLayout::new(&dir);
        let plan = generate_plan(entry, &layout, &[]);

        apply_plan(&plan, &dir).unwrap();

        let parent_mod = dir.join("src/shared/ui/mod.rs");
        assert!(parent_mod.exists(), "Parent mod.rs should exist");

        let content = fs::read_to_string(&parent_mod).unwrap();
        assert!(
            content.contains("pub mod dialog"),
            "Parent mod.rs should export dialog"
        );

        cleanup(&dir);
    }

    #[test]
    fn apply_is_idempotent() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("dialog").unwrap();
        let layout = DefaultLayout::new(&dir);
        let plan = generate_plan(entry, &layout, &[]);

        // Apply twice
        apply_plan(&plan, &dir).unwrap();
        apply_plan(&plan, &dir).unwrap();

        // Verify the export line isn't duplicated
        let parent_mod = dir.join("src/shared/ui/mod.rs");
        let content = fs::read_to_string(&parent_mod).unwrap();
        let count = content.matches("pub mod dialog").count();
        assert_eq!(count, 1, "Export should appear exactly once (idempotent)");

        cleanup(&dir);
    }

    #[test]
    fn apply_writes_provenance() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("dialog").unwrap();
        let layout = DefaultLayout::new(&dir);
        let plan = generate_plan(entry, &layout, &[]);

        apply_plan(&plan, &dir).unwrap();

        // Check that at least one provenance file exists
        let component_dir = dir.join("src/shared/ui/dialog");
        let provenance_files: Vec<_> = fs::read_dir(&component_dir)
            .unwrap()
            .flatten()
            .filter(|e| {
                e.path()
                    .to_string_lossy()
                    .contains("provenance.json")
            })
            .collect();

        assert!(
            !provenance_files.is_empty(),
            "Provenance metadata should be written for installed files"
        );

        // Verify provenance content
        let prov_content = fs::read_to_string(provenance_files[0].path()).unwrap();
        let prov: serde_json::Value = serde_json::from_str(&prov_content).unwrap();
        assert!(prov.get("source").is_some());
        assert!(prov.get("license").is_some());
        assert!(prov.get("installed_by").is_some());

        cleanup(&dir);
    }

    // -- All 3 POC components apply --

    #[test]
    fn apply_all_poc_components() {
        let dir = temp_dir();
        let index = registry::generate_registry();

        for name in &["dialog", "select", "tabs"] {
            let entry = index.get(name).unwrap();
            let layout = DefaultLayout::new(&dir);
            let plan = generate_plan(entry, &layout, &[]);
            apply_plan(&plan, &dir).unwrap();
        }

        // Verify all component directories exist
        assert!(dir.join("src/shared/ui/dialog").exists());
        assert!(dir.join("src/shared/ui/select").exists());
        assert!(dir.join("src/shared/ui/tabs").exists());

        // Verify parent mod.rs exports all three
        let parent_mod = dir.join("src/shared/ui/mod.rs");
        let content = fs::read_to_string(&parent_mod).unwrap();
        assert!(content.contains("pub mod dialog"));
        assert!(content.contains("pub mod select"));
        assert!(content.contains("pub mod tabs"));

        cleanup(&dir);
    }

    // -- Plan JSON round-trip via file --

    #[test]
    fn plan_file_roundtrip() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("dialog").unwrap();
        let layout = DefaultLayout::new(&dir);
        let plan = generate_plan(entry, &layout, &[]);

        // Write plan to file
        let plan_file = dir.join("dialog-plan.json");
        let json = plan.to_json().unwrap();
        fs::write(&plan_file, &json).unwrap();

        // Read and parse
        let read_json = fs::read_to_string(&plan_file).unwrap();
        let restored = PlanContract::from_json(&read_json).unwrap();

        assert_eq!(restored.component_name, "Dialog");
        assert_eq!(restored.mutations.len(), plan.mutations.len());

        cleanup(&dir);
    }

    // -- Determinism test --

    #[test]
    fn plan_output_is_deterministic() {
        let dir = temp_dir();
        let index = registry::generate_registry();
        let entry = index.get("dialog").unwrap();
        let layout = DefaultLayout::new(&dir);

        let plan1 = generate_plan(entry, &layout, &[]);
        let plan2 = generate_plan(entry, &layout, &[]);

        let json1 = plan1.to_json().unwrap();
        let json2 = plan2.to_json().unwrap();

        assert_eq!(json1, json2, "Identical inputs must produce identical plans");

        cleanup(&dir);
    }

    // -- CliOutput envelope tests --

    #[test]
    fn cli_output_success_envelope() {
        let output = CliOutput::success("test data");
        let json = output.to_json().unwrap();

        assert!(json.contains("\"success\": true"));
        assert!(json.contains("\"test data\""));
        assert!(json.contains("\"errors\": []"));
    }

    #[test]
    fn cli_output_failure_envelope() {
        let errors = vec![CliError {
            code: "NOT_FOUND".to_string(),
            message: "Component not found".to_string(),
        }];
        let output = CliOutput::failure("error context", errors);
        let json = output.to_json().unwrap();

        assert!(json.contains("\"success\": false"));
        assert!(json.contains("\"NOT_FOUND\""));
        assert!(json.contains("\"Component not found\""));
    }

    // -- Error handling tests --

    #[test]
    fn nonexistent_component_returns_error() {
        let index = registry::generate_registry();
        assert!(index.get("nonexistent").is_none());
    }
}
