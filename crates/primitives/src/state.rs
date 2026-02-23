//! State management primitive: controlled vs uncontrolled behavior patterns.
//!
//! Components can be either:
//! - **Controlled**: Parent owns the state and passes it down. Component calls
//!   `on_change` to request state changes but doesn't modify state internally.
//! - **Uncontrolled**: Component owns its own state internally. Parent can set
//!   an initial value via `default_value` but doesn't control ongoing state.
//!
//! This module provides the `Controllable<T>` type that encapsulates this pattern,
//! and standard state type definitions used across components.

/// Represents a value that can be either controlled (externally owned) or
/// uncontrolled (internally owned with a default).
#[derive(Debug, Clone)]
pub enum Controllable<T> {
    /// Parent controls the value. The component should render this value
    /// and call `on_change` to request changes.
    Controlled(T),
    /// Component controls its own value. The initial value is provided
    /// as the default; the component maintains state internally.
    Uncontrolled(T),
}

impl<T> Controllable<T> {
    /// Get the current value regardless of control mode.
    pub fn value(&self) -> &T {
        match self {
            Controllable::Controlled(v) | Controllable::Uncontrolled(v) => v,
        }
    }

    /// Returns true if the value is externally controlled.
    pub fn is_controlled(&self) -> bool {
        matches!(self, Controllable::Controlled(_))
    }

    /// Returns true if the value is internally managed.
    pub fn is_uncontrolled(&self) -> bool {
        matches!(self, Controllable::Uncontrolled(_))
    }
}

impl<T: Default> Default for Controllable<T> {
    fn default() -> Self {
        Controllable::Uncontrolled(T::default())
    }
}

/// Standard open/closed toggle state.
///
/// Used by Dialog (open/closed), Select (popover open/closed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OpenState {
    #[default]
    Closed,
    Open,
}

impl OpenState {
    pub fn is_open(&self) -> bool {
        *self == OpenState::Open
    }

    pub fn is_closed(&self) -> bool {
        *self == OpenState::Closed
    }

    pub fn toggle(&mut self) {
        *self = match self {
            OpenState::Open => OpenState::Closed,
            OpenState::Closed => OpenState::Open,
        };
    }

    pub fn open(&mut self) {
        *self = OpenState::Open;
    }

    pub fn close(&mut self) {
        *self = OpenState::Closed;
    }
}

/// Standard selection state.
///
/// Used by Select (selected item), Tabs (active tab).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionState {
    #[default]
    Unselected,
    Selected,
}

impl SelectionState {
    pub fn is_selected(&self) -> bool {
        *self == SelectionState::Selected
    }

    pub fn toggle(&mut self) {
        *self = match self {
            SelectionState::Selected => SelectionState::Unselected,
            SelectionState::Unselected => SelectionState::Selected,
        };
    }
}

/// Interaction capability state.
///
/// Disabled: blocks all interaction, receives no focus or events.
/// Readonly: allows focus and reading, blocks mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InteractionState {
    #[default]
    Enabled,
    Disabled,
    Readonly,
}

impl InteractionState {
    pub fn is_enabled(&self) -> bool {
        *self == InteractionState::Enabled
    }

    pub fn is_disabled(&self) -> bool {
        *self == InteractionState::Disabled
    }

    pub fn is_readonly(&self) -> bool {
        *self == InteractionState::Readonly
    }

    /// Returns true if the element can be interacted with (enabled or readonly).
    pub fn is_focusable(&self) -> bool {
        !self.is_disabled()
    }

    /// Returns true if the element can be mutated (only when enabled).
    pub fn is_mutable(&self) -> bool {
        self.is_enabled()
    }
}

/// Visual hover state for pointer interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HoverState {
    #[default]
    Idle,
    Hovered,
}

impl HoverState {
    pub fn is_hovered(&self) -> bool {
        *self == HoverState::Hovered
    }
}

/// Validation state for form-like components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationState {
    #[default]
    None,
    Error,
}

impl ValidationState {
    pub fn is_error(&self) -> bool {
        *self == ValidationState::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controllable_controlled() {
        let c = Controllable::Controlled(42);
        assert!(c.is_controlled());
        assert!(!c.is_uncontrolled());
        assert_eq!(*c.value(), 42);
    }

    #[test]
    fn controllable_uncontrolled() {
        let c = Controllable::Uncontrolled("hello");
        assert!(c.is_uncontrolled());
        assert_eq!(*c.value(), "hello");
    }

    #[test]
    fn controllable_default_is_uncontrolled() {
        let c: Controllable<i32> = Controllable::default();
        assert!(c.is_uncontrolled());
        assert_eq!(*c.value(), 0);
    }

    #[test]
    fn open_state_toggle() {
        let mut s = OpenState::Closed;
        assert!(s.is_closed());
        s.toggle();
        assert!(s.is_open());
        s.toggle();
        assert!(s.is_closed());
    }

    #[test]
    fn open_state_explicit() {
        let mut s = OpenState::default();
        s.open();
        assert!(s.is_open());
        s.close();
        assert!(s.is_closed());
    }

    #[test]
    fn selection_state_toggle() {
        let mut s = SelectionState::Unselected;
        s.toggle();
        assert!(s.is_selected());
        s.toggle();
        assert!(!s.is_selected());
    }

    #[test]
    fn interaction_state_capabilities() {
        let enabled = InteractionState::Enabled;
        assert!(enabled.is_focusable());
        assert!(enabled.is_mutable());

        let disabled = InteractionState::Disabled;
        assert!(!disabled.is_focusable());
        assert!(!disabled.is_mutable());

        let readonly = InteractionState::Readonly;
        assert!(readonly.is_focusable());
        assert!(!readonly.is_mutable());
    }

    #[test]
    fn hover_state() {
        let idle = HoverState::Idle;
        assert!(!idle.is_hovered());
        let hovered = HoverState::Hovered;
        assert!(hovered.is_hovered());
    }

    #[test]
    fn validation_state() {
        let none = ValidationState::None;
        assert!(!none.is_error());
        let error = ValidationState::Error;
        assert!(error.is_error());
    }
}
