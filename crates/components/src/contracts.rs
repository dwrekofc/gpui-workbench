//! Component contracts schema for the gpui-workbench design system.
//!
//! A `ComponentContract` captures the full specification of a UI component:
//! its props, variants, states, token dependencies, interaction model,
//! acceptance criteria, and performance evidence. Contracts are constructed
//! via the builder pattern and can be serialized to JSON for tooling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// The full contract for a single UI component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentContract {
    /// Human-readable component name (e.g. "Button", "Dialog").
    pub name: String,
    /// Semver version string for this contract revision.
    pub version: String,
    /// Whether this component is reused, forked, or rewritten.
    pub disposition: Disposition,
    /// Prop definitions describing the component's public API surface.
    pub props: Vec<PropDef>,
    /// Named visual variants the component supports.
    pub variants: Vec<String>,
    /// Interactive / visual states the component can enter.
    pub states: Vec<ComponentState>,
    /// Design-token paths the component depends on.
    pub token_dependencies: Vec<TokenRef>,
    /// Narrative checklist describing interaction behaviors.
    pub interaction_checklist: InteractionChecklist,
    /// Boolean acceptance checklist for sign-off.
    pub acceptance_checklist: AcceptanceChecklist,
    /// Optional performance evidence collected in release mode.
    pub perf_evidence: Option<PerfEvidence>,
    /// File paths required for the component implementation.
    pub required_files: Vec<String>,
    /// Shared identifiers available on all component instances.
    pub shared_identifiers: SharedIdentifiers,
}

/// Shared identifiers that every component instance may carry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SharedIdentifiers {
    /// Unique identifier for the component instance.
    pub id: Option<String>,
    /// Tooltip text.
    pub tooltip: Option<String>,
    /// Arbitrary key-value metadata.
    pub metadata: HashMap<String, String>,
}

/// A single prop definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropDef {
    /// Prop name as it appears in code.
    pub name: String,
    /// Rust type name (e.g. `"bool"`, `"Option<SharedString>"`).
    pub type_name: String,
    /// Whether the prop must be provided by the caller.
    pub required: bool,
    /// String representation of the default value, if any.
    pub default_value: Option<String>,
    /// Human-readable description of the prop's purpose.
    pub description: String,
}

/// Interactive and visual states a component can enter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentState {
    Hover,
    Active,
    Focused,
    Disabled,
    Error,
    Open,
    Selected,
    Readonly,
}

impl ComponentState {
    /// Returns a slice of all possible component states.
    pub fn all() -> &'static [ComponentState] {
        &[
            ComponentState::Hover,
            ComponentState::Active,
            ComponentState::Focused,
            ComponentState::Disabled,
            ComponentState::Error,
            ComponentState::Open,
            ComponentState::Selected,
            ComponentState::Readonly,
        ]
    }
}

/// A reference to a design token used by a component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRef {
    /// Dot-separated token path (e.g. `"border.default"`).
    pub path: String,
    /// Human-readable description of how the token is used.
    pub usage: String,
}

/// Narrative descriptions of how the component handles interactions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractionChecklist {
    /// How the component participates in focus navigation.
    pub focus_behavior: Option<String>,
    /// Keyboard shortcuts and navigation model.
    pub keyboard_model: Option<String>,
    /// Click, hover, and pointer interaction behavior.
    pub pointer_behavior: Option<String>,
    /// Controlled vs. uncontrolled state management.
    pub state_model: Option<String>,
    /// Behavior when the component is disabled.
    pub disabled_behavior: Option<String>,
    /// Behavior when the component is read-only.
    pub readonly_behavior: Option<String>,
}

