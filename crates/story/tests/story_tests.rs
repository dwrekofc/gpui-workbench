//! Integration tests for the story framework.
//!
//! Tests are placed in an integration test file to avoid stack overflow from
//! GPUI IntoElement derive macro expansion in the unit test harness.
//! See AGENTS.md: "Test stack overflow" pattern.

use story::*;

#[test]
fn registry_starts_empty() {
    let registry = StoryRegistry::new();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn registry_register_and_lookup() {
    let mut registry = StoryRegistry::new();
    registry.register(DialogStory);
    registry.register(SelectStory);
    registry.register(TabsStory);

    assert_eq!(registry.len(), 3);
    assert!(registry.get("Dialog").is_some());
    assert!(registry.get("Select").is_some());
    assert!(registry.get("Tabs").is_some());
    assert!(registry.get("Nonexistent").is_none());
}

#[test]
fn registry_names() {
    let mut registry = StoryRegistry::new();
    registry.register(DialogStory);
    registry.register(SelectStory);
    registry.register(TabsStory);

    let names: Vec<&str> = registry.names().collect();
    assert_eq!(names, vec!["Dialog", "Select", "Tabs"]);
}

#[test]
fn story_entries_have_valid_contracts() {
    let mut registry = StoryRegistry::new();
    registry.register(DialogStory);
    registry.register(SelectStory);
    registry.register(TabsStory);

    for entry in registry.entries() {
        let contract = entry.contract();
        let errors = contract.validate();
        assert!(
            errors.is_empty(),
            "Story '{}' contract has validation errors: {:?}",
            entry.name(),
            errors
        );
    }
}

#[test]
fn story_names_not_empty() {
    let stories: Vec<Box<dyn Story>> = vec![
        Box::new(DialogStory),
        Box::new(SelectStory),
        Box::new(TabsStory),
    ];

    for story in &stories {
        assert!(!story.name().is_empty(), "Story name should not be empty");
    }
}

#[test]
fn all_poc_stories_have_descriptions() {
    // All three POC stories should have non-empty descriptions
    // because they are the primary validation targets.
    assert!(!DialogStory.description().is_empty());
    assert!(!SelectStory.description().is_empty());
    assert!(!TabsStory.description().is_empty());
}

#[test]
fn state_matrix_from_dialog_contract() {
    let contract = components::Dialog::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(!matrix.states().is_empty());
    // Dialog has Open, Focused, Hover, Active states
    assert!(matrix.states().len() >= 4);
}

#[test]
fn state_matrix_from_select_contract() {
    let contract = components::Select::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(!matrix.states().is_empty());
    // Select has Open, Focused, Hover, Active, Selected, Disabled states
    assert!(matrix.states().len() >= 6);
}

#[test]
fn state_matrix_from_tabs_contract() {
    let contract = components::Tabs::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(!matrix.states().is_empty());
    // Tabs has Focused, Hover, Active, Selected, Disabled states
    assert!(matrix.states().len() >= 5);
}

#[test]
fn state_matrix_preserves_token_deps() {
    let contract = components::Dialog::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(
        !matrix.token_paths().is_empty(),
        "Matrix should preserve token dependency paths"
    );
    assert!(
        matrix
            .token_paths()
            .iter()
            .any(|p| p == "surface.elevated_surface"),
        "Dialog matrix should include surface.elevated_surface token"
    );
}

#[test]
fn state_matrix_preserves_variants() {
    // POC components don't define explicit variants, so variants should be empty.
    let contract = components::Dialog::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(
        matrix.variants().is_empty(),
        "Dialog has no explicit variants"
    );
}

#[test]
fn all_stories_registered_have_unique_names() {
    let mut registry = StoryRegistry::new();
    registry.register(DialogStory);
    registry.register(SelectStory);
    registry.register(TabsStory);

    let names: Vec<&str> = registry.names().collect();
    let mut deduped = names.clone();
    deduped.sort();
    deduped.dedup();
    assert_eq!(
        names.len(),
        deduped.len(),
        "All story names should be unique"
    );
}

#[test]
fn story_contract_names_match_story_names() {
    // Verify that the story name matches its component contract name.
    let stories: Vec<Box<dyn Story>> = vec![
        Box::new(DialogStory),
        Box::new(SelectStory),
        Box::new(TabsStory),
    ];

    for story in &stories {
        let contract = story.contract();
        assert_eq!(
            story.name(),
            contract.name,
            "Story name '{}' should match contract name '{}'",
            story.name(),
            contract.name
        );
    }
}

#[test]
fn all_poc_contracts_have_token_dependencies() {
    // Every POC component story should have at least one token dependency,
    // verifying they use the token system (no hardcoded colors).
    let stories: Vec<Box<dyn Story>> = vec![
        Box::new(DialogStory),
        Box::new(SelectStory),
        Box::new(TabsStory),
    ];

    for story in &stories {
        let contract = story.contract();
        assert!(
            !contract.token_dependencies.is_empty(),
            "Story '{}' contract should have token dependencies",
            story.name()
        );
    }
}

#[test]
fn all_poc_contracts_have_interaction_checklists() {
    // Every POC component should have at least focus and keyboard model documented.
    let stories: Vec<Box<dyn Story>> = vec![
        Box::new(DialogStory),
        Box::new(SelectStory),
        Box::new(TabsStory),
    ];

    for story in &stories {
        let contract = story.contract();
        let ic = &contract.interaction_checklist;
        assert!(
            ic.focus_behavior.is_some(),
            "Story '{}' should document focus behavior",
            story.name()
        );
        assert!(
            ic.keyboard_model.is_some(),
            "Story '{}' should document keyboard model",
            story.name()
        );
    }
}
