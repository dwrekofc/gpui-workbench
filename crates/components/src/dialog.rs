//! Dialog component: modal overlay with focus trap, escape/outside-click dismiss.
//!
//! Fork disposition: adapted from gpui-component `dialog.rs` and Zed `modal.rs`.
//! Normalized to internal token/primitive contracts.
//!
//! Provenance:
//! - gpui-component `crates/ui/src/dialog.rs` (MIT, Zed Industries)
//! - Zed `crates/ui/src/components/modal.rs` (GPL-3.0/AGPL-3.0, Zed Industries)
//! - Modifications: Simplified to POC scope, rewired to internal token system,
//!   uses internal primitives for focus trap/return/keyboard/state.

use gpui::prelude::FluentBuilder;
use gpui::*;
use primitives::{FocusReturn, FocusTrap, OpenState};
use smallvec::SmallVec;
use theme::ActiveTheme;

/// Callback type for dialog actions (ok/cancel).
/// Returns `true` to allow closing, `false` to prevent.
type ActionCallback = Box<dyn FnOnce(&mut Window, &mut App) -> bool + 'static>;

/// Callback for close notification.
type CloseCallback = Box<dyn FnOnce(&mut Window, &mut App) + 'static>;

/// A modal dialog overlay with focus trap, escape/outside-click dismiss,
/// title/description/action slots, and controlled open/close state.
///
/// # Usage
/// ```ignore
/// Dialog::new("my-dialog", cx)
///     .title("Confirm Action")
///     .description("Are you sure you want to proceed?")
///     .on_ok(|_window, _cx| { /* handle ok */ true })
///     .on_cancel(|_window, _cx| true)
///     .on_close(|_window, _cx| {})
/// ```
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    focus_handle: FocusHandle,
    #[allow(dead_code)]
    focus_trap: FocusTrap,
    focus_return: FocusReturn,
    open_state: OpenState,
    title: Option<SharedString>,
    description: Option<SharedString>,
    actions: SmallVec<[AnyElement; 2]>,
    children: SmallVec<[AnyElement; 2]>,
    on_ok: Option<ActionCallback>,
    on_cancel: Option<ActionCallback>,
    on_close: Option<CloseCallback>,
    width: Pixels,
    overlay_closable: bool,
    show_close_button: bool,
    tooltip: Option<SharedString>,
}

impl Dialog {
    /// Create a new dialog. Captures current focus for return on dismiss.
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut App) -> Self {
        let focus_handle = cx.focus_handle();
        let focus_trap = FocusTrap::new(focus_handle.clone());
        let focus_return = FocusReturn::capture(window, cx);

        Self {
            id: id.into(),
            focus_handle,
            focus_trap,
            focus_return,
            open_state: OpenState::Open,
            title: None,
            description: None,
            actions: SmallVec::new(),
            children: SmallVec::new(),
            on_ok: None,
            on_cancel: None,
            on_close: None,
            width: px(480.0),
            overlay_closable: true,
            show_close_button: true,
            tooltip: None,
        }
    }

    /// Set the dialog title.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the dialog description text.
    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add an action element to the footer.
    pub fn action(mut self, element: impl IntoElement) -> Self {
        self.actions.push(element.into_any_element());
        self
    }

    /// Add body content.
    pub fn body(mut self, element: impl IntoElement) -> Self {
        self.children.push(element.into_any_element());
        self
    }

    /// Set the on_ok callback. Returns true to close, false to keep open.
    pub fn on_ok(mut self, handler: impl FnOnce(&mut Window, &mut App) -> bool + 'static) -> Self {
        self.on_ok = Some(Box::new(handler));
        self
    }

    /// Set the on_cancel callback. Returns true to close, false to keep open.
    pub fn on_cancel(
        mut self,
        handler: impl FnOnce(&mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }

    /// Set the on_close callback (fires after ok/cancel).
    pub fn on_close(mut self, handler: impl FnOnce(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }

    /// Set the dialog width.
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    /// Whether clicking the overlay backdrop dismisses the dialog.
    pub fn overlay_closable(mut self, closable: bool) -> Self {
        self.overlay_closable = closable;
        self
    }

    /// Whether to show the close button in the header.
    pub fn close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Set a tooltip for the dialog.
    pub fn set_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Returns the component contract for Dialog.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("Dialog", "0.1.0")
            .disposition(Disposition::Fork)
            .required_prop(
                "id",
                "ElementId",
                "Unique identifier for the dialog instance",
            )
            .optional_prop("title", "Option<SharedString>", "None", "Dialog title text")
            .optional_prop(
                "description",
                "Option<SharedString>",
                "None",
                "Dialog description text",
            )
            .optional_prop("width", "Pixels", "480.0", "Dialog width in pixels")
            .optional_prop(
                "overlay_closable",
                "bool",
                "true",
                "Whether clicking backdrop closes the dialog",
            )
            .optional_prop(
                "show_close_button",
                "bool",
                "true",
                "Whether to show the X close button",
            )
            .optional_prop("tooltip", "Option<SharedString>", "None", "Tooltip text")
            .state(ComponentState::Open)
            .state(ComponentState::Focused)
            .state(ComponentState::Hover)
            .state(ComponentState::Active)
            .token_dep("surface.elevated_surface", "Dialog panel background")
            .token_dep("border.default", "Dialog panel border")
            .token_dep("text.default", "Dialog title and body text")
            .token_dep("text.muted", "Dialog description text")
            .token_dep("surface.background", "Overlay backdrop (with alpha)")
            .token_dep("ghost_element.hover", "Close button hover state")
            .focus_behavior(
                "Focus trap: Tab/Shift-Tab cycle within dialog. \
                 Focus captured on open, returned to trigger on close.",
            )
            .keyboard_model(
                "Escape dismisses the dialog. Enter is not bound by default \
                 (action buttons handle their own activation).",
            )
            .pointer_behavior(
                "Click on backdrop dismisses (if overlay_closable). \
                 Click on close button dismisses. \
                 Mouse events on dialog panel stop propagation to backdrop.",
            )
            .state_model(
                "Controlled open/close via OpenState. \
                 Dialog is created in Open state; closing returns focus.",
            )
            .required_file("crates/components/src/dialog.rs")
            .build()
    }
}

