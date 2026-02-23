# Primitives

## Purpose
Unstyled behavior engines that encapsulate shared focus, keyboard, popover, and state management logic consumed by styled components. Primitives provide the interaction contract layer between raw GPUI events and component behavior.

## Requirements
- Provide a focus management primitive:
  - Predictable focus entry and exit
  - Focus trap for modal contexts (Dialog)
  - Focus return on dismiss
  - Focus ring/indicator support via tokens
- Provide keyboard model primitives:
  - Tab/Shift-Tab for focus navigation
  - Enter/Space for activation
  - Arrow key navigation for lists (Select) and groups (Tabs)
  - Escape for dismissal (Dialog, Select popover)
  - Shortcut handling that is explicit and testable
- Provide a popover/overlay positioning primitive:
  - Anchor-relative positioning
  - Viewport boundary awareness
  - Outside-click dismissal
  - Escape dismissal
- Provide a state management primitive:
  - Controlled vs uncontrolled behavior pattern
  - State types: open/closed, selected/unselected, active/inactive, disabled, error, readonly
  - State transition hooks for component consumers
- Provide pointer behavior primitives:
  - Click, hover, drag semantics
  - Outside-click detection and dismissal
- Extract primitives only when at least two components share the behavior (FR-011)
- For Phase 0.5, implement only the primitives needed by Dialog, Select, and Tabs

## Constraints
- Lives in `crates/primitives/`
- Primitives are unstyled — they provide behavior, not visual presentation
- Must follow GPUI event model and rendering patterns (not web/React event system)
- Primary interaction reference: Radix/Base UI behavior patterns, translated to GPUI desktop semantics
- Primary correctness reference: Zed component interaction behavior
- Do not blindly copy web API shapes — translate interaction semantics to desktop conventions
- Disabled state blocks all interaction and communicates state visually
- Readonly allows focus where appropriate but blocks mutation

## Acceptance Criteria
1. Focus management primitive exists and supports trap, return, and entry/exit
2. Keyboard primitives handle Tab, Enter/Space, Arrow, and Escape for POC components
3. Popover positioning primitive anchors correctly and respects viewport boundaries
4. State management supports controlled and uncontrolled patterns
5. Outside-click dismissal works for Dialog and Select
6. All primitives are used by at least one POC component (Dialog, Select, or Tabs)
7. `cargo test -p primitives` passes (or equivalent test target)

## References
- Reference: `.refs/zed_gpui_refs/primitives/packages/react/progress/src/progress.tsx` — Radix primitive layering pattern
- Reference: `.refs/zed_gpui_refs/base-ui/packages/react/src/switch/root/SwitchRoot.tsx` — Base UI controlled/uncontrolled state pattern
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/modal.rs` — Zed modal focus trap behavior
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/popover.rs` — Zed popover positioning and dismiss
- Reference: `.refs/zed_gpui_refs/zed-main/crates/gpui/README.md` — GPUI event model and rendering architecture

## Decision Rationale
- Decision: Separate unstyled primitives crate consumed by styled components
- Why: Radix/Base UI layering model is proven; separating behavior from styling allows reuse and independent testing
- Alternatives considered: Inline behavior in each component (rejected — leads to duplication and inconsistent interaction patterns)
- Decision: Extract only when 2+ components share behavior
- Why: Prevents premature abstraction; primitives earn their existence through proven duplication
