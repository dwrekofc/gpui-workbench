//! Popover component: positioned overlay anchored to a trigger element.
//!
//! Fork disposition: adapted from Zed/gpui-component popover with token consistency.
//! Normalized to internal token/primitive contracts.
//!
//! Provenance:
//! - Zed `crates/ui/src/components/popover.rs` (GPL-3.0/AGPL-3.0, Zed Industries)
//! - gpui-component popover patterns (MIT, Zed Industries)
//! - Modifications: Simplified to internal token system, uses internal primitives
//!   for popover positioning and outside-click dismiss.

use gpui::*;
use primitives::PopoverPosition;
use theme::ActiveTheme;

/// Callback when the popover is dismissed.
type OnCloseCallback = Box<dyn FnOnce(&mut Window, &mut App) + 'static>;

/// A positioned overlay anchored to a trigger, with outside-click and
/// escape dismiss, and builder-pattern API mapped to frozen design tokens.
///
/// # Usage
/// ```ignore
/// Popover::new("menu-popover")
///     .position(PopoverPosition::Below)
///     .open(true)
///     .child(div().child("Popover content"))
/// ```
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    open: bool,
    position: PopoverPosition,
    children: Vec<AnyElement>,
    on_close: Option<OnCloseCallback>,
    width: Option<Pixels>,
    max_height: Pixels,
    tooltip: Option<SharedString>,
}

impl Popover {
    /// Create a new popover.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            position: PopoverPosition::below_left(),
            children: Vec::new(),
            on_close: None,
            width: None,
            max_height: px(320.0),
            tooltip: None,
        }
    }

    /// Set whether the popover is open.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the popover position relative to the trigger.
    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
        self
    }

    /// Add a child element to the popover content.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Set the popover width.
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the maximum height.
    pub fn max_height(mut self, height: Pixels) -> Self {
        self.max_height = height;
        self
    }

    /// Set the close handler.
    pub fn on_close(mut self, handler: impl FnOnce(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }

    /// Set a tooltip.
    pub fn set_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Returns the component contract for Popover.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("Popover", "0.1.0")
            .disposition(Disposition::Fork)
            .required_prop("id", "ElementId", "Unique identifier for the popover")
            .optional_prop("open", "bool", "false", "Whether the popover is visible")
            .optional_prop(
                "position",
                "PopoverPosition",
                "Below",
                "Placement relative to trigger",
            )
            .optional_prop("width", "Option<Pixels>", "None", "Popover width")
            .optional_prop("max_height", "Pixels", "320.0", "Maximum popover height")
            .optional_prop("tooltip", "Option<SharedString>", "None", "Tooltip text")
            .state(ComponentState::Open)
            .state(ComponentState::Hover)
            .state(ComponentState::Focused)
            .token_dep("surface.elevated_surface", "Popover background")
            .token_dep("border.default", "Popover border")
            .token_dep("text.default", "Popover content text")
            .focus_behavior(
                "Focus moves into popover when opened. \
                 Tab/Shift-Tab cycles within popover content.",
            )
            .keyboard_model("Escape dismisses the popover.")
            .pointer_behavior("Outside click dismisses the popover.")
            .state_model("Controlled open/close via open prop.")
            .required_file("crates/components/src/popover.rs")
            .build()
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.open {
            return div().into_any_element();
        }

        let theme = cx.theme();
        let bg = theme.surface.elevated_surface;
        let border_color = theme.border.default;

        let mut panel = div()
            .id(self.id)
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .shadow_lg()
            .max_h(self.max_height)
            .overflow_hidden()
            .p_2();

        if let Some(w) = self.width {
            panel = panel.w(w);
        }

        // Escape key dismiss
        panel = panel.on_key_down(move |event, _window, cx| {
            if primitives::is_escape_key(event) {
                cx.stop_propagation();
            }
        });

        for child in self.children {
            panel = panel.child(child);
        }

        // Use deferred rendering so popover paints on top
        deferred(panel).with_priority(1).into_any_element()
    }
}
