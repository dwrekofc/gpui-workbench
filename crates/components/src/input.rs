//! Input component: single-line text input with validation and states.
//!
//! Fork disposition: adapted from gpui-component `input.rs` with Zed focus/keyboard alignment.
//! Normalized to internal token/primitive contracts.
//!
//! Provenance:
//! - gpui-component `crates/ui/src/input/input.rs` (MIT, Zed Industries)
//! - Zed focus/keyboard patterns (GPL-3.0/AGPL-3.0, Zed Industries)
//! - Modifications: Simplified to internal token system, stateless RenderOnce for Phase 1,
//!   uses internal primitives for state management.

use gpui::*;
use theme::ActiveTheme;

/// Input size controlling height and text size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    /// Small input: 28px height.
    Small,
    /// Medium input: 32px height (default).
    #[default]
    Medium,
    /// Large input: 36px height.
    Large,
}

/// Callback when the input value changes.
type OnChangeCallback = Box<dyn Fn(&str, &mut Window, &mut App) + 'static>;

/// A single-line text input component with placeholder, validation states,
/// and builder-pattern API mapped to frozen design tokens.
///
/// # Usage
/// ```ignore
/// Input::new("email-input")
///     .value("user@example.com")
///     .placeholder("Enter your email")
///     .on_change(|value, _window, _cx| {
///         println!("Value: {value}");
///     })
/// ```
#[derive(IntoElement)]
pub struct Input {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
    size: InputSize,
    disabled: bool,
    readonly: bool,
    error: bool,
    error_message: Option<SharedString>,
    prefix: Option<SharedString>,
    suffix: Option<SharedString>,
    on_change: Option<OnChangeCallback>,
    tooltip: Option<SharedString>,
    full_width: bool,
}

impl Input {
    /// Create a new empty input.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: SharedString::default(),
            placeholder: SharedString::default(),
            size: InputSize::Medium,
            disabled: false,
            readonly: false,
            error: false,
            error_message: None,
            prefix: None,
            suffix: None,
            on_change: None,
            tooltip: None,
            full_width: false,
        }
    }

    /// Set the input value.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    /// Set the placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the input size.
    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the readonly state.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Set the error state.
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Set an error message to display below the input.
    pub fn error_message(mut self, message: impl Into<SharedString>) -> Self {
        self.error_message = Some(message.into());
        self.error = true;
        self
    }

    /// Set a prefix label (e.g., "$" or "https://").
    pub fn prefix(mut self, prefix: impl Into<SharedString>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set a suffix label (e.g., ".com" or unit).
    pub fn suffix(mut self, suffix: impl Into<SharedString>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set the change handler.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Set a tooltip.
    pub fn set_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Make the input take full width.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Returns the component contract for Input.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("Input", "0.1.0")
            .disposition(Disposition::Fork)
            .required_prop("id", "ElementId", "Unique identifier for the input")
            .optional_prop("value", "SharedString", "\"\"", "Current input value")
            .optional_prop(
                "placeholder",
                "SharedString",
                "\"\"",
                "Placeholder text when empty",
            )
            .optional_prop(
                "size",
                "InputSize",
                "Medium",
                "Input size: Small, Medium, Large",
            )
            .optional_prop("disabled", "bool", "false", "Whether the input is disabled")
            .optional_prop(
                "readonly",
                "bool",
                "false",
                "Whether the input is read-only",
            )
            .optional_prop(
                "error",
                "bool",
                "false",
                "Whether the input is in error state",
            )
            .optional_prop(
                "error_message",
                "Option<SharedString>",
                "None",
                "Error message displayed below input",
            )
            .optional_prop("prefix", "Option<SharedString>", "None", "Prefix label")
            .optional_prop("suffix", "Option<SharedString>", "None", "Suffix label")
            .optional_prop("tooltip", "Option<SharedString>", "None", "Tooltip text")
            .optional_prop("full_width", "bool", "false", "Take full container width")
            .state(ComponentState::Hover)
            .state(ComponentState::Active)
            .state(ComponentState::Focused)
            .state(ComponentState::Disabled)
            .state(ComponentState::Error)
            .state(ComponentState::Readonly)
            .token_dep("element.background", "Input background")
            .token_dep("element.hover", "Input hover background")
            .token_dep("element.disabled", "Disabled input background")
            .token_dep("text.default", "Input text color")
            .token_dep("text.placeholder", "Placeholder text color")
            .token_dep("text.disabled", "Disabled text color")
            .token_dep("text.muted", "Prefix/suffix text color")
            .token_dep("border.default", "Input border")
            .token_dep("border.focused", "Focused input border")
            .token_dep("border.disabled", "Disabled input border")
            .token_dep("status.error.foreground", "Error message text color")
            .token_dep("status.error.border", "Error state border color")
            .focus_behavior("Tab/Shift-Tab navigates to/from input. Focus shows focused border.")
            .keyboard_model(
                "Standard text input keyboard behavior. \
                 All printable keys enter text. Backspace/Delete remove text.",
            )
            .pointer_behavior("Click focuses the input. Hover shows hover state.")
            .state_model(
                "Controlled value via prop. Error state shows error border and message. \
                 Readonly allows focus and selection but not editing.",
            )
            .disabled_behavior("Disabled inputs show muted styling and cannot be focused.")
            .readonly_behavior("Readonly inputs can be focused and selected but not edited.")
            .required_file("crates/components/src/input.rs")
            .build()
    }
}