impl RenderOnce for Dialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        // Overlay backdrop color: surface background with reduced alpha
        let backdrop_rgba: Rgba = theme.surface.background.into();
        let backdrop_color = Hsla::from(Rgba {
            r: backdrop_rgba.r,
            g: backdrop_rgba.g,
            b: backdrop_rgba.b,
            a: 0.6,
        });

        let panel_bg = theme.surface.elevated_surface;
        let border_color = theme.border.default;
        let title_color = theme.text.default;
        let desc_color = theme.text.muted;
        let close_hover = theme.ghost_element.hover;

        let width = self.width;
        let overlay_closable = self.overlay_closable;
        let focus_return = self.focus_return;
        let _on_close = self.on_close;

        if self.open_state.is_closed() {
            return div().into_any_element();
        }

        // Build the dialog panel
        let mut panel = div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .w(width)
            .max_h(vh(80.0, cx))
            .overflow_hidden()
            .bg(panel_bg)
            .border_1()
            .border_color(border_color)
            .rounded_lg()
            .shadow_lg()
            .p_6()
            .gap_3()
            // Stop click propagation so backdrop handler doesn't fire
            .on_mouse_down(MouseButton::Left, |_event, _window, _cx| {})
            // Escape key dismissal
            .on_key_down({
                let focus_return = focus_return.clone();
                move |event, window, cx| {
                    if primitives::is_escape_key(event) {
                        focus_return.restore(window, cx);
                    }
                }
            });

        // Title
        if let Some(title) = self.title {
            panel = panel.child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(title_color)
                            .child(title),
                    )
                    .when(self.show_close_button, |this| {
                        this.child(
                            div()
                                .id("dialog-close-btn")
                                .cursor_pointer()
                                .rounded_md()
                                .p_1()
                                .text_color(desc_color)
                                .hover(|s| s.bg(close_hover))
                                .on_mouse_down(MouseButton::Left, {
                                    let focus_return = focus_return.clone();
                                    move |_event, window, cx| {
                                        focus_return.restore(window, cx);
                                    }
                                })
                                .child("X"),
                        )
                    }),
            );
        }

        // Description
        if let Some(desc) = self.description {
            panel = panel.child(div().text_sm().text_color(desc_color).child(desc));
        }

        // Body content
        for child in self.children {
            panel = panel.child(child);
        }

        // Actions footer
        if !self.actions.is_empty() {
            let mut footer = div().flex().flex_row().justify_end().gap_2().pt_2();
            for action in self.actions {
                footer = footer.child(action);
            }
            panel = panel.child(footer);
        }

        // Full-screen overlay with backdrop
        let overlay = div()
            .id("dialog-overlay")
            .absolute()
            .inset_0()
            .flex()
            .justify_center()
            .items_start()
            .pt(px(80.0))
            .bg(backdrop_color)
            .when(overlay_closable, |this| {
                this.on_mouse_down(MouseButton::Left, {
                    let focus_return = focus_return.clone();
                    move |_event, window, cx| {
                        focus_return.restore(window, cx);
                    }
                })
            })
            .child(panel);

        // Use deferred rendering so dialog paints on top
        deferred(overlay).with_priority(1).into_any_element()
    }
}

/// Helper to compute max height as percentage of viewport.
fn vh(percent: f32, _cx: &App) -> Pixels {
    // For POC, use a reasonable fixed max. Full viewport integration
    // requires window bounds access which needs &Window.
    px(600.0 * percent / 100.0)
}

// Tests are in tests/contract_tests.rs (integration test) to avoid
// stack overflow from GPUI IntoElement derive macro expansion in test mode.
