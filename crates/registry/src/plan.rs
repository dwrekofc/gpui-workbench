//! Plan contract for deterministic component installation.
//!
//! The `PlanContract` defines a JSON-serializable mutation plan that describes
//! exactly how files will be created, modified, or deleted when installing,
//! updating, or removing a component. Plans are deterministic (identical inputs
//! yield identical output) and human/agent-readable.
//!
//! The plan does NOT mutate files -- only `apply` does (FR-001, FR-002).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::RegistryEntry;

// ---------------------------------------------------------------------------
// Core plan types
// ---------------------------------------------------------------------------

/// The operation being planned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    /// Install a new component.
    Add,
    /// Update an existing component to a new version.
    Update,
    /// Remove an installed component.
    Remove,
}

/// The action to perform on a single file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileAction {
    /// Create a new file.
    Create,
    /// Modify an existing file.
    Modify,
    /// Delete an existing file.
    Delete,
}

/// The strategy for modifying a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationStrategy {
    /// Write the full file contents (for new files or full replacements).
    WriteFile,
    /// Append a `pub mod` or `pub use` export line to a module file.
    AppendExport,
    /// Insert a `use` import statement.
    InsertUse,
    /// Replace a specific section identified by markers.
    ReplaceSection,
    /// Remove the entire file.
    DeleteFile,
}

/// A single file mutation in the plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMutation {
    /// The action to perform.
    pub action: FileAction,
    /// Target file path relative to the project root.
    pub file_path: PathBuf,
    /// How to apply the mutation.
    pub strategy: MutationStrategy,
    /// The content to write (for Create/Modify) or the pattern to match (for ReplaceSection).
    /// Empty for Delete.
    pub content: String,
    /// Human-readable description of what this mutation does.
    pub description: String,
}

/// A detected conflict with an existing file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conflict {
    /// The file that conflicts.
    pub file_path: PathBuf,
    /// Description of the conflict.
    pub reason: String,
}

/// A provenance action for a file that needs attribution metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceAction {
    /// The file that needs provenance metadata.
    pub file_path: PathBuf,
    /// Source repository or upstream reference.
    pub source: String,
    /// License identifier.
    pub license: String,
    /// Description of local modifications.
    pub modifications: String,
}

/// The full plan contract describing a deterministic set of file mutations.
///
/// This is the JSON schema for `plan` and `apply` payloads. An agent or human
/// can read this plan and predict exactly which files will be created, modified,
/// or deleted (FR-016, AC-010).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanContract {
    /// The operation being planned.
    pub operation: Operation,
    /// Target component name.
    pub component_name: String,
    /// Target component version.
    pub component_version: String,
    /// Ordered list of file mutations to apply.
    pub mutations: Vec<FileMutation>,
    /// Detected conflicts (empty if none).
    pub conflicts: Vec<Conflict>,
    /// Files requiring provenance attribution.
    pub provenance_actions: Vec<ProvenanceAction>,
    /// File checksums for integrity verification (supports Phase 1 `doctor` command).
    pub file_checksums: BTreeMap<PathBuf, String>,
    /// The target layout used for this plan.
    pub target_layout: String,
}

impl PlanContract {
    /// Serialize the plan to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize a plan from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Whether this plan has any conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Total number of file mutations.
    pub fn mutation_count(&self) -> usize {
        self.mutations.len()
    }
}

// ---------------------------------------------------------------------------
// TemplateAdapter -- abstraction for target app layouts
// ---------------------------------------------------------------------------

/// Defines how components are laid out in a target application.
///
/// Different applications may organize their component files differently.
/// The `TemplateAdapter` trait abstracts this layout so plan generation
/// can target any supported structure.
pub trait TemplateAdapter {
    /// Human-readable name of this layout (e.g. "default", "flat").
    fn name(&self) -> &str;

    /// Returns the directory path where a component's source files should go.
    fn component_dir(&self, component_name: &str) -> PathBuf;