/// Boolean acceptance checklist for component sign-off.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcceptanceChecklist {
    // -- Contract checks --
    /// Focus behavior is documented.
    pub has_focus_behavior: bool,
    /// Keyboard model is documented.
    pub has_keyboard_model: bool,
    /// Pointer behavior is documented.
    pub has_pointer_behavior: bool,
    /// State model is documented.
    pub has_state_model: bool,
    /// Disabled semantics are documented.
    pub has_disabled_semantics: bool,

    // -- Design / token checks --
    /// All visual surfaces map to design tokens.
    pub surfaces_mapped_to_tokens: bool,
    /// No hard-coded color literals.
    pub no_hardcoded_colors: bool,

    // -- Performance gates --
    /// Release-mode performance evidence exists.
    pub has_release_mode_evidence: bool,
    /// No unapproved regressions from baseline.
    pub no_unapproved_regressions: bool,
    /// Bounded rendering verified (for virtualized / large lists).
    pub bounded_rendering_verified: bool,

    // -- Quality gates --
    /// Story coverage exists for all variants and states.
    pub has_story_coverage: bool,
    /// Interaction tests verify keyboard, focus, and pointer.
    pub has_interaction_tests: bool,
    /// Provenance metadata is present.
    pub has_provenance_metadata: bool,
}

/// Disposition rule describing how the component was sourced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Disposition {
    /// Re-used from an upstream library without modification.
    Reuse,
    /// Forked from upstream with local modifications.
    Fork,
    /// Written from scratch for this project.
    Rewrite,
}