impl RenderOnce for Input {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let (bg, border_color, text_color, placeholder_color) = if self.disabled {
            (
                theme.element.disabled,
                theme.border.disabled,
                theme.text.disabled,
                theme.text.disabled,
            )
        } else if self.error {
            (
                theme.element.background,
                theme.status.error.border,
                theme.text.default,
                theme.text.placeholder,
            )
        } else {
            (
                theme.element.background,
                theme.border.default,
                theme.text.default,
                theme.text.placeholder,
            )
        };

        let hover_border = if self.error {
            theme.status.error.border
        } else {
            theme.border.focused
        };
        let affix_color = theme.text.muted;
        let error_text_color = theme.status.error.foreground;

        let height = match self.size {
            InputSize::Small => px(28.0),
            InputSize::Medium => px(32.0),
            InputSize::Large => px(36.0),
        };

        let disabled = self.disabled;

        // Input field container
        let mut field = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .h(height)
            .px_3()
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded_md();

        if self.full_width {
            field = field.w_full();
        } else {
            field = field.min_w(px(200.0));
        }

        if !disabled {
            field = field.hover(move |s| s.border_color(hover_border));
        }

        // Text size
        field = match self.size {
            InputSize::Small => field.text_xs(),
            InputSize::Medium => field.text_sm(),
            InputSize::Large => field.text_sm(),
        };

        // Prefix
        if let Some(prefix) = self.prefix {
            field = field.child(
                div()
                    .text_color(affix_color)
                    .mr_1()
                    .flex_shrink_0()
                    .child(prefix),
            );
        }

        // Value or placeholder
        if self.value.is_empty() {
            field = field.child(
                div()
                    .flex_1()
                    .text_color(placeholder_color)
                    .child(self.placeholder),
            );
        } else {
            field = field.child(div().flex_1().text_color(text_color).child(self.value));
        }

        // Suffix
        if let Some(suffix) = self.suffix {
            field = field.child(
                div()
                    .text_color(affix_color)
                    .ml_1()
                    .flex_shrink_0()
                    .child(suffix),
            );
        }

        // Wrap with error message
        let mut wrapper = div().flex().flex_col().gap_1();
        if self.full_width {
            wrapper = wrapper.w_full();
        }

        wrapper = wrapper.child(field);

        // Error message below input
        if let Some(error_msg) = self.error_message {
            wrapper = wrapper.child(
                div()
                    .text_xs()
                    .text_color(error_text_color)
                    .child(error_msg),
            );
        }

        wrapper
    }
}