    /// Returns the path to the module file that exports components.
    fn module_file(&self) -> PathBuf;

    /// Returns the export line to add to the module file for a component.
    fn export_line(&self, component_name: &str) -> String;

    /// Returns the path to the theme tokens file (for token injection).
    fn theme_tokens_file(&self) -> PathBuf;
}

/// The default target layout: feature-first vertical slice.
///
/// Layout:
/// - Component source: `src/shared/ui/<component>/`
/// - Module exports: `src/shared/ui/mod.rs`
/// - Theme tokens: `src/shared/theme/tokens.rs`
#[derive(Debug, Clone)]
pub struct DefaultLayout {
    /// Root directory of the target project.
    pub project_root: PathBuf,
}

impl DefaultLayout {
    pub fn new(project_root: impl Into<PathBuf>) -> Self {
        Self {
            project_root: project_root.into(),
        }
    }
}

impl TemplateAdapter for DefaultLayout {
    fn name(&self) -> &str {
        "default"
    }

    fn component_dir(&self, component_name: &str) -> PathBuf {
        self.project_root
            .join("src/shared/ui")
            .join(component_name.to_lowercase())
    }

    fn module_file(&self) -> PathBuf {
        self.project_root.join("src/shared/ui/mod.rs")
    }

    fn export_line(&self, component_name: &str) -> String {
        let lower = component_name.to_lowercase();
        format!("pub mod {};", lower)
    }

    fn theme_tokens_file(&self) -> PathBuf {
        self.project_root.join("src/shared/theme/tokens.rs")
    }
}

// ---------------------------------------------------------------------------
// Plan generation
// ---------------------------------------------------------------------------

/// Generate an installation plan for a component.
///
/// The plan describes exactly which files will be created and which existing
/// files will be modified. Conflict detection checks whether target files
/// already exist.
pub fn generate_plan(
    entry: &RegistryEntry,
    layout: &dyn TemplateAdapter,
    existing_files: &[PathBuf],
) -> PlanContract {
    let component_dir = layout.component_dir(&entry.name);
    let mut mutations = Vec::new();
    let mut conflicts = Vec::new();
    let mut checksums = BTreeMap::new();

    // 1. Create component source file(s)
    for source_file in &entry.required_files {
        let source_filename = Path::new(source_file)
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("{}.rs", entry.name.to_lowercase()));

        let target_path = component_dir.join(&source_filename);

        // Conflict detection: check if target already exists
        if existing_files.contains(&target_path) {
            conflicts.push(Conflict {
                file_path: target_path.clone(),
                reason: format!(
                    "File already exists at target path; would overwrite existing {}",
                    source_filename
                ),
            });
        }

        let content = format!(
            "// Component: {} v{}\n// Source: {}\n// This file was installed by `gpui add {}`\n\npub use {}::*;\n",
            entry.name,
            entry.version,
            source_file,
            entry.name.to_lowercase(),
            entry.name.to_lowercase(),
        );

        let checksum = simple_checksum(&content);
        checksums.insert(target_path.clone(), checksum);

        mutations.push(FileMutation {
            action: FileAction::Create,
            file_path: target_path,
            strategy: MutationStrategy::WriteFile,
            content,
            description: format!("Install {} component source", entry.name),
        });
    }

    // 2. Create mod.rs in component directory
    let mod_path = component_dir.join("mod.rs");
    let mod_content = format!(
        "//! {} component module.\n\nmod {};\npub use {}::*;\n",
        entry.name,
        entry.name.to_lowercase(),
        entry.name.to_lowercase(),
    );
    let mod_checksum = simple_checksum(&mod_content);
    checksums.insert(mod_path.clone(), mod_checksum);

    if existing_files.contains(&mod_path) {
        conflicts.push(Conflict {
            file_path: mod_path.clone(),
            reason: "Component mod.rs already exists; would overwrite".to_string(),
        });
    }

    mutations.push(FileMutation {
        action: FileAction::Create,
        file_path: mod_path,
        strategy: MutationStrategy::WriteFile,
        content: mod_content,
        description: format!("Create {} module file", entry.name),
    });

    // 3. Update parent mod.rs with export
    let parent_mod = layout.module_file();
    let export_line = layout.export_line(&entry.name);

    mutations.push(FileMutation {
        action: FileAction::Modify,
        file_path: parent_mod,
        strategy: MutationStrategy::AppendExport,
        content: export_line,
        description: format!("Add {} export to shared UI module", entry.name),
    });

    // 4. Provenance actions for all required files
    let provenance_actions: Vec<ProvenanceAction> = entry
        .required_files
        .iter()
        .map(|f| {
            let target_filename = Path::new(f)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            ProvenanceAction {
                file_path: component_dir.join(&target_filename),
                source: f.clone(),
                license: "Apache-2.0 OR MIT".to_string(),
                modifications: format!("Installed via gpui add {}", entry.name.to_lowercase()),
            }
        })
        .collect();

    PlanContract {
        operation: Operation::Add,
        component_name: entry.name.clone(),
        component_version: entry.version.clone(),
        mutations,
        conflicts,
        provenance_actions,
        file_checksums: checksums,
        target_layout: layout.name().to_string(),
    }
}

