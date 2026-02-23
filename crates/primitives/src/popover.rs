//! Popover/overlay positioning primitive.
//!
//! Provides anchor-relative positioning with viewport boundary awareness.
//! GPUI's built-in `deferred()` + `anchored()` elements handle the actual
//! overlay rendering and edge-avoidance. This module provides the higher-level
//! positioning intent types and dismiss behavior coordination.

use gpui::{Corner, KeyDownEvent, Pixels, Point};

/// Specifies where a popover should be positioned relative to its trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopoverPosition {
    /// Which corner of the trigger element the popover anchors to.
    pub anchor: Corner,
    /// Which corner of the popover content attaches to the anchor point.
    pub attach: Corner,
}

impl PopoverPosition {
    /// Popover opens below the trigger, aligned to the left edge.
    /// Used by Select dropdown.
    pub fn below_left() -> Self {
        Self {
            anchor: Corner::BottomLeft,
            attach: Corner::TopLeft,
        }
    }

    /// Popover opens below the trigger, aligned to the right edge.
    pub fn below_right() -> Self {
        Self {
            anchor: Corner::BottomRight,
            attach: Corner::TopRight,
        }
    }

    /// Popover opens above the trigger, aligned to the left edge.
    /// Used when viewport constrains downward opening.
    pub fn above_left() -> Self {
        Self {
            anchor: Corner::TopLeft,
            attach: Corner::BottomLeft,
        }
    }

    /// Popover opens above the trigger, aligned to the right edge.
    pub fn above_right() -> Self {
        Self {
            anchor: Corner::TopRight,
            attach: Corner::BottomRight,
        }
    }
}

impl Default for PopoverPosition {
    fn default() -> Self {
        Self::below_left()
    }
}

/// Check whether a click point is outside a given rectangular bounds.
///
/// Used for outside-click dismissal of popovers and dialogs.
/// The bounds represent the popover/dialog content area.
pub fn is_outside_bounds(
    click_point: Point<Pixels>,
    bounds_origin: Point<Pixels>,
    bounds_width: Pixels,
    bounds_height: Pixels,
) -> bool {
    click_point.x < bounds_origin.x
        || click_point.x > bounds_origin.x + bounds_width
        || click_point.y < bounds_origin.y
        || click_point.y > bounds_origin.y + bounds_height
}

/// Determine if a popover should flip its vertical position based on available space.
///
/// Returns `true` if the popover should open upward instead of downward.
/// `trigger_y`: Y position of the trigger element.
/// `popover_height`: Height of the popover content.
/// `viewport_height`: Total viewport height.
pub fn should_flip_vertical(
    trigger_y: Pixels,
    trigger_height: Pixels,
    popover_height: Pixels,
    viewport_height: Pixels,
) -> bool {
    let space_below = viewport_height - (trigger_y + trigger_height);
    let space_above = trigger_y;
    space_below < popover_height && space_above > space_below
}

/// Check if a key event should dismiss the popover (Escape key).
pub fn is_dismiss_key(event: &KeyDownEvent) -> bool {
    event.keystroke.key.as_str() == super::keyboard::keys::ESCAPE
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::px;

    #[test]
    fn default_position_is_below_left() {
        let pos = PopoverPosition::default();
        assert_eq!(pos, PopoverPosition::below_left());
    }

    #[test]
    fn outside_bounds_detection() {
        let origin = Point::new(px(100.0), px(100.0));
        let width = px(200.0);
        let height = px(150.0);

        // Inside
        assert!(!is_outside_bounds(
            Point::new(px(150.0), px(150.0)),
            origin,
            width,
            height
        ));

        // Outside left
        assert!(is_outside_bounds(
            Point::new(px(50.0), px(150.0)),
            origin,
            width,
            height
        ));

        // Outside right
        assert!(is_outside_bounds(
            Point::new(px(350.0), px(150.0)),
            origin,
            width,
            height
        ));

        // Outside top
        assert!(is_outside_bounds(
            Point::new(px(150.0), px(50.0)),
            origin,
            width,
            height
        ));

        // Outside bottom
        assert!(is_outside_bounds(
            Point::new(px(150.0), px(300.0)),
            origin,
            width,
            height
        ));
    }

    #[test]
    fn flip_when_no_space_below() {
        // Trigger near bottom of viewport
        assert!(should_flip_vertical(
            px(800.0),
            px(40.0),
            px(300.0),
            px(1000.0)
        ));
    }

    #[test]
    fn no_flip_when_space_below() {
        // Trigger near top of viewport
        assert!(!should_flip_vertical(
            px(100.0),
            px(40.0),
            px(300.0),
            px(1000.0)
        ));
    }
}
