# Primitives

## Purpose
Unstyled behavior building blocks that encapsulate reusable interaction patterns (focus, keyboard navigation, state management, popover positioning) shared across multiple components.

## Requirements
- Provide keyboard classification: map arrow keys to navigation directions based on orientation [observed from code]
- Provide index-based navigation with wrapping and disabled-item skipping [observed from code]
- Provide single-purpose key predicates: activation key, escape key, tab key, shift-tab [observed from code]
- Provide `Controllable<T>` enum modeling parent-owned vs self-owned state [observed from code]
- Provide `OpenState` with toggle/open/close semantics [observed from code]
- Provide `SelectionState`, `InteractionState` (Enabled/Disabled/Readonly), `HoverState`, `ValidationState` [observed from code]
- Provide `FocusReturn` for capturing and restoring focus on dismiss [observed from code]
- Provide `FocusTrap` wrapping a `FocusHandle` with containment queries [observed from code]
- Provide `PopoverPosition` with anchor/attach corners and viewport-aware flipping [observed from code]
- Provide `is_outside_bounds()` for outside-click dismiss detection [observed from code]
- Extract shared primitives only when at least two components share the behavior (FR-011)
- `InteractionState::Disabled` blocks interaction; `Readonly` allows focus but blocks mutation [observed from code]

## Constraints
- Lives in `crates/primitives/`
- Must not depend on `components` or `theme` — primitives are unstyled
- Primitives are behavior engines, not visual components
- New primitives are extracted from proven duplication, not speculatively created (FR-011)

## Acceptance Criteria
1. Keyboard navigation primitives exist and are used by at least Tabs, Select, DropdownMenu, and Radio
2. `Controllable<T>` enum exists with `Controlled` and `Uncontrolled` variants
3. State enums (`OpenState`, `SelectionState`, `InteractionState`, `HoverState`, `ValidationState`) exist
4. `FocusReturn` and `FocusTrap` exist and are used by Dialog and Select
5. `PopoverPosition` and `is_outside_bounds()` exist and are used by Popover, Select, DropdownMenu
6. No primitive depends on the `components` or `theme` crate

## References
- Reference: `.refs/zed_gpui_refs/base-ui/` — Base UI interaction primitive patterns
- Reference: `.refs/zed_gpui_refs/primitives/` — Radix primitive patterns
- Reqs Section 4 step 3: Primitive contract checklist
- Reqs Section 7.1: `crates/primitives/` in monorepo layout
- Reqs Section 8: FR-011
