//! Textarea component: multi-line text input with rows and resize control.
//!
//! Fork disposition: adapted from gpui-component multiline model.
//! Normalized to internal token/primitive contracts.
//!
//! Provenance:
//! - gpui-component multiline input patterns (MIT, Zed Industries)
//! - Modifications: Simplified to internal token system, stateless RenderOnce.

use gpui::*;
use theme::ActiveTheme;

/// Callback when the textarea value changes.
type OnChangeCallback = Box<dyn Fn(&str, &mut Window, &mut App) + 'static>;

/// A multi-line text input component with configurable rows,
/// and builder-pattern API mapped to frozen design tokens.
///
/// # Usage
/// ```ignore
/// Textarea::new("bio-textarea")
///     .value("Hello world")
///     .placeholder("Enter your bio...")
///     .rows(4)
/// ```
#[derive(IntoElement)]
pub struct Textarea {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
    rows: u32,
    disabled: bool,
    readonly: bool,
    error: bool,
    error_message: Option<SharedString>,
    on_change: Option<OnChangeCallback>,
    tooltip: Option<SharedString>,
    full_width: bool,
}

impl Textarea {
    /// Create a new empty textarea.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: SharedString::default(),
            placeholder: SharedString::default(),
            rows: 3,
            disabled: false,
            readonly: false,
            error: false,
            error_message: None,
            on_change: None,
            tooltip: None,
            full_width: false,
        }
    }

    /// Set the textarea value.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    /// Set the placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the number of visible rows.
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = rows;
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

    /// Set an error message.
    pub fn error_message(mut self, message: impl Into<SharedString>) -> Self {
        self.error_message = Some(message.into());
        self.error = true;
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

    /// Make the textarea take full width.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Returns the component contract for Textarea.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("Textarea", "0.1.0")
            .disposition(Disposition::Fork)
            .required_prop("id", "ElementId", "Unique identifier for the textarea")
            .optional_prop("value", "SharedString", "\"\"", "Current text value")
            .optional_prop("placeholder", "SharedString", "\"\"", "Placeholder text")
            .optional_prop("rows", "u32", "3", "Number of visible rows")
            .optional_prop("disabled", "bool", "false", "Whether disabled")
            .optional_prop("readonly", "bool", "false", "Whether read-only")
            .optional_prop("error", "bool", "false", "Whether in error state")
            .optional_prop(
                "error_message",
                "Option<SharedString>",
                "None",
                "Error message below textarea",
            )
            .optional_prop("tooltip", "Option<SharedString>", "None", "Tooltip text")
            .optional_prop("full_width", "bool", "false", "Take full container width")
            .state(ComponentState::Hover)
            .state(ComponentState::Active)
            .state(ComponentState::Focused)
            .state(ComponentState::Disabled)
            .state(ComponentState::Error)
            .state(ComponentState::Readonly)
            .token_dep("element.background", "Textarea background")
            .token_dep("element.hover", "Textarea hover background")
            .token_dep("element.disabled", "Disabled textarea background")
            .token_dep("text.default", "Textarea text color")
            .token_dep("text.placeholder", "Placeholder text color")
            .token_dep("text.disabled", "Disabled text color")
            .token_dep("border.default", "Textarea border")
            .token_dep("border.focused", "Focused textarea border")
            .token_dep("border.disabled", "Disabled textarea border")
            .token_dep("status.error.foreground", "Error message color")
            .token_dep("status.error.border", "Error state border")
            .focus_behavior("Tab/Shift-Tab navigates to/from textarea.")
            .keyboard_model("Standard multiline text input. Enter creates newline.")
            .pointer_behavior("Click focuses. Hover shows hover state.")
            .state_model(
                "Controlled value. Error state shows error border/message. \
                 Readonly allows focus but not editing.",
            )
            .disabled_behavior("Disabled textareas show muted styling and cannot be focused.")
            .readonly_behavior("Readonly textareas can be focused and selected but not edited.")
            .required_file("crates/components/src/textarea.rs")
            .build()
    }
}

impl RenderOnce for Textarea {
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
        let error_text_color = theme.status.error.foreground;
        let disabled = self.disabled;

        // Height based on rows (approximate 20px per row + padding)
        let row_height = px(20.0 * self.rows as f32 + 16.0);

        let mut field = div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .h(row_height)
            .p_3()
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .text_sm();

        if self.full_width {
            field = field.w_full();
        } else {
            field = field.min_w(px(200.0));
        }

        if !disabled {
            field = field.hover(move |s| s.border_color(hover_border));
        }

        // Value or placeholder
        if self.value.is_empty() {
            field = field.child(div().text_color(placeholder_color).child(self.placeholder));
        } else {
            field = field.child(div().text_color(text_color).child(self.value));
        }

        // Wrap with error message
        let mut wrapper = div().flex().flex_col().gap_1();
        if self.full_width {
            wrapper = wrapper.w_full();
        }
        wrapper = wrapper.child(field);

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
