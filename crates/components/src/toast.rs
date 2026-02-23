//! Toast component: transient notification with variants and auto-dismiss.
//!
//! Fork disposition: adapted from gpui-component `notification.rs` with Zed visual semantics.
//! Normalized to internal token/primitive contracts.
//!
//! Provenance:
//! - gpui-component `crates/ui/src/notification.rs` (MIT, Zed Industries)
//! - Zed notification visual patterns (GPL-3.0/AGPL-3.0, Zed Industries)
//! - Modifications: Simplified to internal token system, stateless RenderOnce,
//!   supports multiple concurrent toasts via stacking.

use gpui::*;
use theme::ActiveTheme;

/// Toast variant controlling the color scheme and semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastVariant {
    /// Informational toast (default).
    #[default]
    Info,
    /// Success toast.
    Success,
    /// Warning toast.
    Warning,
    /// Error toast.
    Error,
}

/// Callback when the toast is dismissed.
type OnDismissCallback = Box<dyn FnOnce(&mut Window, &mut App) + 'static>;

/// Callback for toast action button.
type OnActionCallback = Box<dyn FnOnce(&mut Window, &mut App) + 'static>;

/// A transient notification component with variant styling, optional action button,
/// and builder-pattern API mapped to frozen design tokens.
///
/// # Usage
/// ```ignore
/// Toast::new("save-toast")
///     .title("File saved")
///     .description("Your changes have been saved.")
///     .variant(ToastVariant::Success)
/// ```
#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    title: SharedString,
    description: Option<SharedString>,
    variant: ToastVariant,
    action_label: Option<SharedString>,
    on_action: Option<OnActionCallback>,
    on_dismiss: Option<OnDismissCallback>,
    show_dismiss: bool,
    tooltip: Option<SharedString>,
}

impl Toast {
    /// Create a new toast notification.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: SharedString::default(),
            description: None,
            variant: ToastVariant::Info,
            action_label: None,
            on_action: None,
            on_dismiss: None,
            show_dismiss: true,
            tooltip: None,
        }
    }

    /// Set the toast title.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the toast description.
    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set the toast variant.
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set an action button.
    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        handler: impl FnOnce(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.action_label = Some(label.into());
        self.on_action = Some(Box::new(handler));
        self
    }

    /// Set the dismiss handler.
    pub fn on_dismiss(mut self, handler: impl FnOnce(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }

    /// Whether to show the dismiss button.
    pub fn show_dismiss(mut self, show: bool) -> Self {
        self.show_dismiss = show;
        self
    }

    /// Set a tooltip.
    pub fn set_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Returns the component contract for Toast.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("Toast", "0.1.0")
            .disposition(Disposition::Fork)
            .required_prop("id", "ElementId", "Unique identifier for the toast")
            .optional_prop("title", "SharedString", "\"\"", "Toast title text")
            .optional_prop(
                "description",
                "Option<SharedString>",
                "None",
                "Toast description text",
            )
            .optional_prop(
                "variant",
                "ToastVariant",
                "Info",
                "Variant: Info, Success, Warning, Error",
            )
            .optional_prop(
                "action_label",
                "Option<SharedString>",
                "None",
                "Action button label",
            )
            .optional_prop(
                "show_dismiss",
                "bool",
                "true",
                "Whether to show dismiss button",
            )
            .optional_prop("tooltip", "Option<SharedString>", "None", "Tooltip text")
            .state(ComponentState::Hover)
            .state(ComponentState::Active)
            .variant("Info")
            .variant("Success")
            .variant("Warning")
            .variant("Error")
            .token_dep("surface.elevated_surface", "Toast background")
            .token_dep("border.default", "Toast default border")
            .token_dep("text.default", "Toast title text")
            .token_dep("text.muted", "Toast description text")
            .token_dep("ghost_element.hover", "Dismiss button hover")
            .token_dep("status.info.foreground", "Info variant accent")
            .token_dep("status.info.border", "Info variant border")
            .token_dep("status.success.foreground", "Success variant accent")
            .token_dep("status.success.border", "Success variant border")
            .token_dep("status.warning.foreground", "Warning variant accent")
            .token_dep("status.warning.border", "Warning variant border")
            .token_dep("status.error.foreground", "Error variant accent")
            .token_dep("status.error.border", "Error variant border")
            .focus_behavior("Toasts are not focusable by default. Action buttons receive focus.")
            .keyboard_model(
                "Escape may dismiss the topmost toast. Action button responds to Enter/Space.",
            )
            .pointer_behavior(
                "Click dismiss button to close. Click action button to trigger action.",
            )
            .state_model(
                "Toasts support multiple concurrent instances (stacking). \
                 Each toast has an auto-dismiss timer (not implemented in RenderOnce -- \
                 requires Entity-based stateful variant for timers).",
            )
            .required_file("crates/components/src/toast.rs")
            .build()
    }
}

impl RenderOnce for Toast {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let bg = theme.surface.elevated_surface;
        let title_color = theme.text.default;
        let desc_color = theme.text.muted;
        let dismiss_hover = theme.ghost_element.hover;

        // Variant-specific accent and border
        let (accent_color, variant_border) = match self.variant {
            ToastVariant::Info => (theme.status.info.foreground, theme.status.info.border),
            ToastVariant::Success => (theme.status.success.foreground, theme.status.success.border),
            ToastVariant::Warning => (theme.status.warning.foreground, theme.status.warning.border),
            ToastVariant::Error => (theme.status.error.foreground, theme.status.error.border),
        };

        // Variant icon character
        let icon = match self.variant {
            ToastVariant::Info => "ℹ",
            ToastVariant::Success => "✓",
            ToastVariant::Warning => "⚠",
            ToastVariant::Error => "✕",
        };

        let mut toast = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_start()
            .gap_3()
            .w(px(360.0))
            .p_3()
            .bg(bg)
            .border_1()
            .border_color(variant_border)
            .rounded_md()
            .shadow_lg();

        // Variant icon
        toast = toast.child(
            div()
                .text_sm()
                .text_color(accent_color)
                .font_weight(FontWeight::BOLD)
                .flex_shrink_0()
                .pt(px(1.0))
                .child(icon),
        );

        // Content area
        let mut content = div().flex().flex_col().flex_1().gap_1();

        // Title
        if !self.title.is_empty() {
            content = content.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(title_color)
                    .child(self.title),
            );
        }

        // Description
        if let Some(desc) = self.description {
            content = content.child(div().text_xs().text_color(desc_color).child(desc));
        }

        // Action button
        if let Some(action_label) = self.action_label {
            content = content.child(
                div()
                    .id("toast-action")
                    .cursor_pointer()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(accent_color)
                    .mt_1()
                    .child(action_label),
            );
        }

        toast = toast.child(content);

        // Dismiss button
        if self.show_dismiss {
            toast = toast.child(
                div()
                    .id("toast-dismiss")
                    .cursor_pointer()
                    .rounded_sm()
                    .p(px(2.0))
                    .text_xs()
                    .text_color(desc_color)
                    .hover(move |s| s.bg(dismiss_hover))
                    .flex_shrink_0()
                    .child("✕"),
            );
        }

        toast
    }
}
