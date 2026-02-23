pub mod focus;
pub mod keyboard;
pub mod popover;
pub mod state;

pub use focus::{FocusReturn, FocusTrap};
pub use keyboard::{
    NavDirection, Orientation, classify_nav_key, focus_next, focus_prev, is_activation_key,
    is_escape_key, is_shift_tab, is_tab_key, navigate_index,
};
pub use popover::{PopoverPosition, is_dismiss_key, is_outside_bounds, should_flip_vertical};
pub use state::{
    Controllable, HoverState, InteractionState, OpenState, SelectionState, ValidationState,
};

pub fn init(_cx: &mut gpui::App) {
    // Primitive initialization will register global state/event handlers here.
    // Currently no global state needed — primitives are consumed by components directly.
}
