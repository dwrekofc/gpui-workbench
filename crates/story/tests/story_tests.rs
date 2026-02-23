//! Integration tests for the story framework.
//!
//! Tests are placed in an integration test file to avoid stack overflow from
//! GPUI IntoElement derive macro expansion in the unit test harness.
//! See AGENTS.md: "Test stack overflow" pattern.

use story::*;

/// Helper: create a registry with all 12 component stories registered.
fn full_registry() -> StoryRegistry {
    let mut registry = StoryRegistry::new();
    registry.register(ButtonStory);
    registry.register(CheckboxStory);
    registry.register(DialogStory);
    registry.register(DropdownMenuStory);
    registry.register(InputStory);
    registry.register(PopoverStory);
    registry.register(RadioStory);
    registry.register(SelectStory);
    registry.register(TabsStory);
    registry.register(TextareaStory);
    registry.register(ToastStory);
    registry.register(TooltipStory);
    registry
}

/// Helper: all stories as boxed trait objects.
fn all_stories() -> Vec<Box<dyn Story>> {
    vec![
        Box::new(ButtonStory),
        Box::new(CheckboxStory),
        Box::new(DialogStory),
        Box::new(DropdownMenuStory),
        Box::new(InputStory),
        Box::new(PopoverStory),
        Box::new(RadioStory),
        Box::new(SelectStory),
        Box::new(TabsStory),
        Box::new(TextareaStory),
        Box::new(ToastStory),
        Box::new(TooltipStory),
    ]
}

#[test]
fn registry_starts_empty() {
    let registry = StoryRegistry::new();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn registry_register_and_lookup() {
    let registry = full_registry();

    assert_eq!(registry.len(), 12);
    assert!(registry.get("Button").is_some());
    assert!(registry.get("Checkbox").is_some());
    assert!(registry.get("Dialog").is_some());
    assert!(registry.get("DropdownMenu").is_some());
    assert!(registry.get("Input").is_some());
    assert!(registry.get("Popover").is_some());
    assert!(registry.get("Radio").is_some());
    assert!(registry.get("Select").is_some());
    assert!(registry.get("Tabs").is_some());
    assert!(registry.get("Textarea").is_some());
    assert!(registry.get("Toast").is_some());
    assert!(registry.get("Tooltip").is_some());
    assert!(registry.get("Nonexistent").is_none());
}

#[test]
fn registry_names() {
    let registry = full_registry();
    let names: Vec<&str> = registry.names().collect();
    assert_eq!(
        names,
        vec![
            "Button",
            "Checkbox",
            "Dialog",
            "DropdownMenu",
            "Input",
            "Popover",
            "Radio",
            "Select",
            "Tabs",
            "Textarea",
            "Toast",
            "Tooltip",
        ]
    );
}

#[test]
fn story_entries_have_valid_contracts() {
    let registry = full_registry();

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
    for story in &all_stories() {
        assert!(!story.name().is_empty(), "Story name should not be empty");
    }
}

#[test]
fn all_stories_have_descriptions() {
    for story in &all_stories() {
        assert!(
            !story.description().is_empty(),
            "Story '{}' should have a description",
            story.name()
        );
    }
}

#[test]
fn state_matrix_from_button_contract() {
    let contract = components::Button::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(!matrix.states().is_empty());
    assert!(matrix.states().len() >= 5);
    assert_eq!(matrix.variants().len(), 4);
}

#[test]
fn state_matrix_from_dialog_contract() {
    let contract = components::Dialog::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(!matrix.states().is_empty());
    assert!(matrix.states().len() >= 4);
}

#[test]
fn state_matrix_from_select_contract() {
    let contract = components::Select::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(!matrix.states().is_empty());
    assert!(matrix.states().len() >= 6);
}

#[test]
fn state_matrix_from_tabs_contract() {
    let contract = components::Tabs::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(!matrix.states().is_empty());
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
    // Dialog doesn't define explicit variants
    let contract = components::Dialog::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert!(
        matrix.variants().is_empty(),
        "Dialog has no explicit variants"
    );

    // Button has 4 explicit variants
    let contract = components::Button::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert_eq!(matrix.variants().len(), 4, "Button has 4 variants");

    // Toast has 4 explicit variants
    let contract = components::Toast::contract();
    let matrix = StateMatrix::from_contract(&contract);
    assert_eq!(matrix.variants().len(), 4, "Toast has 4 variants");
}

#[test]
fn all_stories_registered_have_unique_names() {
    let registry = full_registry();
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
    for story in &all_stories() {
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
fn all_contracts_have_token_dependencies() {
    for story in &all_stories() {
        let contract = story.contract();
        assert!(
            !contract.token_dependencies.is_empty(),
            "Story '{}' contract should have token dependencies",
            story.name()
        );
    }
}

#[test]
fn all_contracts_have_interaction_checklists() {
    for story in &all_stories() {
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
