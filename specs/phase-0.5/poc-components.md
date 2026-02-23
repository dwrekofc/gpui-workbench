# POC Components

## Purpose
Three proof-of-concept styled components — Dialog, Select, and Tabs — that validate the entire stack end-to-end: tokens, primitives, contracts, stories, workbench, registry, and CLI. These are the most interaction-complex Core-12 members and exercise the widest range of primitive behaviors.

## Requirements

### Dialog
- Render as a modal overlay with backdrop
- Implement focus trap: focus stays within dialog while open, returns to trigger on close
- Support Escape key dismissal
- Support outside-click dismissal (click on backdrop)
- Provide title, description, and action slots
- Support controlled open/close state
- Map all surfaces (backdrop, panel, border, text) to frozen design tokens
- Builder-pattern API with `id`, `tooltip`, optional `metadata`
- Pass full Component Acceptance Checklist

### Select
- Render a trigger button that opens a popover dropdown list
- Popover positions relative to trigger, respects viewport boundaries
- Arrow-key navigation through list items
- Enter/Space to select, Escape to close
- Support controlled and uncontrolled selected value
- Support disabled state (blocks interaction, communicates visually)
- Support placeholder text when no value selected
- Map all surfaces to frozen design tokens
- Builder-pattern API with `id`, `tooltip`, optional `metadata`
- Pass full Component Acceptance Checklist

### Tabs
- Render a tab bar with clickable tab triggers and associated content panels
- Left/Right arrow key navigation between tabs in the tab bar
- Tab/Shift-Tab moves focus into/out of the tab bar (not between tabs)
- Active tab state with visual indicator mapped to tokens
- Support controlled and uncontrolled active tab
- Support disabled individual tabs
- Map all surfaces to frozen design tokens
- Builder-pattern API with `id`, `tooltip`, optional `metadata`
- Pass full Component Acceptance Checklist

### All POC Components
- Each component has a fully populated `ComponentContract`
- Each component has story coverage with state matrix (all variants x all states)
- Each component has release-mode performance evidence
- Each component has provenance metadata for any adapted source code
- Disposition recorded: Fork for all three (per adoption matrix)

## Constraints
- Lives in `crates/components/`
- All three are Fork disposition: adapt from reference implementations, normalize to internal contracts/tokens
- Dialog references: Zed `modal.rs` + `alert_modal.rs`, gpui-component `dialog.rs`
- Select references: gpui-component `select.rs`, Zed `dropdown_menu.rs` + `popover.rs`
- Tabs references: Zed `tab.rs` + `tab_bar.rs`, gpui-component `tab/tab.rs` + `tab/tab_bar.rs`
- Must consume primitives from `crates/primitives/` (not inline behavior)
- Must consume tokens from `crates/theme/` (no hard-coded colors)
- Component APIs follow GPUI idioms (builder pattern, `RenderOnce`/`Render` traits)
- Interaction latency within one 60Hz frame (~16.6ms) (NFR-008)

## Acceptance Criteria
1. Dialog renders as modal overlay with focus trap, escape/outside-click dismiss
2. Select renders trigger + popover list with arrow-key nav and enter-to-select
3. Tabs renders tab bar with left/right arrow nav and content panel switching
4. All three components map exclusively to frozen design tokens (no hard-coded colors)
5. All three have populated `ComponentContract` metadata
6. All three have story coverage with state matrix in the workbench
7. All three pass the Component Acceptance Checklist (contract, token, perf, quality gates)
8. All three are installable via `gpui add <component>`
9. `cargo test` passes for all three components
10. Provenance metadata is complete for all adapted source code

## References
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/modal.rs` — Zed modal shell semantics
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/notification/alert_modal.rs` — Zed alert modal
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/dialog.rs` — gpui-component dialog
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/select.rs` — gpui-component select
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/dropdown_menu.rs` — Zed dropdown/select interaction
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/popover.rs` — Zed popover positioning
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tab.rs` — Zed tab component
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tab_bar.rs` — Zed tab bar
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tab/tab.rs` — gpui-component tab
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tab/tab_bar.rs` — gpui-component tab bar

## Decision Rationale
- Decision: Dialog, Select, and Tabs as POC trio
- Why: Together they exercise focus trap, popover, three distinct keyboard models (escape, arrow-list, arrow-group), pointer dismiss, controlled/uncontrolled state, overlay rendering, list rendering, and grouped selection — covering most of the primitive surface area
- Alternatives considered: Button/Input/Checkbox (rejected — too simple, wouldn't validate complex primitives like focus trap and popover)
