//! Tooltip component: hover-triggered contextual text overlay.
//!
//! Reuse disposition: adopted with token remapping to internal token system.
//! The Tooltip is the simplest component -- a positioned text overlay that
//! appears on hover and disappears on mouse leave.
//!
//! Provenance:
//! - Zed `crates/ui/src/components/tooltip.rs` (GPL-3.0/AGPL-3.0, Zed Industries)
//! - gpui-component tooltip patterns (MIT, Zed Industries)
//! - Modifications: Simplified to internal token system, reuse disposition
//!   with only color remapping.

use gpui::*;
use theme::ActiveTheme;

/// Tooltip placement relative to the trigger element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPlacement {
    /// Above the trigger.
    Top,
    /// Below the trigger (default).
    #[default]
    Bottom,
    /// To the left of the trigger.
    Left,
    /// To the right of the trigger.
    Right,
}

/// A tooltip overlay that appears on hover with contextual text.
///
/// # Usage
/// ```ignore
/// Tooltip::new("my-tooltip")
///     .text("Save your work")
///     .placement(TooltipPlacement::Top)
/// ```
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    text: SharedString,
    placement: TooltipPlacement,
    max_width: Pixels,
}

impl Tooltip {
    /// Create a new tooltip with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            text: SharedString::default(),
            placement: TooltipPlacement::Bottom,
            max_width: px(250.0),
        }
    }

    /// Set the tooltip text content.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }

    /// Set the tooltip placement relative to the trigger.
    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Set the maximum width of the tooltip.
    pub fn max_width(mut self, width: Pixels) -> Self {
        self.max_width = width;
        self
    }

    /// Returns the component contract for Tooltip.
    pub fn contract() -> crate::ComponentContract {
        use crate::*;
        ComponentContract::builder("Tooltip", "0.1.0")
            .disposition(Disposition::Reuse)
            .required_prop("id", "ElementId", "Unique identifier for the tooltip")
            .optional_prop("text", "SharedString", "\"\"", "Tooltip text content")
            .optional_prop(
                "placement",
                "TooltipPlacement",
                "Bottom",
                "Placement relative to trigger: Top, Bottom, Left, Right",
            )
            .optional_prop(
                "max_width",
                "Pixels",
                "250.0",
                "Maximum width of the tooltip",
            )
            .state(ComponentState::Hover)
            .token_dep("surface.elevated_surface", "Tooltip background")
            .token_dep("border.default", "Tooltip border")
            .token_dep("text.default", "Tooltip text color")
            .focus_behavior("Tooltips are not focusable. They appear on hover only.")
            .keyboard_model("No keyboard interaction. Tooltip hides when trigger loses focus.")
            .pointer_behavior("Appears on hover over trigger, disappears on mouse leave.")
            .state_model("Visibility controlled by hover state of the trigger element.")
            .required_file("crates/components/src/tooltip.rs")
            .build()
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let bg = theme.surface.elevated_surface;
        let border_color = theme.border.default;
        let text_color = theme.text.default;

        div()
            .id(self.id)
            .max_w(self.max_width)
            .px_2()
            .py_1()
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .shadow_md()
            .text_xs()
            .text_color(text_color)
            .child(self.text)
    }
}