/// Performance evidence collected in release mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfEvidence {
    /// Time to first render in milliseconds.
    pub render_time_ms: Option<f64>,
    /// Interaction-to-visual-update latency in milliseconds.
    pub interaction_latency_ms: Option<f64>,
    /// Free-form notes about the measurement.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// An error produced by contract validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Dot-path to the field with the issue.
    pub field: String,
    /// Human-readable description of the problem.
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl ComponentContract {
    /// Validate the contract, returning a list of errors.
    ///
    /// Rules:
    /// - `name` must not be empty.
    /// - `version` must not be empty.
    /// - At least one prop must be defined.
    /// - At least one state must be listed.
    /// - Required props must not have a default value (they are caller-supplied).
    /// - All required interaction-checklist fields for the declared states must
    ///   be filled in (e.g. if `Disabled` is listed, `disabled_behavior` must
    ///   be `Some`).
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(ValidationError {
                field: "name".into(),
                message: "Component name must not be empty".into(),
            });
        }

        if self.version.is_empty() {
            errors.push(ValidationError {
                field: "version".into(),
                message: "Version must not be empty".into(),
            });
        }

        if self.props.is_empty() {
            errors.push(ValidationError {
                field: "props".into(),
                message: "At least one prop must be defined".into(),
            });
        }

        if self.states.is_empty() {
            errors.push(ValidationError {
                field: "states".into(),
                message: "At least one state must be listed".into(),
            });
        }

        // Required props should not carry default values.
        for (i, prop) in self.props.iter().enumerate() {
            if prop.required && prop.default_value.is_some() {
                errors.push(ValidationError {
                    field: format!("props[{}].default_value", i),
                    message: format!(
                        "Required prop '{}' should not have a default value",
                        prop.name
                    ),
                });
            }
        }

        // State-dependent interaction checklist validation.
        let ic = &self.interaction_checklist;
        if self.states.contains(&ComponentState::Disabled) && ic.disabled_behavior.is_none() {
            errors.push(ValidationError {
                field: "interaction_checklist.disabled_behavior".into(),
                message: "Disabled state is listed but disabled_behavior is not described".into(),
            });
        }
        if self.states.contains(&ComponentState::Readonly) && ic.readonly_behavior.is_none() {
            errors.push(ValidationError {
                field: "interaction_checklist.readonly_behavior".into(),
                message: "Readonly state is listed but readonly_behavior is not described".into(),
            });
        }
        if self.states.contains(&ComponentState::Focused) && ic.focus_behavior.is_none() {
            errors.push(ValidationError {
                field: "interaction_checklist.focus_behavior".into(),
                message: "Focused state is listed but focus_behavior is not described".into(),
            });
        }
        if self.states.contains(&ComponentState::Hover) && ic.pointer_behavior.is_none() {
            errors.push(ValidationError {
                field: "interaction_checklist.pointer_behavior".into(),
                message: "Hover state is listed but pointer_behavior is not described".into(),
            });
        }

        errors
    }

    /// Start building a new `ComponentContract`.
    pub fn builder(name: impl Into<String>, version: impl Into<String>) -> ContractBuilder {
        ContractBuilder {
            name: name.into(),
            version: version.into(),
            disposition: Disposition::Rewrite,
            props: Vec::new(),
            variants: Vec::new(),
            states: Vec::new(),
            token_dependencies: Vec::new(),
            interaction_checklist: InteractionChecklist::default(),
            acceptance_checklist: AcceptanceChecklist::default(),
            perf_evidence: None,
            required_files: Vec::new(),
            shared_identifiers: SharedIdentifiers::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builder for constructing a `ComponentContract` incrementally.
#[derive(Debug, Clone)]
pub struct ContractBuilder {
    name: String,
    version: String,
    disposition: Disposition,
    props: Vec<PropDef>,
    variants: Vec<String>,
    states: Vec<ComponentState>,
    token_dependencies: Vec<TokenRef>,
    interaction_checklist: InteractionChecklist,
    acceptance_checklist: AcceptanceChecklist,
    perf_evidence: Option<PerfEvidence>,
    required_files: Vec<String>,
    shared_identifiers: SharedIdentifiers,
}

impl ContractBuilder {
    /// Set the disposition rule.
    pub fn disposition(mut self, disposition: Disposition) -> Self {
        self.disposition = disposition;
        self
    }

    /// Add a prop definition.
    pub fn prop(mut self, prop: PropDef) -> Self {
        self.props.push(prop);
        self
    }

    /// Add a required prop (convenience).
    pub fn required_prop(
        self,
        name: impl Into<String>,
        type_name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.prop(PropDef {
            name: name.into(),
            type_name: type_name.into(),
            required: true,
            default_value: None,
            description: description.into(),
        })
    }

    /// Add an optional prop with a default value (convenience).
    pub fn optional_prop(
        self,
        name: impl Into<String>,
        type_name: impl Into<String>,
        default_value: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.prop(PropDef {
            name: name.into(),
            type_name: type_name.into(),
            required: false,
            default_value: Some(default_value.into()),
            description: description.into(),
        })
    }

    /// Add a named variant.
    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variants.push(variant.into());
        self
    }

    /// Add a component state.
    pub fn state(mut self, state: ComponentState) -> Self {
        if !self.states.contains(&state) {
            self.states.push(state);
        }
        self
    }

    /// Add multiple states at once.
    pub fn states(mut self, states: &[ComponentState]) -> Self {
        for &s in states {
            if !self.states.contains(&s) {
                self.states.push(s);
            }
        }
        self
    }

    /// Add a token dependency.
    pub fn token_dep(mut self, path: impl Into<String>, usage: impl Into<String>) -> Self {
        self.token_dependencies.push(TokenRef {
            path: path.into(),
            usage: usage.into(),
        });
        self
    }

    /// Set the interaction checklist.
    pub fn interaction_checklist(mut self, checklist: InteractionChecklist) -> Self {
        self.interaction_checklist = checklist;
        self
    }

    /// Set focus behavior description.
    pub fn focus_behavior(mut self, desc: impl Into<String>) -> Self {
        self.interaction_checklist.focus_behavior = Some(desc.into());
        self
    }

    /// Set keyboard model description.
    pub fn keyboard_model(mut self, desc: impl Into<String>) -> Self {
        self.interaction_checklist.keyboard_model = Some(desc.into());
        self
    }

    /// Set pointer behavior description.
    pub fn pointer_behavior(mut self, desc: impl Into<String>) -> Self {
        self.interaction_checklist.pointer_behavior = Some(desc.into());
        self
    }

    /// Set state model description.
    pub fn state_model(mut self, desc: impl Into<String>) -> Self {
        self.interaction_checklist.state_model = Some(desc.into());
        self
    }

    /// Set disabled behavior description.
    pub fn disabled_behavior(mut self, desc: impl Into<String>) -> Self {
        self.interaction_checklist.disabled_behavior = Some(desc.into());
        self
    }

    /// Set readonly behavior description.
    pub fn readonly_behavior(mut self, desc: impl Into<String>) -> Self {
        self.interaction_checklist.readonly_behavior = Some(desc.into());
        self
    }

    /// Set the acceptance checklist.
    pub fn acceptance_checklist(mut self, checklist: AcceptanceChecklist) -> Self {
        self.acceptance_checklist = checklist;
        self
    }

    /// Set performance evidence.
    pub fn perf_evidence(mut self, evidence: PerfEvidence) -> Self {
        self.perf_evidence = Some(evidence);
        self
    }

    /// Add a required file path.
    pub fn required_file(mut self, file: impl Into<String>) -> Self {
        self.required_files.push(file.into());
        self
    }

    /// Set the component instance id.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.shared_identifiers.id = Some(id.into());
        self
    }

    /// Set the tooltip.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.shared_identifiers.tooltip = Some(tooltip.into());
        self
    }

    /// Insert a metadata key-value pair.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.shared_identifiers
            .metadata
            .insert(key.into(), value.into());
        self
    }

    /// Consume the builder and produce the finished contract.
    pub fn build(self) -> ComponentContract {
        ComponentContract {
            name: self.name,
            version: self.version,
            disposition: self.disposition,
            props: self.props,
            variants: self.variants,
            states: self.states,
            token_dependencies: self.token_dependencies,
            interaction_checklist: self.interaction_checklist,
            acceptance_checklist: self.acceptance_checklist,
            perf_evidence: self.perf_evidence,
            required_files: self.required_files,
            shared_identifiers: self.shared_identifiers,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_contract() -> ComponentContract {
        ComponentContract::builder("Button", "0.1.0")
            .disposition(Disposition::Rewrite)
            .required_prop("label", "SharedString", "Button label text")
            .optional_prop(
                "disabled",
                "bool",
                "false",
                "Whether the button is disabled",
            )
            .variant("primary")
            .variant("secondary")
            .variant("ghost")
            .state(ComponentState::Hover)
            .state(ComponentState::Active)
            .state(ComponentState::Focused)
            .state(ComponentState::Disabled)
            .token_dep("colors.accent", "button background")
            .token_dep("border.default", "button border color")
            .focus_behavior("Receives focus via Tab key; shows focus ring")
            .keyboard_model("Enter/Space activates the button")
            .pointer_behavior("Click activates; hover shows highlight")
            .state_model("Uncontrolled; fires on_click callback")
            .disabled_behavior("Ignores pointer and keyboard events; reduced opacity")
            .required_file("crates/components/src/button.rs")
            .id("btn-primary")
            .tooltip("Click me")
            .metadata("provenance", "custom")
            .build()
    }

    #[test]
    fn test_builder_construction() {
        let contract = sample_contract();

        assert_eq!(contract.name, "Button");
        assert_eq!(contract.version, "0.1.0");
        assert_eq!(contract.disposition, Disposition::Rewrite);
        assert_eq!(contract.props.len(), 2);
        assert_eq!(contract.variants, vec!["primary", "secondary", "ghost"]);
        assert_eq!(contract.states.len(), 4);
        assert_eq!(contract.token_dependencies.len(), 2);
        assert_eq!(contract.required_files.len(), 1);
        assert_eq!(
            contract.shared_identifiers.id.as_deref(),
            Some("btn-primary")
        );
        assert_eq!(
            contract.shared_identifiers.tooltip.as_deref(),
            Some("Click me")
        );
        assert_eq!(
            contract.shared_identifiers.metadata.get("provenance"),
            Some(&"custom".to_string())
        );
    }

    #[test]
    fn test_prop_details() {
        let contract = sample_contract();
        let label = &contract.props[0];
        assert_eq!(label.name, "label");
        assert!(label.required);
        assert!(label.default_value.is_none());

        let disabled = &contract.props[1];
        assert_eq!(disabled.name, "disabled");
        assert!(!disabled.required);
        assert_eq!(disabled.default_value.as_deref(), Some("false"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let contract = sample_contract();
        let json = serde_json::to_string_pretty(&contract).expect("serialize");
        let deserialized: ComponentContract = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deserialized.name, contract.name);
        assert_eq!(deserialized.version, contract.version);
        assert_eq!(deserialized.props.len(), contract.props.len());
        assert_eq!(deserialized.states.len(), contract.states.len());
        assert_eq!(deserialized.variants, contract.variants);
    }

    #[test]
    fn test_validation_passes_for_valid_contract() {
        let contract = sample_contract();
        let errors = contract.validate();
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validation_empty_name() {
        let contract = ComponentContract::builder("", "0.1.0")
            .required_prop("x", "u32", "a prop")
            .state(ComponentState::Active)
            .build();
        let errors = contract.validate();
        assert!(errors.iter().any(|e| e.field == "name"));
    }

    #[test]
    fn test_validation_empty_version() {
        let contract = ComponentContract::builder("Foo", "")
            .required_prop("x", "u32", "a prop")
            .state(ComponentState::Active)
            .build();
        let errors = contract.validate();
        assert!(errors.iter().any(|e| e.field == "version"));
    }

    #[test]
    fn test_validation_no_props() {
        let contract = ComponentContract::builder("Foo", "0.1.0")
            .state(ComponentState::Active)
            .build();
        let errors = contract.validate();
        assert!(errors.iter().any(|e| e.field == "props"));
    }

    #[test]
    fn test_validation_no_states() {
        let contract = ComponentContract::builder("Foo", "0.1.0")
            .required_prop("x", "u32", "a prop")
            .build();
        let errors = contract.validate();
        assert!(errors.iter().any(|e| e.field == "states"));
    }

    #[test]
    fn test_validation_required_prop_with_default() {
        let contract = ComponentContract::builder("Foo", "0.1.0")
            .prop(PropDef {
                name: "bar".into(),
                type_name: "u32".into(),
                required: true,
                default_value: Some("42".into()),
                description: "bad prop".into(),
            })
            .state(ComponentState::Active)
            .build();
        let errors = contract.validate();
        assert!(errors.iter().any(|e| e.field == "props[0].default_value"));
    }

    #[test]
    fn test_validation_disabled_without_behavior() {
        let contract = ComponentContract::builder("Foo", "0.1.0")
            .required_prop("x", "u32", "a prop")
            .state(ComponentState::Disabled)
            .build();
        let errors = contract.validate();
        assert!(
            errors
                .iter()
                .any(|e| e.field == "interaction_checklist.disabled_behavior")
        );
    }

    #[test]
    fn test_validation_readonly_without_behavior() {
        let contract = ComponentContract::builder("Foo", "0.1.0")
            .required_prop("x", "u32", "a prop")
            .state(ComponentState::Readonly)
            .build();
        let errors = contract.validate();
        assert!(
            errors
                .iter()
                .any(|e| e.field == "interaction_checklist.readonly_behavior")
        );
    }

    #[test]
    fn test_validation_focused_without_behavior() {
        let contract = ComponentContract::builder("Foo", "0.1.0")
            .required_prop("x", "u32", "a prop")
            .state(ComponentState::Focused)
            .build();
        let errors = contract.validate();
        assert!(
            errors
                .iter()
                .any(|e| e.field == "interaction_checklist.focus_behavior")
        );
    }

    #[test]
    fn test_validation_hover_without_pointer() {
        let contract = ComponentContract::builder("Foo", "0.1.0")
            .required_prop("x", "u32", "a prop")
            .state(ComponentState::Hover)
            .build();
        let errors = contract.validate();
        assert!(
            errors
                .iter()
                .any(|e| e.field == "interaction_checklist.pointer_behavior")
        );
    }

    #[test]
    fn test_all_states_represented() {
        let all = ComponentState::all();
        assert_eq!(all.len(), 8);
        assert!(all.contains(&ComponentState::Hover));
        assert!(all.contains(&ComponentState::Active));
        assert!(all.contains(&ComponentState::Focused));
        assert!(all.contains(&ComponentState::Disabled));
        assert!(all.contains(&ComponentState::Error));
        assert!(all.contains(&ComponentState::Open));
        assert!(all.contains(&ComponentState::Selected));
        assert!(all.contains(&ComponentState::Readonly));
    }

    #[test]
    fn test_state_deduplication() {
        let contract = ComponentContract::builder("Foo", "0.1.0")
            .state(ComponentState::Hover)
            .state(ComponentState::Hover)
            .state(ComponentState::Hover)
            .required_prop("x", "u32", "a prop")
            .pointer_behavior("click")
            .build();
        assert_eq!(contract.states.len(), 1);
    }

    #[test]
    fn test_disposition_variants() {
        for disp in [Disposition::Reuse, Disposition::Fork, Disposition::Rewrite] {
            let contract = ComponentContract::builder("X", "1.0.0")
                .disposition(disp)
                .required_prop("x", "u32", "a prop")
                .state(ComponentState::Active)
                .build();
            assert_eq!(contract.disposition, disp);
        }
    }

    #[test]
    fn test_perf_evidence() {
        let contract = ComponentContract::builder("X", "1.0.0")
            .required_prop("x", "u32", "a prop")
            .state(ComponentState::Active)
            .perf_evidence(PerfEvidence {
                render_time_ms: Some(2.5),
                interaction_latency_ms: Some(16.0),
                notes: "Measured on M1 MacBook Pro".into(),
            })
            .build();

        let evidence = contract.perf_evidence.as_ref().unwrap();
        assert_eq!(evidence.render_time_ms, Some(2.5));
        assert_eq!(evidence.interaction_latency_ms, Some(16.0));
        assert_eq!(evidence.notes, "Measured on M1 MacBook Pro");
    }

    #[test]
    fn test_acceptance_checklist_default_all_false() {
        let checklist = AcceptanceChecklist::default();
        assert!(!checklist.has_focus_behavior);
        assert!(!checklist.has_keyboard_model);
        assert!(!checklist.has_pointer_behavior);
        assert!(!checklist.has_state_model);
        assert!(!checklist.has_disabled_semantics);
        assert!(!checklist.surfaces_mapped_to_tokens);
        assert!(!checklist.no_hardcoded_colors);
        assert!(!checklist.has_release_mode_evidence);
        assert!(!checklist.no_unapproved_regressions);
        assert!(!checklist.bounded_rendering_verified);
        assert!(!checklist.has_story_coverage);
        assert!(!checklist.has_interaction_tests);
        assert!(!checklist.has_provenance_metadata);
    }

    #[test]
    fn test_json_state_names() {
        let json = serde_json::to_string(&ComponentState::Hover).unwrap();
        assert_eq!(json, "\"hover\"");
        let json = serde_json::to_string(&ComponentState::Readonly).unwrap();
        assert_eq!(json, "\"readonly\"");
    }

    #[test]
    fn test_json_disposition_names() {
        let json = serde_json::to_string(&Disposition::Reuse).unwrap();
        assert_eq!(json, "\"reuse\"");
        let json = serde_json::to_string(&Disposition::Fork).unwrap();
        assert_eq!(json, "\"fork\"");
        let json = serde_json::to_string(&Disposition::Rewrite).unwrap();
        assert_eq!(json, "\"rewrite\"");
    }
}
