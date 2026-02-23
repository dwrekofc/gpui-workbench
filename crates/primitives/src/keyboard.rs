//! Keyboard navigation primitive: standard key handlers for component interaction.
//!
//! Provides shared keyboard behavior patterns:
//! - Tab/Shift-Tab for focus navigation
//! - Enter/Space for activation
//! - Arrow keys for list/group navigation
//! - Escape for dismissal
//!
//! Components consume these patterns rather than re-implementing key handling.

use gpui::{KeyDownEvent, Window};

/// Standard key identifiers used across components.
pub mod keys {
    pub const TAB: &str = "tab";
    pub const ENTER: &str = "enter";
    pub const SPACE: &str = "space";
    pub const ESCAPE: &str = "escape";
    pub const ARROW_UP: &str = "up";
    pub const ARROW_DOWN: &str = "down";
    pub const ARROW_LEFT: &str = "left";
    pub const ARROW_RIGHT: &str = "right";
    pub const HOME: &str = "home";
    pub const END: &str = "end";
}

/// Direction for arrow key navigation within a list or group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
    /// Previous item (Up arrow in vertical lists, Left in horizontal tabs).
    Previous,
    /// Next item (Down arrow in vertical lists, Right in horizontal tabs).
    Next,
    /// Jump to first item (Home key).
    First,
    /// Jump to last item (End key).
    Last,
}

/// Orientation of the navigable container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Vertical list (Select dropdown): Up/Down for navigation.
    Vertical,
    /// Horizontal group (Tabs): Left/Right for navigation.
    Horizontal,
}

/// Classify a key event into a navigation direction based on the container's orientation.
///
/// Returns `None` if the key event is not a navigation key for the given orientation.
pub fn classify_nav_key(event: &KeyDownEvent, orientation: Orientation) -> Option<NavDirection> {
    let key = event.keystroke.key.as_str();
    match orientation {
        Orientation::Vertical => match key {
            keys::ARROW_UP => Some(NavDirection::Previous),
            keys::ARROW_DOWN => Some(NavDirection::Next),
            keys::HOME => Some(NavDirection::First),
            keys::END => Some(NavDirection::Last),
            _ => None,
        },
        Orientation::Horizontal => match key {
            keys::ARROW_LEFT => Some(NavDirection::Previous),
            keys::ARROW_RIGHT => Some(NavDirection::Next),
            keys::HOME => Some(NavDirection::First),
            keys::END => Some(NavDirection::Last),
            _ => None,
        },
    }
}

/// Check if a key event is an activation key (Enter or Space).
pub fn is_activation_key(event: &KeyDownEvent) -> bool {
    let key = event.keystroke.key.as_str();
    key == keys::ENTER || key == keys::SPACE
}

/// Check if a key event is the Escape key.
pub fn is_escape_key(event: &KeyDownEvent) -> bool {
    event.keystroke.key.as_str() == keys::ESCAPE
}

/// Check if a key event is Tab (with or without Shift).
pub fn is_tab_key(event: &KeyDownEvent) -> bool {
    event.keystroke.key.as_str() == keys::TAB
}

/// Check if Tab has the Shift modifier (for reverse tab navigation).
pub fn is_shift_tab(event: &KeyDownEvent) -> bool {
    event.keystroke.key.as_str() == keys::TAB && event.keystroke.modifiers.shift
}

/// Advance focus to the next focusable element in the window.
pub fn focus_next(window: &mut Window, cx: &mut gpui::App) {
    window.focus_next(cx);
}

/// Move focus to the previous focusable element in the window.
pub fn focus_prev(window: &mut Window, cx: &mut gpui::App) {
    window.focus_prev(cx);
}

/// Compute the next index in a list given a navigation direction.
///
/// Wraps around: going Previous from index 0 wraps to `count - 1`,
/// going Next from the last item wraps to 0. Skips items where
/// `is_disabled` returns true (up to `count` attempts to avoid infinite loop).
pub fn navigate_index(
    current: usize,
    direction: NavDirection,
    count: usize,
    is_disabled: impl Fn(usize) -> bool,
) -> usize {
    if count == 0 {
        return 0;
    }

    let start = match direction {
        NavDirection::Previous => {
            if current == 0 {
                count - 1
            } else {
                current - 1
            }
        }
        NavDirection::Next => {
            if current >= count - 1 {
                0
            } else {
                current + 1
            }
        }
        NavDirection::First => 0,
        NavDirection::Last => count - 1,
    };

    // Skip disabled items
    let step: isize = match direction {
        NavDirection::Previous | NavDirection::Last => -1,
        NavDirection::Next | NavDirection::First => 1,
    };

    let mut idx = start;
    for _ in 0..count {
        if !is_disabled(idx) {
            return idx;
        }
        idx = ((idx as isize + step).rem_euclid(count as isize)) as usize;
    }

    // All items disabled, return current
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigate_next_wraps() {
        assert_eq!(navigate_index(4, NavDirection::Next, 5, |_| false), 0);
        assert_eq!(navigate_index(2, NavDirection::Next, 5, |_| false), 3);
    }

    #[test]
    fn navigate_prev_wraps() {
        assert_eq!(navigate_index(0, NavDirection::Previous, 5, |_| false), 4);
        assert_eq!(navigate_index(3, NavDirection::Previous, 5, |_| false), 2);
    }

    #[test]
    fn navigate_first_last() {
        assert_eq!(navigate_index(3, NavDirection::First, 5, |_| false), 0);
        assert_eq!(navigate_index(1, NavDirection::Last, 5, |_| false), 4);
    }

    #[test]
    fn navigate_skips_disabled() {
        // Items 0, 2 are disabled
        let disabled = |i: usize| i == 0 || i == 2;
        assert_eq!(navigate_index(1, NavDirection::Next, 5, disabled), 3);
        assert_eq!(navigate_index(1, NavDirection::Previous, 5, disabled), 4);
        assert_eq!(navigate_index(3, NavDirection::First, 5, disabled), 1);
        assert_eq!(navigate_index(1, NavDirection::Last, 5, disabled), 4);
    }

    #[test]
    fn navigate_all_disabled_returns_current() {
        assert_eq!(navigate_index(2, NavDirection::Next, 5, |_| true), 2);
    }

    #[test]
    fn navigate_empty_list() {
        assert_eq!(navigate_index(0, NavDirection::Next, 0, |_| false), 0);
    }

    #[test]
    fn navigate_single_item() {
        assert_eq!(navigate_index(0, NavDirection::Next, 1, |_| false), 0);
        assert_eq!(navigate_index(0, NavDirection::Previous, 1, |_| false), 0);
    }
}
