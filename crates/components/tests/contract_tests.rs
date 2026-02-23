//! Contract validation tests for all POC components.
//!
//! These tests verify that Dialog, Select, and Tabs component contracts
//! are well-formed, validate correctly, and serialize to JSON.
//! They are in an integration test to avoid the stack overflow that occurs
//! when compiling tests in the same crate as GPUI IntoElement derives.

use components::dialog::Dialog;
use components::select::{Select, SelectItem};
use components::tabs::{TabItem, Tabs};
use components::{ComponentContract, ComponentState, Disposition};
use primitives::{NavDirection, navigate_index};

// ---- Dialog Contract Tests ----

#[test]
fn dialog_contract_validates() {
    let contract = Dialog::contract();
    let errors = contract.validate();
    assert!(
        errors.is_empty(),
        "Dialog contract validation failed: {:?}",
        errors
    );
}

#[test]
fn dialog_contract_has_correct_disposition() {
    let contract = Dialog::contract();
    assert_eq!(contract.disposition, Disposition::Fork);
}

#[test]
fn dialog_contract_has_token_deps() {
    let contract = Dialog::contract();
    assert!(
        !contract.token_dependencies.is_empty(),
        "Dialog should have token dependencies"
    );
    let paths: Vec<&str> = contract
        .token_dependencies
        .iter()
        .map(|t| t.path.as_str())
        .collect();
    assert!(paths.contains(&"surface.elevated_surface"));
    assert!(paths.contains(&"border.default"));
    assert!(paths.contains(&"text.default"));
}

#[test]
fn dialog_contract_has_required_states() {
    let contract = Dialog::contract();
    assert!(contract.states.contains(&ComponentState::Open));
    assert!(contract.states.contains(&ComponentState::Focused));
}

#[test]
fn dialog_contract_serializes() {
    let contract = Dialog::contract();
    let json = serde_json::to_string_pretty(&contract).expect("serialize");
    assert!(json.contains("Dialog"));
    // Round-trip
    let deserialized: ComponentContract = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(deserialized.name, "Dialog");
}

// ---- Select Contract Tests ----

#[test]
fn select_item_creation() {
    let item = SelectItem::new("Option A");
    assert_eq!(item.label.as_ref(), "Option A");
    assert!(!item.disabled);
}

#[test]
fn select_item_disabled() {
    let item = SelectItem::disabled("Disabled Option");
    assert_eq!(item.label.as_ref(), "Disabled Option");
    assert!(item.disabled);
}

#[test]
fn select_contract_validates() {
    let contract = Select::contract();
    let errors = contract.validate();
    assert!(
        errors.is_empty(),
        "Select contract validation failed: {:?}",
        errors
    );
}

#[test]
fn select_contract_has_correct_disposition() {
    let contract = Select::contract();
    assert_eq!(contract.disposition, Disposition::Fork);
}

#[test]
fn select_contract_has_required_states() {
    let contract = Select::contract();
    assert!(contract.states.contains(&ComponentState::Open));
    assert!(contract.states.contains(&ComponentState::Selected));
    assert!(contract.states.contains(&ComponentState::Disabled));
}

#[test]
fn select_contract_has_token_deps() {
    let contract = Select::contract();
    let paths: Vec<&str> = contract
        .token_dependencies
        .iter()
        .map(|t| t.path.as_str())
        .collect();
    assert!(paths.contains(&"element.background"));
    assert!(paths.contains(&"surface.elevated_surface"));
    assert!(paths.contains(&"ghost_element.hover"));
}

#[test]
fn select_contract_serializes() {
    let contract = Select::contract();
    let json = serde_json::to_string_pretty(&contract).expect("serialize");
    assert!(json.contains("Select"));
    let deserialized: ComponentContract = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(deserialized.name, "Select");
}

#[test]
fn select_navigate_skips_disabled() {
    let items = vec![
        SelectItem::new("A"),
        SelectItem::disabled("B"),
        SelectItem::new("C"),
    ];
    let next = navigate_index(0, NavDirection::Next, items.len(), |i| items[i].disabled);
    assert_eq!(next, 2);
}

#[test]
fn select_navigate_wraps() {
    let items = vec![
        SelectItem::new("A"),
        SelectItem::new("B"),
        SelectItem::new("C"),
    ];
    let next = navigate_index(2, NavDirection::Next, items.len(), |i| items[i].disabled);
    assert_eq!(next, 0);
}

