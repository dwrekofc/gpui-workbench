# Component Library

## Purpose
The collection of styled, token-driven UI components distributed as source-copy installations, starting with the Core-12 in Phase 1 and expanding to the full inventory in Phase 2.

## Requirements
- Deliver Core-12 components in Phase 1: Button, Input, Textarea, Select, Checkbox, Radio, Dialog, Tooltip, Tabs, DropdownMenu, Popover, Toast (FR-010)
- Every component passes the Component Acceptance Checklist before admission (FR-014)
- Stateful components document and test controlled vs uncontrolled behavior (FR-017)
- Components include explicit keyboard semantics: Tab, Enter/Space, arrow navigation, Escape dismissal where applicable (FR-018)
- Extract shared primitives only when at least two components share behavior (FR-011)
- Support builder-pattern composition for all component APIs
- Provide shared identifiers on all components: `id`, `tooltip`, optional `metadata` map
- Map all component colors/surfaces/states to frozen design tokens
- Prohibit hard-coded colors outside approved token exceptions
- Record disposition (Reuse/Fork/Rewrite) for each component per the adoption matrix
- All Core-12 components use `#[derive(IntoElement)]` + `impl RenderOnce` pattern [observed from code]
- All components resolve colors from `cx.theme()` at render time [observed from code]
- All components provide a static `contract()` method returning `ComponentContract` [observed from code]
- All 12 Core-12 components are stateless `RenderOnce` structs; interactive state held by parent [observed from code]

### Phase 1 Adoption Matrix
| Component | Disposition | Primary Reference |
|-----------|------------|-------------------|
| Button | Fork | Zed + gpui-component |
| Input | Fork | gpui-component (richer editing), Zed (focus/keyboard) |
| Textarea | Fork | gpui-component (multiline model) |
| Select | Fork | gpui-component, normalize to Radix/Base contract |
| Checkbox | Fork | Zed toggle behavior |
| Radio | Fork | gpui-component, checklist-driven group keyboard model |
| Dialog | Fork | Zed modal shell + reusable dialog API |
| Tooltip | Reuse (with token map) | Fastest path if checklist passes |
| Tabs | Fork | Zed ergonomics + gpui-component breadth |
| DropdownMenu | Fork | Zed menu interaction model |
| Popover | Fork | Enforce dismiss/focus contracts |
| Toast | Fork | gpui-component general-purpose + Zed visual semantics |

### Phase 2 Scope
- All 39 remaining modules from `gpui-component-main` (`accordion` through `tree`)
- Priority items: Table (Fork), Tree (Fork), Command Palette (Rewrite, Zed-patterned)

## Constraints
- Styled components live in `crates/components/`
- Default distribution: source-copy installation into app codebases (shadcn-style)
- Optional: crate-based consumption for centralized updates
- Components must follow GPUI idioms (`RenderOnce`/`Render` traits, builder pattern)
- Performance gates: release-mode measurements, no unapproved regressions (NFR-007)
- Interaction latency: primary feedback within one 60Hz frame (~16.6ms) (NFR-008)

## Acceptance Criteria
1. All 12 Core-12 components exist in `crates/components/` with full implementations
2. Each component has a static `contract()` returning a valid `ComponentContract`
3. Each component's disposition (Fork/Reuse/Rewrite) is recorded
4. No component uses hard-coded colors — all resolve from theme tokens
5. All Core-12 are installable via CLI, rendered in workbench, and have performance evidence (AC-004, AC-008)
6. Phase 2 completes the full remaining inventory (AC-009)

## References
- Reqs Section 10.1: Phase 1 Core-12 Adoption Matrix
- Reqs Section 10.2: Phase 2 Full Remaining Library
- Reqs Section 10.3: Phase 2 Priority Mapping
- Reqs Section 5: Component Acceptance Checklist
- Reqs Section 8: FR-010, FR-011, FR-014, FR-017, FR-018
