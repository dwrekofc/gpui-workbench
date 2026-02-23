# Extended Library

## Purpose
All 39 remaining components from the gpui-component inventory beyond the Core-12, completing the full planned component library surface.

## Requirements
- Deliver all of the following components, each passing the Component Acceptance Checklist:
  - `accordion` — collapsible content sections
  - `alert` — contextual feedback messages
  - `animation` — animation utilities and transitions
  - `avatar` — user/entity image with fallback
  - `badge` — status indicator label
  - `breadcrumb` — navigation path display
  - `chart` — data visualization charts
  - `clipboard` — copy-to-clipboard functionality
  - `collapsible` — expandable/collapsible container
  - `color_picker` — color selection widget
  - `description_list` — key-value pair display
  - `divider` — visual separator
  - `dock` — dockable panel system
  - `form` — form layout and validation
  - `group_box` — grouped content container
  - `highlighter` — syntax/text highlighting
  - `history` — undo/redo history management
  - `hover_card` — hover-triggered content card
  - `kbd` — keyboard shortcut display
  - `label` — text label for form elements
  - `link` — clickable navigation link
  - `list` — scrollable item list
  - `pagination` — page navigation controls
  - `plot` — data plotting/graphing
  - `progress` — progress indicator (bar/circular)
  - `rating` — star/score rating input
  - `resizable` — resizable container/panel
  - `scroll` — custom scroll area
  - `setting` — settings/preference controls
  - `sheet` — slide-in overlay panel
  - `sidebar` — navigation sidebar panel
  - `skeleton` — loading placeholder
  - `slider` — range input slider
  - `spinner` — loading spinner indicator
  - `stepper` — multi-step process indicator
  - `switch` — toggle switch
  - `table` — see `priority-components` spec for detailed requirements
  - `tag` — categorization label
  - `text` — styled text display
  - `tree` — see `priority-components` spec for detailed requirements
- Deliver extended variants of Phase 1 modules:
  - `input` advanced behaviors beyond basic Input/Textarea
  - `menu` variants beyond Core-12 DropdownMenu
  - `notification` variants beyond Core-12 Toast
  - `tab` utilities beyond Core-12 Tabs
- Each component has a populated `ComponentContract`
- Each component maps to frozen design tokens
- Each component has story coverage with state matrix
- Each component is installable via CLI
- Extract shared primitives from proven duplication across the full library

## Constraints
- Lives in `crates/components/`
- Phase 1 gate must be met before starting Phase 2
- Apply same Reuse/Fork/Rewrite disposition policy per acceptance checklist results
- Normalize all adopted code to internal contracts/tokens (no raw external patterns)
- Virtualized structures (list, table, tree, scroll) must demonstrate bounded rendering under large datasets
- Provenance metadata required for all adapted code

## Acceptance Criteria
1. All 39 components exist in `crates/components/` and compile
2. All 39 pass the Component Acceptance Checklist
3. All 39 are installable via `gpui add <component>`
4. All 39 have story coverage in the workbench
5. Shared primitives have been extracted where 2+ components share behavior
6. Virtualized components (list, table, tree, scroll) demonstrate bounded rendering
7. `cargo test` passes for all extended library components
8. Provenance metadata complete for all adapted source code

## References
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/lib.rs` — Full module inventory (52 modules total)
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/` — Individual component implementations
- Plan Section 10.2: Phase 2 Full Remaining GPUI Component Library