// ---- Tabs Contract Tests ----

#[test]
fn tab_item_creation() {
    let item = TabItem::new("Tab 1");
    assert_eq!(item.label.as_ref(), "Tab 1");
    assert!(!item.disabled);
}

#[test]
fn tab_item_disabled() {
    let item = TabItem::new("Disabled Tab").set_disabled(true);
    assert!(item.disabled);
}

#[test]
fn tabs_contract_validates() {
    let contract = Tabs::contract();
    let errors = contract.validate();
    assert!(
        errors.is_empty(),
        "Tabs contract validation failed: {:?}",
        errors
    );
}

#[test]
fn tabs_contract_has_correct_disposition() {
    let contract = Tabs::contract();
    assert_eq!(contract.disposition, Disposition::Fork);
}

#[test]
fn tabs_contract_has_required_states() {
    let contract = Tabs::contract();
    assert!(contract.states.contains(&ComponentState::Focused));
    assert!(contract.states.contains(&ComponentState::Selected));
    assert!(contract.states.contains(&ComponentState::Disabled));
}

#[test]
fn tabs_contract_has_token_deps() {
    let contract = Tabs::contract();
    let paths: Vec<&str> = contract
        .token_dependencies
        .iter()
        .map(|t| t.path.as_str())
        .collect();
    assert!(paths.contains(&"tab.bar_background"));
    assert!(paths.contains(&"tab.active_background"));
    assert!(paths.contains(&"tab.inactive_background"));
}

#[test]
fn tabs_contract_serializes() {
    let contract = Tabs::contract();
    let json = serde_json::to_string_pretty(&contract).expect("serialize");
    assert!(json.contains("Tabs"));
    let deserialized: ComponentContract = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(deserialized.name, "Tabs");
}

#[test]
fn tabs_navigate_skips_disabled() {
    let tabs = vec![
        TabItem::new("A"),
        TabItem::new("B").set_disabled(true),
        TabItem::new("C"),
    ];
    let next = navigate_index(0, NavDirection::Next, tabs.len(), |i| tabs[i].disabled);
    assert_eq!(next, 2);
}

#[test]
fn tabs_navigate_horizontal_wraps() {
    let tabs = vec![TabItem::new("A"), TabItem::new("B"), TabItem::new("C")];
    let next = navigate_index(2, NavDirection::Next, tabs.len(), |i| tabs[i].disabled);
    assert_eq!(next, 0);
}

// ---- Cross-component tests ----

#[test]
fn all_poc_contracts_are_fork_disposition() {
    assert_eq!(Dialog::contract().disposition, Disposition::Fork);
    assert_eq!(Select::contract().disposition, Disposition::Fork);
    assert_eq!(Tabs::contract().disposition, Disposition::Fork);
}

#[test]
fn all_poc_contracts_validate() {
    for (name, contract) in [
        ("Dialog", Dialog::contract()),
        ("Select", Select::contract()),
        ("Tabs", Tabs::contract()),
    ] {
        let errors = contract.validate();
        assert!(
            errors.is_empty(),
            "{name} contract validation failed: {:?}",
            errors
        );
    }
}

#[test]
fn all_poc_contracts_have_no_hardcoded_colors_documented() {
    // Verify contracts document token dependencies (no hardcoded colors)
    for (name, contract) in [
        ("Dialog", Dialog::contract()),
        ("Select", Select::contract()),
        ("Tabs", Tabs::contract()),
    ] {
        assert!(
            !contract.token_dependencies.is_empty(),
            "{name} should document token dependencies"
        );
    }
}

#[test]
fn all_poc_contracts_have_interaction_checklists() {
    for (name, contract) in [
        ("Dialog", Dialog::contract()),
        ("Select", Select::contract()),
        ("Tabs", Tabs::contract()),
    ] {
        assert!(
            contract.interaction_checklist.focus_behavior.is_some(),
            "{name} should document focus behavior"
        );
        assert!(
            contract.interaction_checklist.keyboard_model.is_some(),
            "{name} should document keyboard model"
        );
        assert!(
            contract.interaction_checklist.pointer_behavior.is_some(),
            "{name} should document pointer behavior"
        );
        assert!(
            contract.interaction_checklist.state_model.is_some(),
            "{name} should document state model"
        );
    }
}
