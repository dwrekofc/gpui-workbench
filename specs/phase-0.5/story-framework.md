# Story Framework

## Purpose
A trait-based story system that registers components for rendering in the workbench with state matrix support, enabling systematic visual validation of every variant and state combination.

## Requirements
- Define a `Story` trait that components implement to provide workbench rendering
- Support state matrix rendering: display all combinations of variants x states for a component
- Provide a story registration mechanism so the workbench discovers available stories
- Every CLI-installable component must have a corresponding story (FR-007)
- Stories must render components in isolation (no inter-component dependencies in story context)
- Stories must demonstrate: default state, all variants, hover, active, focused, disabled, error states
- Support rendering stories with the current theme (tokens resolved from theme engine)

## Constraints
- Lives in `crates/story/`
- Story trait must be lightweight — minimal boilerplate per component
- Stories are for validation, not production UI
- Must integrate with the workbench app's rendering pipeline
- State matrix should be generated from component contract metadata where possible

## Acceptance Criteria
1. `Story` trait is defined in `crates/story/`
2. Story registration mechanism exists and the workbench discovers registered stories
3. All 3 POC components (Dialog, Select, Tabs) have story implementations
4. State matrix rendering shows all variant x state combinations for at least one component
5. Stories render with the current theme and update when theme switches
6. Adding a new story requires only implementing the trait (no manual wiring beyond registration)

## References
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/` — Zed component patterns (stories are inline in Zed's storybook)
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/` — gpui-component structure (stories alongside components)

## Decision Rationale
- Decision: Trait-based story system with registration
- Why: Rust idiomatic, allows compile-time verification, and keeps stories co-located with components
- Alternatives considered: Macro-based auto-discovery (possible future optimization, but trait-first is simpler and more explicit)
