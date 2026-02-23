# Component Contracts

## Purpose
The metadata schema and acceptance checklist that every installable component must satisfy before being admitted to the shared library. This is the enforceable quality contract between component authors and the system.

## Requirements
- Define a `ComponentContract` metadata structure containing:
  - Component name and version
  - Props with types
  - Supported variants (e.g., primary, secondary, ghost)
  - Supported states (hover, active, focused, disabled, error, open, selected, readonly)
  - Token dependencies (which design tokens the component requires)
  - Interaction checklist hooks (focus, keyboard, pointer, state model, disabled/readonly)
  - Performance evidence links
  - Required files list
- Enforce builder-pattern composition for component APIs
- Require shared identifiers on all components: `id`, `tooltip`, optional `metadata` map
- Require explicit controlled vs uncontrolled state behavior documentation for stateful components (FR-017)
- Require explicit keyboard semantics: Tab, Enter/Space, arrow navigation (where applicable), Escape dismissal (where applicable) (FR-018)
- Enforce the Component Acceptance Checklist for every component:
  - Contract checks: focus behavior, keyboard model, pointer behavior, state model, disabled/readonly semantics
  - Design/token checks: surfaces mapped to frozen tokens, no hard-coded colors
  - Performance gates: release-mode measurements, no unapproved regressions, virtualized structures demonstrate bounded rendering
  - Quality gates: story/state matrix coverage, accessibility/interaction tests, provenance metadata complete
- Define disposition rules: Reuse (all checks pass), Fork (behavior passes but token/styling needs adaptation), Rewrite (interaction semantics or perf gates fail)

## Constraints
- Contract metadata lives alongside component source in `crates/components/`
- Contract must be machine-readable (Rust structs, derivable to JSON for registry)
- Component APIs must follow GPUI idioms (builder pattern, `RenderOnce`/`Render` traits)
- Performance gates require release-mode (`--release`) measurements using Zed-style workflows
- Interaction latency budget: primary hover/click/keyboard feedback within one 60Hz frame (~16.6ms) (NFR-008)

## Acceptance Criteria
1. `ComponentContract` struct exists in code with all required fields
2. At least one component has a fully populated contract (during POC phase)
3. Builder-pattern API is demonstrated on at least one component
4. Acceptance checklist is codified (runnable or documented) and referenced by component contracts
5. Disposition (Reuse/Fork/Rewrite) is recorded for each admitted component

## References
- Reference: `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/button.rs` — Zed builder-pattern component API reference
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/button/button.rs` — gpui-component builder-pattern reference
- Reference: `.refs/zed_gpui_refs/base-ui/packages/react/src/switch/root/SwitchRoot.tsx` — Base UI controlled/uncontrolled state pattern reference
- Reference: `.refs/zed_gpui_refs/primitives/packages/react/progress/src/progress.tsx` — Radix primitive contract pattern reference
- Plan Section 5: Component Acceptance Checklist
- Plan Section 7.2: Required Internal Contracts

## Decision Rationale
- Decision: Single enforceable contract per component with machine-readable metadata
- Why: Enables automated registry generation, deterministic CLI operations, and consistent quality gates
- Alternatives considered: Separate checklist documents per component (rejected — too easy to drift; machine-readable contracts enable tooling)
