//! Component registry for the gpui-workbench design system.
//!
//! The registry indexes components from their `ComponentContract` source metadata,
//! providing lookup, listing, and JSON serialization for CLI consumption.
//! It is generated from source -- not hand-maintained manifests -- ensuring
//! the registry is always regenerable and never stale (FR-006).

pub mod plan;

use std::collections::HashMap;

use components::{
    ComponentContract, ComponentState, Disposition, PropDef, TokenRef,
};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// RegistryEntry -- the indexed summary of a single component
// ---------------------------------------------------------------------------

/// A registry entry summarizing a component's metadata for CLI and tooling.
///
/// This is a flattened, serializable view of a `ComponentContract` optimized
/// for lookup and listing. It contains the fields specified by the registry spec:
/// name, version, variants, states, props (with types), and required files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    /// Component name (e.g. "Dialog", "Select", "Tabs").
    pub name: String,
    /// Semver version string.
    pub version: String,
    /// Sourcing disposition (reuse, fork, rewrite).
    pub disposition: Disposition,
    /// Named visual variants.
    pub variants: Vec<String>,
    /// Interactive/visual states the component supports.
    pub states: Vec<ComponentState>,
    /// Prop definitions with names and types.
    pub props: Vec<PropDef>,
    /// Design-token dependencies.
    pub token_dependencies: Vec<TokenRef>,
    /// File paths required for installation.
    pub required_files: Vec<String>,
}

impl RegistryEntry {
    /// Create a registry entry from a `ComponentContract`.
    pub fn from_contract(contract: &ComponentContract) -> Self {
        Self {
            name: contract.name.clone(),
            version: contract.version.clone(),
            disposition: contract.disposition,
            variants: contract.variants.clone(),
            states: contract.states.clone(),
            props: contract.props.clone(),
            token_dependencies: contract.token_dependencies.clone(),
            required_files: contract.required_files.clone(),
        }
    }

    /// Produce a short summary string for listing output.
    pub fn summary(&self) -> String {
        let state_names: Vec<&str> = self.states.iter().map(|s| state_label(s)).collect();
        format!(
            "{} v{} ({:?}) -- {} props, {} states [{}], {} files",
            self.name,
            self.version,
            self.disposition,
            self.props.len(),
            self.states.len(),
            state_names.join(", "),
            self.required_files.len(),
        )
    }
}

/// Returns a static string label for a component state.
fn state_label(state: &ComponentState) -> &'static str {
    match state {
        ComponentState::Hover => "hover",
        ComponentState::Active => "active",
        ComponentState::Focused => "focused",
        ComponentState::Disabled => "disabled",
        ComponentState::Error => "error",
        ComponentState::Open => "open",
        ComponentState::Selected => "selected",
        ComponentState::Readonly => "readonly",
    }
}

// ---------------------------------------------------------------------------
// RegistryIndex -- the full component index
// ---------------------------------------------------------------------------

/// The component registry, indexing all installable components by name.
///
/// Generated from `ComponentContract` source metadata. Supports lookup by name,
/// listing all entries, and JSON serialization for CLI consumption.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegistryIndex {
    /// Components indexed by lowercase name for case-insensitive lookup.
    entries: HashMap<String, RegistryEntry>,
}

impl RegistryIndex {
    /// Create an empty registry index.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Register a component from its `ComponentContract`.
    ///
    /// The component is indexed by its lowercased name. If a component with
    /// the same name already exists, it is replaced (latest wins).
    pub fn register(&mut self, contract: &ComponentContract) {
        let entry = RegistryEntry::from_contract(contract);
        self.entries.insert(entry.name.to_lowercase(), entry);
    }

    /// Look up a component by name (case-insensitive).
    pub fn get(&self, name: &str) -> Option<&RegistryEntry> {
        self.entries.get(&name.to_lowercase())
    }

    /// List all registered entries, sorted by name.
    pub fn list(&self) -> Vec<&RegistryEntry> {
        let mut entries: Vec<&RegistryEntry> = self.entries.values().collect();
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }

    /// Return all registered component names, sorted.
    pub fn names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.entries.values().map(|e| e.name.as_str()).collect();
        names.sort();
        names
    }

    /// Number of registered components.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Remove a component by name (case-insensitive). Returns the removed entry.
    pub fn remove(&mut self, name: &str) -> Option<RegistryEntry> {
        self.entries.remove(&name.to_lowercase())
    }

    /// Serialize the registry index to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize a registry index from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// ---------------------------------------------------------------------------
