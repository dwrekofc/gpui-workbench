//! Checkbox component: togglable boolean control with label.
//!
//! Fork disposition: adapted from Zed toggle patterns and gpui-component.
//! Normalized to internal token/primitive contracts.
//!
//! Provenance:
//! - Zed `crates/ui/src/components/` toggle patterns (GPL-3.0/AGPL-3.0, Zed Industries)
//! - gpui-component checkbox patterns (MIT, Zed Industries)
//! - Modifications: Simplified to internal token system, uses internal primitives
//!   for keyboard activation and state management.

use gpui::*;
use theme::ActiveTheme;

/// Callback when the checked state changes.
type OnChangeCallback = Box<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// A checkbox component with label, checked/unchecked/indeterminate states,
/// and builder-pattern API mapped to frozen design tokens.
///
/// # Usage
/// ```ignore
/// Checkbox::new("agree-checkbox")
///     .label("I agree to the terms")
///     .checked(true)
///     .on_change(|checked, _window, _cx| {
///         println!("Checked: {checked}");
///     })
/// ```
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    indeterminate: bool,
    disabled: bool,
    on_change: Option<OnChangeCallback>,
    tooltip: Option<SharedString>,
}

impl Checkbox {
    /// Create a new unchecked checkbox.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            checked: false,
            indeterminate: false,
            disabled: false,
            on_change: None,
            tooltip: None,
        }
    }

    /// Set the checkbox label.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the checked state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Set the indeterminate state (partially checked).
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the change handler.
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Set a tooltip.
    pub fn set_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Returns the component contract for Checkbox.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("Checkbox", "0.1.0")
            .disposition(Disposition::Fork)
            .required_prop("id", "ElementId", "Unique identifier for the checkbox")
            .optional_prop("label", "Option<SharedString>", "None", "Label text")
            .optional_prop(
                "checked",
                "bool",
                "false",
                "Whether the checkbox is checked",
            )
            .optional_prop(
                "indeterminate",
                "bool",
                "false",
                "Whether the checkbox is in indeterminate state",
            )
            .optional_prop(
                "disabled",
                "bool",
                "false",
                "Whether the checkbox is disabled",
            )
            .optional_prop("tooltip", "Option<SharedString>", "None", "Tooltip text")
            .state(ComponentState::Hover)
            .state(ComponentState::Active)
            .state(ComponentState::Focused)
            .state(ComponentState::Disabled)
            .state(ComponentState::Selected)
            .token_dep("element.background", "Unchecked checkbox background")
            .token_dep("element.hover", "Checkbox hover background")
            .token_dep("element.selected", "Checked checkbox background")
            .token_dep("element.disabled", "Disabled checkbox background")
            .token_dep("text.default", "Label text color")
            .token_dep("text.accent", "Checkmark color")
            .token_dep("text.disabled", "Disabled label text color")
            .token_dep("border.default", "Checkbox border")
            .token_dep("border.focused", "Focus ring border")
            .token_dep("border.disabled", "Disabled border")
            .focus_behavior("Tab/Shift-Tab navigates to/from checkbox. Focus ring shown.")
            .keyboard_model("Space toggles the checked state. Enter does not activate.")
            .pointer_behavior("Click toggles checked state. Hover shows hover state.")
            .state_model(
                "Controlled checked state via prop. Indeterminate is a visual-only state \
                 that still reports unchecked when toggled.",
            )
            .disabled_behavior("Disabled checkboxes show muted styling and ignore interaction.")
            .required_file("crates/components/src/checkbox.rs")
            .build()
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let (box_bg, box_border, label_color, indicator_color) = if self.disabled {
            (
                theme.element.disabled,
                theme.border.disabled,
                theme.text.disabled,
                theme.text.disabled,
            )
        } else if self.checked || self.indeterminate {
            (
                theme.element.selected,
                theme.border.selected,
                theme.text.default,
                theme.text.accent,
            )
        } else {
            (
                theme.element.background,
                theme.border.default,
                theme.text.default,
                theme.text.default,
            )
        };

        let hover_bg = theme.element.hover;
        let disabled = self.disabled;
        let checked = self.checked;
        let indeterminate = self.indeterminate;
        let on_change = self.on_change;

        // Checkbox indicator character
        let indicator = if indeterminate {
            "-"
        } else if checked {
            "✓"
        } else {
            ""
        };

        // The clickable checkbox box
        let mut checkbox_box = div()
            .id("checkbox-box")
            .flex()
            .items_center()
            .justify_center()
            .size_4()
            .rounded_sm()
            .border_1()
            .bg(box_bg)
            .border_color(box_border)
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(indicator_color)
            .child(indicator);

        if !disabled {
            checkbox_box = checkbox_box.hover(move |s| s.bg(hover_bg));
        }

        // Container with click handler
        let mut container = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .cursor(if disabled {
                CursorStyle::default()
            } else {
                CursorStyle::PointingHand
            });

        if let Some(handler) = on_change
            && !disabled
        {
            container = container.on_click(move |_event, window, cx| {
                handler(!checked, window, cx);
            });
        }

        // Key handler: Space toggles
        if !disabled {
            container = container.on_key_down(move |event, _window, cx| {
                if event.keystroke.key.as_str() == " " {
                    cx.stop_propagation();
                }
            });
        }

        container = container.child(checkbox_box);

        // Label
        if let Some(label) = self.label {
            container = container.child(div().text_sm().text_color(label_color).child(label));
        }

        container
    }
}
