# Priority Components

## Purpose
Three high-priority, Zed-aligned components — Table, Tree, and Command Palette — that require special attention due to performance requirements, complex interaction models, and strategic importance.

## Requirements

### Table
- Virtualized table rendering with bounded memory/rendering under large datasets
- Column headers with sort indicators
- Row selection (single and multi-select)
- Keyboard navigation: arrow keys between cells/rows, Tab between focusable elements
- Support controlled and uncontrolled selection state
- Release-mode performance evidence with large dataset benchmarks
- Fork disposition: adapt from gpui-component `table/mod.rs` with Zed perf gates

### Tree
- Hierarchical tree structure with expand/collapse nodes
- Keyboard navigation: arrow up/down between items, arrow right to expand, arrow left to collapse
- Support selection (single-select)
- Virtualized rendering for large trees
- Support drag-and-drop reordering [inferred]
- Fork disposition: adapt using Zed tree interaction semantics with gpui-component structural helpers

### Command Palette
- Full-text fuzzy search over registered commands
- Keyboard-first interaction: open with shortcut, type to filter, arrow navigate results, Enter to execute
- Display command names with keyboard shortcut hints
- Support command categories/grouping [inferred]
- Virtualized result list for large command sets
- Rewrite disposition: Zed implementation is too app-specific; implement local version using same architectural pattern

### All Priority Components
- Each passes the full Component Acceptance Checklist
- Each has populated `ComponentContract`
- Each maps to frozen design tokens
- Each has story coverage with state matrix
- Each has release-mode performance evidence specifically demonstrating bounded rendering

## Constraints
- Lives in `crates/components/`
- Table and Tree must demonstrate bounded rendering: frame budget stays within 16.6ms regardless of dataset size
- Command Palette is a rewrite, not a fork — Zed's version is too coupled to editor internals
- All three are performance-critical: require explicit perf gate evidence before acceptance
- Virtualization must use GPUI's rendering primitives (uniform_list or equivalent)

## Acceptance Criteria
1. Table renders 10,000+ rows without frame budget regression
2. Tree renders 10,000+ nodes without frame budget regression
3. Command Palette searches 1,000+ commands with sub-frame filtering latency
4. All three have keyboard navigation matching their specified models
5. All three pass Component Acceptance Checklist including performance gates
6. All three are installable via `gpui add <component>`
7. All three have story coverage with state matrix
8. `cargo test` passes for all three

## References
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/data_table.rs` — Zed data table
- Reference: `.refs/zed_gpui_refs/zed-main/crates/gpui/src/elements/uniform_list.rs` — GPUI uniform list (virtualization primitive)
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/table/mod.rs` — gpui-component table
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/virtual_list.rs` — gpui-component virtual list
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tree_view_item.rs` — Zed tree view
- Reference: `.refs/zed_gpui_refs/zed-main/crates/gpui/examples/tree.rs` — GPUI tree example
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tree.rs` — gpui-component tree
- Reference: `.refs/zed_gpui_refs/zed-main/crates/command_palette/src/command_palette.rs` — Zed command palette (architectural pattern reference)
- Plan Section 10.3: Phase 2 Priority Mapping

## Decision Rationale
- Decision: Command Palette is a rewrite, not a fork
- Why: Zed's command palette is deeply coupled to editor-specific command registration; a local version using the same pattern but decoupled from Zed internals is cleaner
- Decision: Explicit large-dataset perf benchmarks required
- Why: Table, Tree, and Command Palette are the components most likely to hit performance walls; proving bounded rendering before acceptance prevents regressions
