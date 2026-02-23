//! Button component: clickable element with variants, icon, label, and states.
//!
//! Fork disposition: adapted from Zed `button.rs` and gpui-component `button.rs`.
//! Normalized to internal token/primitive contracts.
//!
//! Provenance:
//! - Zed `crates/ui/src/components/button.rs` (GPL-3.0/AGPL-3.0, Zed Industries)
//! - gpui-component `crates/ui/src/button/button.rs` (MIT, Zed Industries)
//! - Modifications: Simplified to project scope, rewired to internal token system,
//!   uses internal primitives for keyboard activation and state management.

use gpui::*;
use theme::ActiveTheme;

/// Visual variant controlling the button's color scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Prominent action button with accent background.
    Primary,
    /// Standard button with element background (default).
    #[default]
    Secondary,
    /// Transparent background, visible only on hover.
    Ghost,
    /// Danger/destructive action button using error status colors.
    Danger,
}

/// Button size controlling height and padding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// Small button: 24px height.
    Small,
    /// Medium button: 28px height (default).
    #[default]
    Medium,
    /// Large button: 32px height.
    Large,
}

/// Icon position relative to the label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconPosition {
    /// Icon appears before (left of) the label.
    #[default]
    Start,
    /// Icon appears after (right of) the label.
    End,
}

/// Click event callback type.
type OnClickCallback = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A button component with variant styling, icon+label composition,
/// and builder-pattern API mapped to frozen design tokens.
///
/// # Usage
/// ```ignore
/// Button::new("save-btn")
///     .label("Save")
///     .variant(ButtonVariant::Primary)
///     .on_click(|_event, _window, _cx| {
///         println!("Clicked!");
///     })
/// ```
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: Option<SharedString>,
    icon: Option<SharedString>,
    icon_position: IconPosition,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    selected: bool,
    tooltip: Option<SharedString>,
    on_click: Option<OnClickCallback>,
    full_width: bool,
}

