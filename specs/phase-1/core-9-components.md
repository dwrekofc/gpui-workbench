# Core-9 Components

## Purpose
The nine remaining Core-12 styled components — Button, Input, Textarea, Checkbox, Radio, Tooltip, DropdownMenu, Popover, Toast — delivered after the Phase 0.5 POC validates the full stack.

## Requirements

### Button
- Support variants: primary, secondary, ghost (and others per contract)
- Support states: hover, active, focused, disabled
- Support icon + label composition
- Builder-pattern API with `id`, `tooltip`, optional `metadata`
- Fork disposition: adapt from Zed `button.rs` with token mapping

### Input
- Single-line text input with focus, selection, cursor management
- Support placeholder text
- Support controlled and uncontrolled value
- Support disabled and readonly states
- Keyboard: standard text editing (select-all, copy, paste, cursor movement)
- Fork disposition: adapt from gpui-component `input.rs` with Zed focus/keyboard alignment

### Textarea
- Multi-line text input extending Input behavior
- Support controlled and uncontrolled value
- Support disabled and readonly states
- Support auto-resize or fixed-height modes
- Fork disposition: adapt from gpui-component multiline model with token mapping

### Checkbox
- Toggle between checked/unchecked (and optionally indeterminate)
- Support controlled and uncontrolled checked state
- Keyboard: Space to toggle, Tab to navigate
- Support disabled state
- Fork disposition: adapt from Zed `toggle.rs` behavior

### Radio
- Radio group with single-selection semantics
- Arrow key navigation within group
- Support controlled and uncontrolled selected value
- Support disabled individual items and entire group
- Fork disposition: adapt from gpui-component `radio.rs` with checklist-driven keyboard model

### Tooltip
- Display on hover/focus after a brief delay
- Position relative to trigger element
- Dismiss on pointer leave or Escape
- Reuse disposition (with token map): adopt if checklist passes with only token remapping

### DropdownMenu
- Trigger opens a dropdown menu with items
- Arrow key navigation through items
- Enter/Space to select item, Escape to close
- Support nested submenus [inferred]
- Support disabled menu items
- Fork disposition: adapt from Zed `dropdown_menu.rs` with gpui-component convenience API

### Popover
- Anchor-relative popup panel for arbitrary content
- Viewport-aware positioning
- Outside-click and Escape dismissal
- Support controlled open/close state
- Fork disposition: adapt from Zed/gpui-component with token consistency

### Toast
- Notification popup with auto-dismiss timer
- Support action buttons within toast
- Support multiple concurrent toasts (stacking)
- Support variants: info, success, warning, error [inferred]
- Dismiss on click or after timeout
- Fork disposition: adapt from gpui-component `notification.rs` with Zed visual semantics

### All Core-9 Components
- Each passes the full Component Acceptance Checklist
- Each has a populated `ComponentContract`
- Each maps exclusively to frozen design tokens
- Each has story coverage with state matrix in the workbench
- Each has release-mode performance evidence
- Each has provenance metadata for adapted source code
- Extract additional primitives only when 2+ components share behavior (FR-011)

## Constraints
- Lives in `crates/components/`
- Must consume primitives from `crates/primitives/` where applicable
- Must consume tokens from `crates/theme/`
- Component APIs follow GPUI idioms (builder pattern, `RenderOnce`/`Render` traits)
- Interaction latency within one 60Hz frame (~16.6ms) (NFR-008)
- All dispositions are Fork except Tooltip which is Reuse (with token map)
- Phase 0.5 gate must be met before starting these components

## Acceptance Criteria
1. All 9 components render correctly in the workbench with stories
2. All 9 pass the Component Acceptance Checklist (contract, token, perf, quality gates)
3. All 9 are installable via `gpui add <component>`
4. All 9 have populated `ComponentContract` metadata
5. All 9 have state matrix story coverage
6. All 9 have release-mode performance evidence
7. Combined with POC trio, all 12 Core components are installable and validated
8. `cargo test` passes for all 9 components
9. Provenance metadata complete for all adapted source code

## References
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/button.rs` — Zed button
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/button/button.rs` — gpui-component button
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui_input/src/input_field.rs` — Zed input field
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/input/input.rs` — gpui-component input
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/input/state.rs` — gpui-component input/textarea state
- Reference: `.refs/zed_gpui_refs/zed-main/crates/gpui/examples/input.rs` — GPUI input example
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/toggle.rs` — Zed toggle (checkbox/radio base)
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/checkbox.rs` — gpui-component checkbox
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/radio.rs` — gpui-component radio
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tooltip.rs` — Zed tooltip
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tooltip.rs` — gpui-component tooltip
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/dropdown_menu.rs` — Zed dropdown menu
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/menu/dropdown_menu.rs` — gpui-component dropdown menu
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/menu/popup_menu.rs` — gpui-component popup menu
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/popover.rs` — Zed popover
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/popover.rs` — gpui-component popover
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/notification/announcement_toast.rs` — Zed toast
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/notification.rs` — gpui-component notification/toast
- Plan Section 10.1: Core-12 Adoption Matrix