/// Simple content checksum using a basic hash for integrity verification.
/// Uses a deterministic string hash (FNV-1a variant) for portability.
fn simple_checksum(content: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in content.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}

// ---------------------------------------------------------------------------
// Apply failure report
// ---------------------------------------------------------------------------

/// Post-failure state report after an apply operation fails (NFR-002).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyFailureReport {
    /// The plan that was being applied.
    pub plan: PlanContract,
    /// Index of the mutation that failed (0-based).
    pub failed_at_index: usize,
    /// Description of the failure.
    pub error: String,
    /// Mutations that were successfully applied before the failure.
    pub completed_mutations: Vec<FileMutation>,
    /// Mutations that were not applied.
    pub remaining_mutations: Vec<FileMutation>,
}

impl ApplyFailureReport {
    /// Serialize the failure report to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_registry;

    fn default_layout() -> DefaultLayout {
        DefaultLayout::new("/test/project")
    }

    // -- Plan generation tests --

    #[test]
    fn generate_plan_for_dialog() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        assert_eq!(plan.operation, Operation::Add);
        assert_eq!(plan.component_name, "Dialog");
        assert_eq!(plan.component_version, "0.1.0");
        assert_eq!(plan.target_layout, "default");
        assert!(!plan.mutations.is_empty());
        assert!(plan.conflicts.is_empty());
        assert!(!plan.provenance_actions.is_empty());
        assert!(!plan.file_checksums.is_empty());
    }

    #[test]
    fn generate_plan_for_select() {
        let registry = generate_registry();
        let entry = registry.get("Select").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        assert_eq!(plan.component_name, "Select");
        assert!(!plan.mutations.is_empty());
    }

    #[test]
    fn generate_plan_for_tabs() {
        let registry = generate_registry();
        let entry = registry.get("Tabs").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        assert_eq!(plan.component_name, "Tabs");
        assert!(!plan.mutations.is_empty());
    }

    #[test]
    fn plan_has_create_and_modify_mutations() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        let creates: Vec<_> = plan
            .mutations
            .iter()
            .filter(|m| m.action == FileAction::Create)
            .collect();
        let modifies: Vec<_> = plan
            .mutations
            .iter()
            .filter(|m| m.action == FileAction::Modify)
            .collect();

        assert!(!creates.is_empty(), "Plan should have Create mutations");
        assert!(!modifies.is_empty(), "Plan should have Modify mutations");
    }

    #[test]
    fn plan_targets_correct_directory() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        let create_paths: Vec<_> = plan
            .mutations
            .iter()
            .filter(|m| m.action == FileAction::Create)
            .map(|m| m.file_path.to_string_lossy().to_string())
            .collect();

        for path in &create_paths {
            assert!(
                path.contains("src/shared/ui/dialog"),
                "Create target should be in component directory, got: {}",
                path
            );
        }
    }

    #[test]
    fn plan_includes_module_export() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        let export_mutation = plan
            .mutations
            .iter()
            .find(|m| m.strategy == MutationStrategy::AppendExport);

        assert!(
            export_mutation.is_some(),
            "Plan should include a module export mutation"
        );

        let export = export_mutation.unwrap();
        assert!(export.content.contains("pub mod dialog"));
    }

    // -- Determinism tests (NFR-001) --

    #[test]
    fn plan_is_deterministic() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let layout = default_layout();

        let plan1 = generate_plan(entry, &layout, &[]);
        let plan2 = generate_plan(entry, &layout, &[]);

        let json1 = plan1.to_json().unwrap();
        let json2 = plan2.to_json().unwrap();

        assert_eq!(json1, json2, "Identical inputs must produce identical plans (NFR-001)");
    }

    #[test]
    fn all_poc_plans_are_deterministic() {
        let registry = generate_registry();
        let layout = default_layout();

        for name in &["Dialog", "Select", "Tabs"] {
            let entry = registry.get(name).unwrap();
            let plan1 = generate_plan(entry, &layout, &[]);
            let plan2 = generate_plan(entry, &layout, &[]);

            let json1 = plan1.to_json().unwrap();
            let json2 = plan2.to_json().unwrap();

            assert_eq!(json1, json2, "{} plan is not deterministic", name);
        }
    }

    // -- Conflict detection tests --

    #[test]
    fn conflict_detected_for_existing_file() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let layout = default_layout();

        let existing = vec![PathBuf::from(
            "/test/project/src/shared/ui/dialog/dialog.rs",
        )];

        let plan = generate_plan(entry, &layout, &existing);

        assert!(plan.has_conflicts(), "Should detect conflict with existing file");
        assert_eq!(plan.conflicts.len(), 1);
        assert!(plan.conflicts[0].reason.contains("already exists"));
    }

    #[test]
    fn no_conflicts_with_clean_directory() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        assert!(!plan.has_conflicts());
        assert!(plan.conflicts.is_empty());
    }

    // -- JSON serialization tests --

    #[test]
    fn plan_json_roundtrip() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        let json = plan.to_json().unwrap();
        let restored = PlanContract::from_json(&json).unwrap();

        assert_eq!(restored.operation, plan.operation);
        assert_eq!(restored.component_name, plan.component_name);
        assert_eq!(restored.component_version, plan.component_version);
        assert_eq!(restored.mutations.len(), plan.mutations.len());
        assert_eq!(restored.conflicts.len(), plan.conflicts.len());
        assert_eq!(restored.provenance_actions.len(), plan.provenance_actions.len());
        assert_eq!(restored.file_checksums.len(), plan.file_checksums.len());
        assert_eq!(restored.target_layout, plan.target_layout);
    }

    #[test]
    fn plan_json_contains_agent_readable_fields() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);
        let json = plan.to_json().unwrap();

        // An agent must be able to reconstruct the file tree from this JSON (FR-016)
        assert!(json.contains("\"operation\""));
        assert!(json.contains("\"component_name\""));
        assert!(json.contains("\"mutations\""));
        assert!(json.contains("\"file_path\""));
        assert!(json.contains("\"action\""));
        assert!(json.contains("\"strategy\""));
        assert!(json.contains("\"content\""));
        assert!(json.contains("\"file_checksums\""));
    }

    // -- File checksum tests --

    #[test]
    fn plan_includes_checksums_for_created_files() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        assert!(
            !plan.file_checksums.is_empty(),
            "Plan should include file checksums for doctor command"
        );

        // Every Create mutation should have a corresponding checksum
        for mutation in &plan.mutations {
            if mutation.action == FileAction::Create {
                assert!(
                    plan.file_checksums.contains_key(&mutation.file_path),
                    "Missing checksum for created file: {:?}",
                    mutation.file_path
                );
            }
        }
    }

    #[test]
    fn checksums_are_deterministic() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let layout = default_layout();

        let plan1 = generate_plan(entry, &layout, &[]);
        let plan2 = generate_plan(entry, &layout, &[]);

        assert_eq!(plan1.file_checksums, plan2.file_checksums);
    }

    // -- Provenance tests --

    #[test]
    fn plan_has_provenance_for_all_source_files() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        assert_eq!(
            plan.provenance_actions.len(),
            entry.required_files.len(),
            "Should have provenance for each required file"
        );

        for pa in &plan.provenance_actions {
            assert!(!pa.source.is_empty());
            assert!(!pa.license.is_empty());
            assert!(!pa.modifications.is_empty());
        }
    }

    // -- Performance test (NFR-003) --

    #[test]
    fn plan_generation_is_fast() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let layout = default_layout();

        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = generate_plan(entry, &layout, &[]);
        }
        let elapsed = start.elapsed();

        assert!(
            elapsed.as_millis() < 1000,
            "100 plan generations took {:?}, should be sub-second (NFR-003)",
            elapsed
        );
    }

    // -- ApplyFailureReport tests --

    #[test]
    fn failure_report_serializes() {
        let registry = generate_registry();
        let entry = registry.get("Dialog").unwrap();
        let plan = generate_plan(entry, &default_layout(), &[]);

        let report = ApplyFailureReport {
            plan: plan.clone(),
            failed_at_index: 1,
            error: "Permission denied".to_string(),
            completed_mutations: vec![plan.mutations[0].clone()],
            remaining_mutations: plan.mutations[1..].to_vec(),
        };

        let json = report.to_json().unwrap();
        assert!(json.contains("Permission denied"));
        assert!(json.contains("\"failed_at_index\": 1"));
    }

    // -- DefaultLayout tests --

    #[test]
    fn default_layout_paths() {
        let layout = DefaultLayout::new("/myapp");

        assert_eq!(layout.name(), "default");
        assert_eq!(
            layout.component_dir("Dialog"),
            PathBuf::from("/myapp/src/shared/ui/dialog")
        );
        assert_eq!(
            layout.module_file(),
            PathBuf::from("/myapp/src/shared/ui/mod.rs")
        );
        assert_eq!(layout.export_line("Dialog"), "pub mod dialog;");
        assert_eq!(
            layout.theme_tokens_file(),
            PathBuf::from("/myapp/src/shared/theme/tokens.rs")
        );
    }

    // -- Operation / FileAction / MutationStrategy serialization --

    #[test]
    fn operation_json_names() {
        assert_eq!(serde_json::to_string(&Operation::Add).unwrap(), "\"add\"");
        assert_eq!(serde_json::to_string(&Operation::Update).unwrap(), "\"update\"");
        assert_eq!(serde_json::to_string(&Operation::Remove).unwrap(), "\"remove\"");
    }

    #[test]
    fn file_action_json_names() {
        assert_eq!(serde_json::to_string(&FileAction::Create).unwrap(), "\"create\"");
        assert_eq!(serde_json::to_string(&FileAction::Modify).unwrap(), "\"modify\"");
        assert_eq!(serde_json::to_string(&FileAction::Delete).unwrap(), "\"delete\"");
    }

    #[test]
    fn mutation_strategy_json_names() {
        assert_eq!(
            serde_json::to_string(&MutationStrategy::WriteFile).unwrap(),
            "\"write_file\""
        );
        assert_eq!(
            serde_json::to_string(&MutationStrategy::AppendExport).unwrap(),
            "\"append_export\""
        );
        assert_eq!(
            serde_json::to_string(&MutationStrategy::InsertUse).unwrap(),
            "\"insert_use\""
        );
    }
}