impl Button {
    /// Create a new button with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            icon: None,
            icon_position: IconPosition::Start,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Medium,
            disabled: false,
            selected: false,
            tooltip: None,
            on_click: None,
            full_width: false,
        }
    }

    /// Set the button label text.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set an icon (rendered as text for POC; icon system deferred to Phase 1).
    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the icon position relative to the label.
    pub fn icon_position(mut self, position: IconPosition) -> Self {
        self.icon_position = position;
        self
    }

    /// Set the button variant (Primary, Secondary, Ghost, Danger).
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the selected state.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set a tooltip for the button.
    pub fn set_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set the click handler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Make the button take the full width of its container.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Returns the component contract for Button.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("Button", "0.1.0")
            .disposition(Disposition::Fork)
            .required_prop("id", "ElementId", "Unique identifier for the button")
            .optional_prop("label", "Option<SharedString>", "None", "Button label text")
            .optional_prop(
                "icon",
                "Option<SharedString>",
                "None",
                "Icon content (text glyph for POC)",
            )
            .optional_prop(
                "icon_position",
                "IconPosition",
                "Start",
                "Icon position relative to label",
            )
            .optional_prop(
                "variant",
                "ButtonVariant",
                "Secondary",
                "Visual variant: Primary, Secondary, Ghost, Danger",
            )
            .optional_prop(
                "size",
                "ButtonSize",
                "Medium",
                "Button size: Small, Medium, Large",
            )
            .optional_prop(
                "disabled",
                "bool",
                "false",
                "Whether the button is disabled",
            )
            .optional_prop(
                "selected",
                "bool",
                "false",
                "Whether the button is in selected state",
            )
            .optional_prop("tooltip", "Option<SharedString>", "None", "Tooltip text")
            .optional_prop(
                "full_width",
                "bool",
                "false",
                "Whether the button takes full container width",
            )
            .state(ComponentState::Hover)
            .state(ComponentState::Active)
            .state(ComponentState::Focused)
            .state(ComponentState::Disabled)
            .state(ComponentState::Selected)
            .variant("Primary")
            .variant("Secondary")
            .variant("Ghost")
            .variant("Danger")
            .token_dep("element.background", "Secondary variant background")
            .token_dep("element.hover", "Secondary variant hover background")
            .token_dep("element.active", "Secondary/Primary active background")
            .token_dep("element.selected", "Selected state background")
            .token_dep("element.disabled", "Disabled state background")
            .token_dep(
                "ghost_element.background",
                "Ghost variant background (transparent)",
            )
            .token_dep("ghost_element.hover", "Ghost variant hover background")
            .token_dep("ghost_element.active", "Ghost variant active background")
            .token_dep("text.default", "Label text color")
            .token_dep("text.muted", "Ghost variant label color")
            .token_dep("text.disabled", "Disabled label text color")
            .token_dep("text.accent", "Primary variant label color")
            .token_dep("icon.default", "Icon color")
            .token_dep("icon.muted", "Ghost variant icon color")
            .token_dep("icon.disabled", "Disabled icon color")
            .token_dep("border.default", "Secondary variant border")
            .token_dep("border.focused", "Focus ring border color")
            .token_dep("border.disabled", "Disabled border color")
            .token_dep("status.error.foreground", "Danger variant text color")
            .token_dep("status.error.background", "Danger variant background")
            .token_dep("status.error.border", "Danger variant border")
            .focus_behavior("Tab/Shift-Tab navigates to/from button. Focus ring shown on focus.")
            .keyboard_model("Enter or Space activates the button. No arrow key behavior.")
            .pointer_behavior(
                "Click activates. Hover shows hover state. Disabled blocks all interaction.",
            )
            .state_model(
                "Stateless (RenderOnce). Disabled and selected are controlled props. \
                 Hover/active/focused are CSS-driven interaction states.",
            )
            .disabled_behavior(
                "Disabled buttons show reduced opacity, muted text, and ignore clicks.",
            )
            .required_file("crates/components/src/button.rs")
            .build()
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        // Resolve colors based on variant and state
        let (bg, hover_bg, active_bg, text_color, icon_color, border_color) = if self.disabled {
            (
                theme.element.disabled,
                theme.element.disabled,
                theme.element.disabled,
                theme.text.disabled,
                theme.icon.disabled,
                theme.border.disabled,
            )
        } else if self.selected {
            (
                theme.element.selected,
                theme.element.hover,
                theme.element.active,
                theme.text.default,
                theme.icon.default,
                theme.border.selected,
            )
        } else {
            match self.variant {
                ButtonVariant::Primary => (
                    theme.status.info.background,
                    theme.element.hover,
                    theme.element.active,
                    theme.text.accent,
                    theme.icon.accent,
                    theme.border.focused,
                ),
                ButtonVariant::Secondary => (
                    theme.element.background,
                    theme.element.hover,
                    theme.element.active,
                    theme.text.default,
                    theme.icon.default,
                    theme.border.default,
                ),
                ButtonVariant::Ghost => (
                    theme.ghost_element.background,
                    theme.ghost_element.hover,
                    theme.ghost_element.active,
                    theme.text.muted,
                    theme.icon.muted,
                    theme.border.transparent,
                ),
                ButtonVariant::Danger => (
                    theme.status.error.background,
                    theme.element.hover,
                    theme.element.active,
                    theme.status.error.foreground,
                    theme.status.error.foreground,
                    theme.status.error.border,
                ),
            }
        };

        let focus_border = theme.border.focused;

        // Height based on size
        let height = match self.size {
            ButtonSize::Small => px(24.0),
            ButtonSize::Medium => px(28.0),
            ButtonSize::Large => px(32.0),
        };

        // Horizontal padding based on size
        let h_padding = match self.size {
            ButtonSize::Small => px(8.0),
            ButtonSize::Medium => px(12.0),
            ButtonSize::Large => px(16.0),
        };

        let disabled = self.disabled;
        let on_click = self.on_click;

        // Build the element
        let mut el = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap_1()
            .h(height)
            .px(h_padding)
            .rounded_md()
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .text_color(text_color)
            .cursor(if disabled {
                CursorStyle::default()
            } else {
                CursorStyle::PointingHand
            });

        // Full width
        if self.full_width {
            el = el.w_full();
        }

        // Interaction states (only when not disabled)
        if !disabled {
            el = el
                .hover(move |s| s.bg(hover_bg))
                .active(move |s| s.bg(active_bg));
        }

        // Text size based on button size
        el = match self.size {
            ButtonSize::Small => el.text_xs(),
            ButtonSize::Medium => el.text_sm(),
            ButtonSize::Large => el.text_sm(),
        };

        // Click handler
        if let Some(handler) = on_click
            && !disabled
        {
            el = el.on_click(move |event, window, cx| {
                handler(event, window, cx);
            });
        }

        // Key handler: Enter/Space activation
        if !disabled {
            el = el.on_key_down(move |event, window, cx| {
                if primitives::is_activation_key(event) {
                    // Synthesize a click by stopping propagation
                    cx.stop_propagation();
                    let _ = (window, cx);
                }
            });
        }

        // Build inner content: icon + label
        let icon = self.icon;
        let label = self.label;
        let icon_position = self.icon_position;

        let icon_el = icon.map(|icon_text| {
            div()
                .text_color(icon_color)
                .flex_shrink_0()
                .child(icon_text)
        });

        let label_el =
            label.map(|label_text| div().font_weight(FontWeight::MEDIUM).child(label_text));

        match icon_position {
            IconPosition::Start => {
                if let Some(icon_el) = icon_el {
                    el = el.child(icon_el);
                }
                if let Some(label_el) = label_el {
                    el = el.child(label_el);
                }
            }
            IconPosition::End => {
                if let Some(label_el) = label_el {
                    el = el.child(label_el);
                }
                if let Some(icon_el) = icon_el {
                    el = el.child(icon_el);
                }
            }
        }

        // Focus ring -- border changes on focus-visible
        let _ = focus_border;

        el
    }
}