// Registry generation -- populate from POC component contracts
// ---------------------------------------------------------------------------

/// Generate a fully populated registry index from all known component contracts.
///
/// This reads `ComponentContract` metadata directly from component source
/// (via the static `contract()` methods on each component type), ensuring
/// the registry is always in sync with the actual component implementations.
pub fn generate_registry() -> RegistryIndex {
    use components::{Dialog, Select, Tabs};

    let mut index = RegistryIndex::new();
    index.register(&Dialog::contract());
    index.register(&Select::contract());
    index.register(&Tabs::contract());
    index
}

/// Initialize the registry, validating all component contracts.
///
/// Returns errors if any contract fails validation. This ensures the registry
/// only indexes well-formed components.
pub fn generate_registry_validated() -> Result<RegistryIndex, Vec<(String, Vec<components::ValidationError>)>> {
    use components::{Dialog, Select, Tabs};

    let contracts = vec![
        Dialog::contract(),
        Select::contract(),
        Tabs::contract(),
    ];

    let mut validation_errors = Vec::new();
    for contract in &contracts {
        let errors = contract.validate();
        if !errors.is_empty() {
            validation_errors.push((contract.name.clone(), errors));
        }
    }

    if !validation_errors.is_empty() {
        return Err(validation_errors);
    }

    let mut index = RegistryIndex::new();
    for contract in &contracts {
        index.register(contract);
    }
    Ok(index)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use components::{Dialog, Select, Tabs};

    // -- RegistryEntry tests --

    #[test]
    fn entry_from_dialog_contract() {
        let contract = Dialog::contract();
        let entry = RegistryEntry::from_contract(&contract);

        assert_eq!(entry.name, "Dialog");
        assert_eq!(entry.version, "0.1.0");
        assert_eq!(entry.disposition, Disposition::Fork);
        assert!(!entry.props.is_empty());
        assert!(!entry.states.is_empty());
        assert!(!entry.token_dependencies.is_empty());
        assert!(!entry.required_files.is_empty());
    }

    #[test]
    fn entry_from_select_contract() {
        let contract = Select::contract();
        let entry = RegistryEntry::from_contract(&contract);

        assert_eq!(entry.name, "Select");
        assert_eq!(entry.disposition, Disposition::Fork);
        assert!(entry.states.contains(&ComponentState::Disabled));
        assert!(entry.states.contains(&ComponentState::Selected));
    }

    #[test]
    fn entry_from_tabs_contract() {
        let contract = Tabs::contract();
        let entry = RegistryEntry::from_contract(&contract);

        assert_eq!(entry.name, "Tabs");
        assert_eq!(entry.disposition, Disposition::Fork);
        assert!(entry.states.contains(&ComponentState::Disabled));
    }

    #[test]
    fn entry_summary_format() {
        let contract = Dialog::contract();
        let entry = RegistryEntry::from_contract(&contract);
        let summary = entry.summary();

        assert!(summary.contains("Dialog"));
        assert!(summary.contains("v0.1.0"));
        assert!(summary.contains("Fork"));
    }

    // -- RegistryIndex tests --

    #[test]
    fn empty_registry() {
        let index = RegistryIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
        assert!(index.list().is_empty());
        assert!(index.names().is_empty());
    }

    #[test]
    fn register_and_lookup() {
        let mut index = RegistryIndex::new();
        index.register(&Dialog::contract());

        assert_eq!(index.len(), 1);
        assert!(!index.is_empty());

        let entry = index.get("Dialog").unwrap();
        assert_eq!(entry.name, "Dialog");
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let mut index = RegistryIndex::new();
        index.register(&Dialog::contract());

        assert!(index.get("dialog").is_some());
        assert!(index.get("DIALOG").is_some());
        assert!(index.get("Dialog").is_some());
        assert!(index.get("dIaLoG").is_some());
    }

    #[test]
    fn lookup_nonexistent_returns_none() {
        let index = RegistryIndex::new();
        assert!(index.get("NonExistent").is_none());
    }

    #[test]
    fn register_overwrites_duplicate() {
        let mut index = RegistryIndex::new();
        index.register(&Dialog::contract());
        index.register(&Dialog::contract());

        assert_eq!(index.len(), 1);
    }

    #[test]
    fn list_returns_sorted() {
        let mut index = RegistryIndex::new();
        index.register(&Tabs::contract());
        index.register(&Dialog::contract());
        index.register(&Select::contract());

        let entries = index.list();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].name, "Dialog");
        assert_eq!(entries[1].name, "Select");
        assert_eq!(entries[2].name, "Tabs");
    }

    #[test]
    fn names_returns_sorted() {
        let mut index = RegistryIndex::new();
        index.register(&Tabs::contract());
        index.register(&Dialog::contract());
        index.register(&Select::contract());

        let names = index.names();
        assert_eq!(names, vec!["Dialog", "Select", "Tabs"]);
    }

    #[test]
    fn remove_entry() {
        let mut index = RegistryIndex::new();
        index.register(&Dialog::contract());

        let removed = index.remove("Dialog");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().name, "Dialog");
        assert!(index.is_empty());
    }

    #[test]
    fn remove_case_insensitive() {
        let mut index = RegistryIndex::new();
        index.register(&Dialog::contract());

        let removed = index.remove("dialog");
        assert!(removed.is_some());
    }

    #[test]
    fn remove_nonexistent_returns_none() {
        let mut index = RegistryIndex::new();
        assert!(index.remove("Ghost").is_none());
    }

    // -- Registry generation tests --

    #[test]
    fn generate_registry_indexes_all_poc_components() {
        let index = generate_registry();

        assert_eq!(index.len(), 3);
        assert!(index.get("Dialog").is_some());
        assert!(index.get("Select").is_some());
        assert!(index.get("Tabs").is_some());
    }

    #[test]
    fn generate_registry_validated_succeeds() {
        let result = generate_registry_validated();
        assert!(result.is_ok(), "Validation failed: {:?}", result.err());

        let index = result.unwrap();
        assert_eq!(index.len(), 3);
    }

    #[test]
    fn generated_entries_have_complete_metadata() {
        let index = generate_registry();

        for entry in index.list() {
            assert!(!entry.name.is_empty(), "{} has empty name", entry.name);
            assert!(!entry.version.is_empty(), "{} has empty version", entry.name);
            assert!(!entry.props.is_empty(), "{} has no props", entry.name);
            assert!(!entry.states.is_empty(), "{} has no states", entry.name);
            assert!(
                !entry.token_dependencies.is_empty(),
                "{} has no token dependencies",
                entry.name
            );
            assert!(
                !entry.required_files.is_empty(),
                "{} has no required files",
                entry.name
            );
        }
    }

    // -- JSON serialization tests --

    #[test]
    fn registry_json_roundtrip() {
        let index = generate_registry();
        let json = index.to_json().expect("serialize");
        let restored = RegistryIndex::from_json(&json).expect("deserialize");

        assert_eq!(restored.len(), index.len());
        for entry in index.list() {
            let restored_entry = restored.get(&entry.name).unwrap();
            assert_eq!(restored_entry.name, entry.name);
            assert_eq!(restored_entry.version, entry.version);
            assert_eq!(restored_entry.props.len(), entry.props.len());
            assert_eq!(restored_entry.states.len(), entry.states.len());
        }
    }

    #[test]
    fn single_entry_json_roundtrip() {
        let entry = RegistryEntry::from_contract(&Dialog::contract());
        let json = serde_json::to_string_pretty(&entry).expect("serialize");
        let restored: RegistryEntry = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(restored.name, entry.name);
        assert_eq!(restored.version, entry.version);
        assert_eq!(restored.disposition, entry.disposition);
        assert_eq!(restored.variants.len(), entry.variants.len());
        assert_eq!(restored.states.len(), entry.states.len());
        assert_eq!(restored.props.len(), entry.props.len());
        assert_eq!(restored.required_files, entry.required_files);
    }

    #[test]
    fn registry_json_contains_expected_fields() {
        let index = generate_registry();
        let json = index.to_json().expect("serialize");

        // Verify key field names appear in output
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"disposition\""));
        assert!(json.contains("\"variants\""));
        assert!(json.contains("\"states\""));
        assert!(json.contains("\"props\""));
        assert!(json.contains("\"required_files\""));
        assert!(json.contains("\"token_dependencies\""));

        // Verify all 3 POC components appear
        assert!(json.contains("\"Dialog\""));
        assert!(json.contains("\"Select\""));
        assert!(json.contains("\"Tabs\""));
    }

    // -- Performance test --

    #[test]
    fn registry_generation_under_2_seconds() {
        let start = std::time::Instant::now();
        let _index = generate_registry();
        let elapsed = start.elapsed();

        assert!(
            elapsed.as_secs() < 2,
            "Registry generation took {:?}, exceeds 2-second budget (NFR-010)",
            elapsed
        );
    }
}
