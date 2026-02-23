# Implementation Plan

**Current state**: Workspace scaffold complete. Cargo workspace with full directory structure and crate stubs exists. Governance scaffolding complete. Ready to begin Wave 2 (P0.5-03 design tokens + P0.5-05 primitives).

**Active phase**: Phase 0.5 -- Wave 8. P0.5-01 DONE. P0.5-02 DONE. P0.5-03 DONE. P0.5-04 DONE. P0.5-05 DONE. P0.5-06 DONE. P0.5-07 DONE. P0.5-08 DONE. P0.5-09 DONE. P0.5-10 DONE. Next: P0.5-11 (CLI) and P0.5-12 (Workbench app).
**Phase 0.5 gate status**: NOT MET -- Waves 1-8 complete (P0.5-10 done). P0.5-11 (CLI) unblocked. P0.5-12 (Workbench app) can start in parallel.
**P0.5-10 completed**: 2026-02-22
**Last updated**: 2026-02-22
**P0.5-09 completed**: 2026-02-22
**P0.5-01 completed**: 2026-02-22
**P0.5-02 completed**: 2026-02-22
**P0.5-03 completed**: 2026-02-22
**P0.5-05 completed**: 2026-02-22
**P0.5-04 completed**: 2026-02-22
**P0.5-06 completed**: 2026-02-22
**P0.5-07 completed**: 2026-02-22
**P0.5-08 completed**: 2026-02-22
**Plan validated**: 2026-02-22 -- All specs read, codebase confirmed greenfield, dependency graph verified correct.
**Re-validated**: 2026-02-22 -- All 17 specs re-read and cross-referenced. 10 gaps identified and addressed below.
**Re-validated**: 2026-02-22 -- Deep spec analysis with parallel subagents. 8 additional forward-planning gaps identified (see Reconciliation §2).
**Re-validated**: 2026-02-22 -- Full plan audit: all 17 specs re-read, dependency graph confirmed correct, critical path confirmed, 1 minor gap addressed (`.planning/` directory for governance weekly risk reviews).
**Re-validated**: 2026-02-22 -- Full ralph-plan audit: all 17 specs confirmed complete, codebase confirmed 100% greenfield (zero implementation), reference codebase patterns validated against plan. No new gaps. Plan is accurate and ready for P0.5-01 execution.
**Build validation**: 2026-02-22 -- Confirmed: project CANNOT build in pre-P0.5-01 state (blockers now resolved, see Resolved Blockers).
**Reference analysis**: 2026-02-22 -- Deep analysis of gpui-component and Zed reference codebases. 8 technical insights captured as notes on P0.5-01, P0.5-03, P0.5-04, P0.5-05, P0.5-07.
**Re-validated**: 2026-02-22 -- ralph-plan audit with 3 parallel subagents: all 17 specs re-read and cross-referenced, codebase confirmed 100% greenfield (zero implementation), reference patterns re-validated against actual files. No new gaps found. Plan is accurate and ready for P0.5-01 execution.
**Re-validated**: 2026-02-22 -- ralph-plan audit with Opus analysis: 17 specs, codebase, and all reference paths re-verified. 8 gaps found (see Reconciliation §4). Key findings: font/asset loading missing from workbench plan, `open_window` closure signature needs correction, `resolver = "2"` missing from workspace scaffold, P0.5-12 may need P0.5-09 dependency.
**Re-validated**: 2026-02-22 -- Full ralph-plan audit with 4 parallel agents (specs, codebase, references, Opus auditor). Plan verdict: HIGH accuracy. 5 new low-severity gaps + 3 reference corrections captured (see Reconciliation §5). No dependency graph errors, no priority errors, no missing spec coverage. Plan is ready for P0.5-01 execution.
**Re-validated**: 2026-02-22 -- ralph-plan audit Round 6 with Opus auditor: 17 specs re-read, codebase confirmed 100% greenfield, dependency graph re-verified (0 errors), priority ordering re-verified (correct), all 11 gate criteria mapped to plan items. 2 low-severity gaps found (see Reconciliation §6). Plan remains HIGH accuracy and ready for P0.5-01 execution.

---

## Resolved Blockers (all resolved by P0.5-01)

1. **RESOLVED**: Build was broken -- `main.rs` at project root, `Cargo.toml` was a `[package]` not a workspace. Fixed by P0.5-01.
2. **RESOLVED**: GPUI API mismatch (`AppContext` vs `App`) -- corrected during P0.5-01 migration to `apps/studio/`.
3. **RESOLVED**: Struct naming collision (`struct App` shadowing `gpui::App`) -- renamed to `StudioApp` during P0.5-01.

---

## Phase 0.5: Proof of Concept

All items must complete and gate criteria must be met before any Phase 1 work begins (per `specs/delivery-gates.md`). Items are listed in dependency order. Earlier items unblock later items.

---

### P0.5-01: Workspace scaffold and Cargo workspace setup

- **Status**: COMPLETE
- **Spec**: `specs/delivery-gates.md` (workspace is prerequisite for all crate work)
- **What**: Convert the single-package project into a Cargo workspace with the full directory structure and empty crate stubs.
- **Sub-tasks**:
  1. Convert root `Cargo.toml` from `[package]` to `[workspace]` with `members` list and `resolver = "2"` (current `Cargo.toml` has `[package]` with `name = "gpui-workbench"` -- this entire section must be replaced with `[workspace]`). Both gpui-component and Zed workspaces use `resolver = "2"` -- required for correct feature unification in workspace builds.
  2. Move existing root `main.rs` to `apps/studio/src/main.rs` (note: file is currently at project root, NOT at `src/main.rs` -- the build is broken because of this). During the move: rename `struct App` to `StudioApp` (avoid shadowing `gpui::App`), update `Application::new().run()` closure to use `cx: &mut App` instead of `cx: &mut AppContext` (matches current GPUI convention per `create-gpui-app` templates). Also fix the `cx.open_window()` inner closure signature: it takes `|window, cx|` (two args: `&mut Window, &mut Context<T>`), not just `|cx|`. The create-gpui-app template uses `|_, cx|` but gpui-component examples name both args explicitly.
  3. Create `apps/cli/` binary crate with `main.rs` stub (clap placeholder)
  4. Create `crates/components/` library crate with `lib.rs` stub
  5. Create `crates/primitives/` library crate with `lib.rs` stub
  6. Create `crates/registry/` library crate with `lib.rs` stub
  7. Create `crates/theme/` library crate with `lib.rs` stub
  8. Create `crates/story/` library crate with `lib.rs` stub
  9. Create `crates/assets/` library crate with `lib.rs` stub (purpose: embedded binary assets -- fonts, icons -- following the gpui-component `crates/assets/` pattern. Required by P0.5-12 workbench for font registration via `cx.text_system().add_fonts(...)`. Without this, GPUI apps cannot render styled text reliably.)
  10. Create `templates/` directory (non-crate, file templates for CLI install targets)
  11. Verify `docs/` directory exists (already present but empty -- governance files go here in P0.5-02)
  12. Set GPUI dependency from Zed git source in workspace `[dependencies]`: `gpui = { git = "https://github.com/zed-industries/zed" }` (matches gpui-component and create-gpui-app convention -- no branch pin)
  12a. Also add `gpui_macros = { git = "https://github.com/zed-industries/zed" }` -- confirmed: gpui-component declares this as a separate workspace dependency (used for derive macros and inspector reflection in `crates/ui/src/styled.rs`). Not re-exported through `gpui` crate; must be declared independently.
  13. Add `serde`, `serde_json`, `toml`, `clap`, `smallvec` as workspace dependencies (Zed and gpui-component use `SmallVec<[AnyElement; N]>` for inline child element storage in component slots -- needed by Dialog, Select, Tabs action/content slot implementations)
  14. Verify `cargo build` succeeds for entire workspace
  15. Verify `cargo clippy --all-targets -- -D warnings` passes
- **Approach**: Start from the existing root `Cargo.toml` and `main.rs`. Scaffold all crates with minimal valid `Cargo.toml` + `lib.rs`/`main.rs`. Workspace-level dependency inheritance via `[workspace.dependencies]`.
- **GPUI API notes** (verified against reference codebases 2026-02-22):
  - `App` is the canonical context type for `Application::new().run()` closures (per `create-gpui-app` templates). `AppContext` also exists but `App` is preferred in current convention.
  - `Render` trait signature: `fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement`
  - gpui-component uses `pub fn init(cx: &mut App)` pattern for component/module initialization
  - Edition 2024 is correct (matches gpui-component `Cargo.toml`)
  - `gpui::App` is a struct (not a type alias) -- avoid naming application root views `App` to prevent shadowing
- **Workspace configuration notes** (from gpui-component reference analysis 2026-02-22):
  - Use `default-members = ["apps/studio"]` in `[workspace]` so bare `cargo build` builds only the workbench app, while `cargo build --workspace` builds everything. This is the pattern gpui-component uses to scope default builds to core crates and avoid building all examples.
  - Add `[profile.dev.package]` overrides to compile heavy crates at `opt-level = 3` even in dev builds. gpui-component does this for `gpui`, `gpui_platform`, `resvg`, `rustybuzz`, `taffy`, and `smol`. Without this, GPUI debug builds are unusably slow.
  - Consider adding `gpui_platform = { git = "https://github.com/zed-industries/zed", features = ["font-kit"] }` as a separate workspace dependency. gpui-component declares this independently (not re-exported through `gpui`) -- may be needed for font rendering.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/create-gpui-app-main/templates/default/src/main.rs` -- GPUI app bootstrap template
  - `.refs/zed_gpui_refs/zed-main/Cargo.toml` -- Zed workspace structure reference
  - `.refs/zed_gpui_refs/gpui-component-main/Cargo.toml` -- gpui-component workspace structure
- **Implementation discoveries** (captured 2026-02-22 during P0.5-01 execution):
  - **GPUI revision pinned to `d08d98f`**: unpinned resolution pulls `core-text 21.1.0`, which has a version conflict between `core-graphics 0.24` and `0.25` caused by `zed-font-kit`. Pinned to `d08d98f` (gpui-component's known-working revision) avoids this conflict entirely.
  - **`Application::new()` does NOT exist at this revision**: use `gpui_platform::application().run(...)` instead. `Application::new()` was removed or renamed; calling it causes a compile error.
  - **`runtime_shaders` feature required on `gpui_platform`**: without it, GPUI attempts to compile Metal shaders at build time, which requires full Xcode (not just Command Line Tools). Adding `features = ["runtime_shaders"]` to the `gpui_platform` dependency compiles shaders at runtime and avoids the Xcode requirement.
  - **`core-text` must be pinned to `21.0.0`**: run `cargo update core-text@21.1.0 --precise 21.0.0` after initial `cargo build` if the `core-graphics` version conflict surfaces. This pins `core-text` away from `21.1.0` which is the conflict source.
- **Dependencies**: None (START HERE)
- **Gate evidence**: `cargo build` and `cargo clippy` succeed for the full workspace.

---

### P0.5-02: Governance scaffolding

- **Status**: COMPLETE
- **Spec**: `specs/governance.md`
- **What**: Create the governance documentation files required by the delivery gate: provenance tracking, ADR templates, architecture and contracts stubs.
- **Sub-tasks**:
  1. Create `docs/PROVENANCE.md` with provenance metadata format (source URL, commit hash, license, local modifications per adapted file)
  2. Create `docs/ADR/` directory
  3. Create `docs/ADR/000-template.md` with lightweight ADR template (Context, Decision, Consequences)
  4. Create `docs/ARCHITECTURE.md` stub documenting the workspace layout and crate responsibilities
  5. Create `docs/CONTRACTS.md` stub referencing the component contracts schema
  6. Create `docs/ADR/001-workspace-layout.md` as the first real ADR (documents the monorepo structure decision)
  7. Implement CI provenance check gate (governance spec AC-004): create a script or cargo-xtask that validates PROVENANCE.md completeness -- checks that every adapted file listed in source has a corresponding provenance entry with URL, commit, license, and modifications
  8. Document WIP limit policy: max 2 active vertical slices at once (governance spec constraint)
  9. Define gate evidence storage convention: where phase gate evidence artifacts (test output, perf measurements, checklist results) are recorded (e.g., `docs/GATE-EVIDENCE/` or inline in IMPLEMENTATION_PLAN.md)
  10. Populate `.planning/` directory (already exists with historical planning content) with weekly risk review format definition (governance spec requirement: "weekly risk review updates in `.planning/`"). Define the lightweight review template (e.g., date, risks, mitigations, status changes).
- **Approach**: Author markdown files per governance spec requirements. Keep the ADR template lightweight for solo-dev workflow. Provenance check is a concrete script deliverable, not just documentation.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/zed-main/docs/` -- Zed documentation structure patterns
  - `specs/governance.md` -- Full requirements
- **Dependencies**: None (can run in PARALLEL with P0.5-01)
- **Gate evidence**: All files exist with documented formats. PROVENANCE.md indexes adapted files (initially empty). Provenance check script runs and passes (trivially, since no adapted files yet). Gate evidence storage convention documented.

---

### P0.5-03: Design tokens -- freeze Zed One Dark / One Light

- **Status**: COMPLETE
- **Spec**: `specs/design-tokens.md`
- **What**: Extract and freeze design token values from Zed One Dark and One Light themes into Rust types. These tokens are the single source of truth for all component colors, surfaces, and states.
- **Sub-tasks**:
  1. Parse `.refs/zed_gpui_refs/zed-main/assets/themes/one/one.json` to extract One Dark and One Light token values
  2. Study Zed theme settings structure to understand token categories and resolution
  3. Study gpui-component theme schema for compatibility considerations
  4. Define Rust structs in `crates/theme/src/tokens.rs` for token categories:
     - Foreground tokens (text, icon, link)
     - Background tokens (app, editor, panel, modal)
     - Border tokens (default, focused, active, disabled)
     - Surface tokens (primary, secondary, elevated)
     - Accent tokens (primary, secondary, info, success, warning, error)
     - Interaction state tokens (hover, active, focused, disabled, error, selected)
  5. Implement `Default` trait for One Dark baseline values
  6. Implement One Light as a second token set
  7. Add `serde::Serialize` + `serde::Deserialize` derives for JSON round-tripping
  8. Create mapping table (in code or `docs/`) from Zed theme JSON keys to internal token identifiers
  9. Write unit tests: token types compile, both theme sets load, serialization round-trips
- **Approach**: Parse the Zed theme JSON, map keys to semantic Rust identifiers, freeze values. Tokens must be compatible with GPUI's styling system (`Hsla`, `Rgba`, or similar).
- **Technical notes** (validated from `one.json` analysis):
  - Color format is `#RRGGBBAA` (8-char hex with alpha channel, e.g. `#282c33ff`)
  - Some values are `null` (inherit/transparent) -- must handle `Option<Color>` in token types
  - The JSON `style` object has ~15 categories: border (6 keys), surface/background (3), element states (5), ghost element states (5), text (5), icon (5), chrome/UI (17), editor (14), terminal (31 incl. ANSI colors), link (1), version control (7), semantic status (42 as triplets), players (8 entries x 3), syntax (42 token types with color/font_style/font_weight)
  - Both themes share identical key structure, differing only in values
  - For POC scope: focus on border, surface, element states, text, and icon categories. Editor/terminal/syntax tokens can be deferred to Phase 1+
- **Reference architecture note** (from gpui-component analysis 2026-02-22):
  - gpui-component `ThemeConfigColors` uses ~100 semantic color tokens typed as `Option<SharedString>` with a fallback derivation system. Unset tokens derive from base tokens via `Colorize` trait methods (`.opacity()`, `.blend()`, `.darken()`, `.lighten()`). This is more sophisticated than a flat token map -- consider whether P0.5 needs derivation or if flat tokens suffice for the 3 POC components. Flat tokens are simpler for POC; derivation can be layered in Phase 1 if needed.
  - Zed uses `DynamicSpacing::BaseNN.rems(cx)` for theme-aware spacing instead of hardcoded pixel values. Spacing tokens should be considered alongside color tokens, even if implementation is deferred to Phase 1.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/zed-main/assets/themes/one/one.json` -- Zed One Dark/Light source values
  - `.refs/zed_gpui_refs/zed-main/crates/settings_content/src/theme.rs` -- Zed theme settings structure
  - `.refs/zed_gpui_refs/zed-main/docs/src/appearance.md` -- Zed appearance/theme documentation
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/schema.rs` -- gpui-component theme schema (compatibility reference)
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/default-theme.json` -- gpui-component default theme values
- **Dependencies**: P0.5-01 (workspace must exist for `crates/theme/`)
- **Gate evidence**: Token Rust types compile, One Dark + One Light token sets load without error, mapping table documented, `cargo test -p theme` passes.

---

### P0.5-04: Theme engine -- load, switch, live edit

- **Status**: COMPLETE
- **Spec**: `specs/phase-0.5/theme-engine.md`
- **What**: Build the runtime theme engine that loads theme files, switches themes, supports live token editing, and provides a theme context for component token resolution.
- **Sub-tasks**:
  1. Design `ThemeRegistry` struct that holds loaded theme definitions
  2. Implement theme loading from frozen token sets (One Dark, One Light)
  3. Implement runtime theme switching via GPUI `Model`/context system
  4. Build theme context/provider that components access for token resolution (not direct token references)
  5. Implement token mutation API: change individual token values, propagate to all consumers
  6. Implement JSON import: load theme from `.json` file
  7. Implement JSON export: save current theme to `.json` file
  8. Implement TOML import: load theme from `.toml` file
  9. Implement TOML export: save current theme to `.toml` file
  10. Performance validation: theme switching completes within 100ms (NFR-009)
  11. Write tests: load, switch, edit, import/export round-trip
- **Approach**: Use GPUI's `Model<T>` or `Global` system for theme state. Components subscribe to theme changes and re-render. Token mutation triggers GPUI notification for live update propagation.
- **Pattern note**: Follow the `pub fn init(cx: &mut App)` centralized initialization pattern used throughout gpui-component and Zed crates. The theme engine crate should expose `theme::init(cx)` for app bootstrap.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/zed-main/crates/settings_content/src/theme.rs` -- Zed theme settings
  - `.refs/zed_gpui_refs/zed-main/assets/themes/one/one.json` -- Zed theme source
  - `.refs/zed_gpui_refs/zed-main/docs/src/appearance.md` -- Zed appearance documentation
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/schema.rs` -- gpui-component theme schema
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/default-theme.json` -- gpui-component default theme
- **Dependencies**: P0.5-03 (frozen token types must exist)
- **Gate evidence**: Theme loads, switches within 100ms, live edits propagate to rendered elements, JSON/TOML import/export works.

---

### P0.5-05: Primitives -- focus, keyboard, popover, state management

- **Status**: COMPLETE
- **Spec**: `specs/phase-0.5/primitives.md`
- **What**: Build unstyled behavior primitives that encapsulate shared focus, keyboard, popover, state, and pointer logic. These are the behavioral building blocks consumed by styled components.
- **Sub-tasks**:
  1. **Focus management primitive**:
     - Focus entry and exit (predictable focus flow)
     - Focus trap for modal contexts (Dialog) -- focus stays within, returns to trigger on close
     - Focus return on dismiss
     - Focus ring/indicator support via tokens
  2. **Keyboard navigation primitive**:
     - Tab/Shift-Tab for focus navigation between elements
     - Enter/Space for activation/selection
     - Arrow key navigation for lists (Select) and groups (Tabs)
     - Escape for dismissal (Dialog, Select popover)
     - Shortcut handling that is explicit and testable
  3. **Popover/overlay positioning primitive**:
     - Anchor-relative positioning (below trigger, above if constrained)
     - Viewport boundary awareness (flip/shift to stay visible)
     - Outside-click dismissal
     - Escape dismissal
  4. **State management primitive**:
     - Controlled vs uncontrolled behavior pattern
     - State types: open/closed, selected/unselected, active/inactive, disabled, error, readonly
     - State transition hooks for component consumers
  5. **Pointer behavior primitive**:
     - Click, hover semantics
     - Outside-click detection and dismissal (shared by Dialog and Select)
  6. Write unit tests for each primitive: `cargo test -p primitives`
  7. **Spike**: Build a minimal virtualized list using GPUI's `uniform_list` to verify it supports keyboard navigation, selection, and 10K+ item rendering within frame budget. This de-risks Phase 2 Table/Tree/Command Palette work early. Document findings in an ADR. (see Reconciliation Round 2, item 3)
- **Approach**: Study Radix/Base UI interaction patterns and translate to GPUI desktop semantics. Do NOT copy web API shapes -- translate interaction semantics. Implement only primitives needed by Dialog, Select, and Tabs for Phase 0.5. Extract primitives where 2+ components share behavior (FR-011).
- **Pattern note**: Follow the `pub fn init(cx: &mut App)` centralized initialization pattern. The primitives crate should expose `primitives::init(cx)` for registering any global state or event handlers at app bootstrap.
- **Adoption disposition**: N/A (original implementation informed by Radix/Base UI patterns)
- **Reference files to study**:
  - `.refs/zed_gpui_refs/primitives/packages/react/progress/src/progress.tsx` -- Radix primitive layering pattern
  - `.refs/zed_gpui_refs/base-ui/packages/react/src/switch/root/SwitchRoot.tsx` -- Base UI controlled/uncontrolled state pattern
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/modal.rs` -- Zed modal focus trap behavior
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/popover.rs` -- Zed popover positioning and dismiss
  - `.refs/zed_gpui_refs/zed-main/crates/gpui/README.md` -- GPUI event model and rendering architecture
- **Dependencies**: P0.5-01 (workspace must exist for `crates/primitives/`); can run in PARALLEL with P0.5-03 and P0.5-04
- **Gate evidence**: All primitives compile, used by at least one POC component, `cargo test -p primitives` passes.

---

### P0.5-06: Component contracts schema

- **Status**: COMPLETE
- **Spec**: `specs/component-contracts.md`
- **What**: Define the `ComponentContract` metadata schema -- the structured description of a component's props, states, tokens, interaction checklist, performance evidence, and required files.
- **Sub-tasks**:
  1. Define `ComponentContract` Rust struct in `crates/components/` (or a shared types module) with fields:
     - `name: String`, `version: String`
     - `props: Vec<PropDef>` (name, type, required, default)
     - `variants: Vec<String>` (e.g., primary, secondary, ghost)
     - `states: Vec<StateDef>` (hover, active, focused, disabled, error, open, selected, readonly)
     - `token_dependencies: Vec<TokenRef>` (which design tokens the component requires)
     - `interaction_checklist: InteractionChecklist` (focus, keyboard, pointer, state model, disabled/readonly)
     - `perf_evidence: Option<PerfEvidence>` (links to measurements)
     - `required_files: Vec<String>` (file paths needed for installation)
  2. Implement builder-pattern construction for `ComponentContract`
  3. Define shared identifiers required on all components: `id`, `tooltip`, optional `metadata` map
  4. Document controlled vs uncontrolled state behavior conventions (FR-017)
  5. Document keyboard semantics conventions: Tab, Enter/Space, arrow nav, Escape (FR-018)
  6. Define disposition rules: Reuse, Fork, Rewrite (with criteria for each)
  7. Validate by instantiating contracts for Dialog, Select, and Tabs (even before impl -- contract-first)
  8. Add serde derives for JSON serialization (registry consumption)
  9. Write tests: contract construction, serialization, validation
- **Approach**: Machine-readable Rust structs that serialize to JSON for registry and CLI consumption. Builder pattern matches GPUI idioms.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/button.rs` -- Zed builder-pattern component API reference
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/button/button.rs` -- gpui-component builder-pattern reference
  - `.refs/zed_gpui_refs/base-ui/packages/react/src/switch/root/SwitchRoot.tsx` -- Base UI controlled/uncontrolled state pattern
  - `.refs/zed_gpui_refs/primitives/packages/react/progress/src/progress.tsx` -- Radix primitive contract pattern
- **Dependencies**: P0.5-03 (token types for `TokenRef`), P0.5-05 (state model from primitives)
- **Gate evidence**: `ComponentContract` struct compiles, 3 POC component contracts instantiate and validate, serialization to JSON works.

---

### P0.5-07: POC components -- Dialog, Select, Tabs

- **Status**: COMPLETE
- **Spec**: `specs/phase-0.5/poc-components.md`
- **Adoption disposition**: ALL THREE are **Fork** -- adapt from reference implementations, normalize to internal contracts/tokens.
- **What**: Implement three styled POC components that validate the entire stack end-to-end: tokens, primitives, contracts, stories, workbench, registry, and CLI.
- **Sub-tasks**:

  **Dialog** (Fork from Zed `modal.rs` + gpui-component `dialog.rs`):
  1. Modal overlay rendering with backdrop
  2. Focus trap: focus stays within dialog while open
  3. Focus return: focus returns to trigger element on close
  4. Escape key dismissal
  5. Outside-click dismissal (click on backdrop)
  6. Title, description, and action slots
  7. Controlled open/close state
  8. Builder-pattern API with `id`, `tooltip`, optional `metadata`
  9. Map all surfaces (backdrop, panel, border, text) to frozen design tokens
  10. Populate `ComponentContract` for Dialog
  11. Write provenance metadata (source repos, commits, licenses, modifications)

  **Select** (Fork from gpui-component `select.rs` with Radix/Base UI normalization):
  1. Trigger button rendering
  2. Popover dropdown list, positioned relative to trigger
  3. Viewport-aware positioning (flip if constrained)
  4. Arrow-key navigation through list items
  5. Enter/Space to select item, Escape to close
  6. Controlled and uncontrolled selected value
  7. Disabled state (blocks interaction, visual feedback)
  8. Placeholder text when no value selected
  9. Builder-pattern API with `id`, `tooltip`, optional `metadata`
  10. Map all surfaces to frozen design tokens
  11. Populate `ComponentContract` for Select
  12. Write provenance metadata

  **Tabs** (Fork from Zed `tab.rs`/`tab_bar.rs` + gpui-component tabs):
  1. Tab bar with clickable tab triggers
  2. Associated content panels per tab
  3. Left/Right arrow key navigation between tabs
  4. Tab/Shift-Tab moves focus into/out of the tab bar (not between tabs)
  5. Active tab state with visual indicator mapped to tokens
  6. Controlled and uncontrolled active tab
  7. Disabled individual tabs
  8. Builder-pattern API with `id`, `tooltip`, optional `metadata`
  9. Map all surfaces to frozen design tokens
  10. Populate `ComponentContract` for Tabs
  11. Write provenance metadata

  **All three**:
  12. Consume primitives from `crates/primitives/` (no inline behavior)
  13. Consume tokens from `crates/theme/` (no hard-coded colors)
  14. Component APIs follow GPUI idioms (`RenderOnce`/`Render` traits, builder pattern)
      - **GPUI pattern clarification** (from reference analysis 2026-02-22): Use `#[derive(IntoElement)]` + `impl RenderOnce` for components (consumed on render, zero-cost). Use `impl Render` only for persistent views with identity (windows, panels). All three POC components should be `RenderOnce` unless they need persistent state across renders.
      - **Slot pattern**: For generic content slots (Dialog title/description/actions, Tabs content panels), use `<E: IntoElement>(impl Into<Option<E>>)` parameters, type-erased to `AnyElement` for storage. This is the established gpui-component pattern for composable slots.
      - **Select architecture note** (from reference analysis 2026-02-22): gpui-component uses Entity + render wrapper split for stateful components: `SelectState<D>` (stateful entity with `SelectDelegate` trait) + `Select<D>` (stateless `RenderOnce` wrapper). Dialog uses a single struct with `FocusHandle`. Our Select should follow the Entity + wrapper pattern; Dialog and Tabs can use single-struct `RenderOnce` unless they need persistent cross-render state.
  15. Interaction latency within one 60Hz frame (~16.6ms) (NFR-008)
  16. Release-mode performance evidence collected
  17. `cargo test -p components` passes

- **Reference files to study**:
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/modal.rs` -- Zed modal shell semantics
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/notification/alert_modal.rs` -- Zed alert modal
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/dialog.rs` -- gpui-component dialog
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/select.rs` -- gpui-component select
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/dropdown_menu.rs` -- Zed dropdown/select interaction
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/popover.rs` -- Zed popover positioning
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tab.rs` -- Zed tab component
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tab_bar.rs` -- Zed tab bar
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tab/tab.rs` -- gpui-component tab
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tab/tab_bar.rs` -- gpui-component tab bar
- **Dependencies**: P0.5-03 (tokens), P0.5-04 (theme engine), P0.5-05 (primitives), P0.5-06 (contracts schema)
- **Implementation discoveries** (captured 2026-02-22 during P0.5-07 execution):
  - **Stack overflow in test compilation**: GPUI's `#[derive(IntoElement)]` proc macro causes deep type recursion that overflows the compiler stack when test harness expands `#[test]` in the same crate. Solution: moved component contract/navigation tests to `tests/contract_tests.rs` (integration test file). Set `#![recursion_limit = "2048"]` in lib.rs.
  - **`v_flex()` and `h_flex()` not in GPUI**: These convenience functions exist in gpui-component but not in raw GPUI at rev d08d98f. Use `div().flex().flex_col()` and `div().flex().flex_row()` instead.
  - **`overflow_y_scroll()` not in GPUI**: Use `.overflow_hidden()` at this revision.
  - **`z_index()` not in GPUI**: Not available at this revision. Use `deferred().with_priority()` for overlay stacking.
  - **`FluentBuilder` trait must be imported**: The `.when()` method requires `use gpui::prelude::FluentBuilder`.
  - **All three POC components use RenderOnce**: Per plan guidance, Dialog/Tabs are single-struct RenderOnce; Select could benefit from Entity+wrapper pattern in Phase 1 for persistent state.
  - **Components are stateless (RenderOnce)**: State management delegated to parent via on_change callbacks. Full stateful (Entity-based) variants deferred to Phase 1 integration with workbench.
- **Gate evidence**: All 3 styled, pass acceptance checklist, map to frozen tokens, provenance complete, `cargo test` passes, performance evidence collected.

---

### P0.5-08: Story framework

- **Status**: COMPLETE
- **Spec**: `specs/phase-0.5/story-framework.md`
- **What**: Build the trait-based story system for rendering components in the workbench with state matrix support.
- **Sub-tasks**:
  1. Define `Story` trait in `crates/story/` with `render` method and state configurations
  2. Implement story registration mechanism for auto-discovery by the workbench
  3. Build state matrix rendering: display all combinations of variants x states for a component
  4. Implement story isolation: stories render components without inter-component dependencies
  5. Integrate theme context: stories render with current theme, update on theme switch
  6. Generate state matrix from `ComponentContract` metadata where possible (reduce boilerplate)
  7. Implement stories for Dialog (default, open, with-actions, escape-dismiss)
  8. Implement stories for Select (default, open, selected, disabled, with-placeholder)
  9. Implement stories for Tabs (default, 2-tabs, 5-tabs, disabled-tab, active-states)
  10. Verify: adding a new story requires only implementing the trait (no manual wiring beyond registration)
- **Approach**: Trait-based, Rust-idiomatic. Stories are co-located with components or in a separate stories module. Registration via inventory pattern or explicit listing.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/zed-main/crates/ui/src/components/` -- Zed component patterns (stories inline in storybook)
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/` -- gpui-component structure (stories alongside components)
- **Implementation discoveries** (captured 2026-02-22 during P0.5-08 execution):
  - **Stack overflow in test compilation**: Same issue as P0.5-07 -- GPUI's `IntoElement` derive causes stack overflow when compiling tests that transitively depend on `components` crate. Solution: moved all story tests to `tests/story_tests.rs` (integration test file). Set `#![recursion_limit = "2048"]` in lib.rs.
  - **GPUI element builders require `'static` lifetimes**: Borrowed `&str` references cannot be passed into GPUI element builder chains (`.child()`, etc.) because the builder requires `'static`. Solution: convert to owned `SharedString` before passing into builders. Used config struct pattern (DialogPreviewConfig) to avoid clippy's too-many-arguments lint.
  - **Story rendering is inline (not overlay)**: Dialog stories render the dialog panel directly (without overlay backdrop) because stories display multiple dialogs simultaneously. Full overlay rendering would obscure other content.
- **Dependencies**: P0.5-07 (POC components must exist to write stories for them)
- **Gate evidence**: Story framework renders all 3 POC components with state matrix coverage, theme switching updates stories.

---

### P0.5-09: Registry -- component index from source metadata

- **Status**: COMPLETE
- **Spec**: `specs/phase-0.5/registry.md`
- **What**: Build the local component registry that indexes components from source metadata for deterministic CLI operations.
- **Sub-tasks**:
  1. Create `RegistryIndex` struct in `crates/registry/` containing indexed component entries
  2. Create `RegistryEntry` struct: name, version, variants, states, props (with types), required files
  3. Implement registry generation: read `ComponentContract` metadata from component source
  4. Implement lookup API: resolve a component by name, return full metadata
  5. Implement listing API: list all registered components with metadata summary
  6. Support deterministic versioning for add/upgrade semantics
  7. Ensure registry is regenerable from source (no stale cache as source of truth)
  8. Performance: registry generation within 2 seconds (NFR-010)
  9. Verify all 3 POC components (Dialog, Select, Tabs) are indexed
  10. Add serde serialization for JSON output
- **Approach**: Generated from source, not hand-maintained manifests (FR-006). Reads `ComponentContract` from component crate. Entirely local -- no remote service.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts` -- shadcn registry/add workflow reference
  - `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/lib.rs` -- gpui-component module inventory
- **Dependencies**: P0.5-06 (contracts schema), P0.5-07 (POC components with populated contracts)
- **Gate evidence**: Registry indexes all 3 POC components, lookup by name works, regeneration within 2 seconds.

---

### P0.5-10: Plan contract -- JSON schema for plan/apply

- **Status**: COMPLETE
- **Spec**: `specs/plan-contract.md`
- **What**: Define the canonical JSON schema for `plan` and `apply` payloads that enables deterministic, agent-readable file mutations.
- **Sub-tasks**:
  1. Define `PlanContract` Rust struct with serde serialization:
     - Operation type (add, update, remove)
     - Target component name and version
     - Ordered list of file mutations: action (create, modify, delete), file path, mutation strategy (append_export, insert_use, replace_section)
     - Conflict detection results (list of conflicts, empty if none)
     - Provenance actions (files requiring attribution metadata)
  2. Implement `TemplateAdapter` trait/abstraction for different target app layouts
  3. Define default target layout: `src/shared/ui/<component>/` with `mod.rs` exports and token injection
  4. Implement plan generation: given component + target layout, produce deterministic plan
  5. Implement conflict detection: detect when target files already exist or have modifications
  5a. Include file checksums in plan output (`file_checksums: HashMap<PathBuf, String>`) -- enables Phase 1 `doctor` command integrity checking without schema retrofit (see Reconciliation Round 2, item 1)
  6. Implement determinism validation: identical inputs yield identical plans (NFR-001)
  7. Implement apply failure recovery: clear post-failure state report (NFR-002)
  8. Plan generation sub-second to low-second range (NFR-003)
  9. Plan output detailed enough for agent reconstruction of resulting file tree (FR-016)
  10. Write tests: determinism, conflict detection, serialization
- **Approach**: Rust types are source of truth, serialize to JSON via serde. Plan does not mutate files -- only apply does.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts` -- shadcn add command (distribution workflow reference)
- **Dependencies**: P0.5-09 (registry for component metadata resolution)
- **Gate evidence**: Plan contract JSON schema defined, deterministic output for POC components, conflict detection works.

---

### P0.5-11: CLI -- `add`, `plan`, `apply` commands

- **Status**: NOT STARTED
- **Spec**: `specs/phase-0.5/cli.md`
- **What**: Build the `gpui` CLI binary with three commands for component installation and plan management.
- **Sub-tasks**:
  1. Set up `clap` CLI argument parsing in `apps/cli/src/main.rs`
  2. Implement `gpui add <component>` -- install component source into target app
  3. Implement `gpui add <component> --plan` / `gpui plan <component>` -- output deterministic JSON mutation plan
  4. Implement `gpui apply <plan-file>` -- execute a previously generated plan
  5. Wire registry consumption: resolve component by name via `crates/registry/`
  6. Wire plan generation: produce plans via `PlanContract` from P0.5-10
  7. Implement JSON output for all commands (FR-003) using a shared `CliOutput<T>` envelope type (see Reconciliation Round 2, item 8)
  7a. Define `CliOutput<T>` envelope struct: `{ success: bool, data: T, errors: Vec<CliError> }` -- shared by all CLI JSON output for schema consistency across P0.5 and P1 commands
  8. Implement idempotent re-runs: adding an already-installed component is conflict-aware (FR-004)
  9. Capture provenance metadata for copied/adapted files during install (FR-005)
  10. Default install layout: `src/shared/ui/<component>/`, update `mod.rs` exports, inject theme tokens
  11. Verify `cargo install` from CLI crate produces a working `gpui` binary
  12. Test all 3 POC components: `gpui add dialog --plan`, `gpui add select --plan`, `gpui add tabs --plan`
  13. Test apply: `gpui apply` installs component source correctly
- **Approach**: shadcn-style source-copy distribution. Plan-first by default: mutation commands show plan and require explicit apply. Consume registry and plan contract.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts` -- shadcn add command
  - `.refs/zed_gpui_refs/create-gpui-app-main/README.md` -- create-gpui-app CLI patterns
- **Dependencies**: P0.5-09 (registry), P0.5-10 (plan contract)
- **Gate evidence**: `gpui add dialog --plan` returns deterministic JSON, `gpui apply` installs component, works for all 3 POC components, `cargo install` produces working binary.

---

### P0.5-12: Workbench app -- launch and display stories

- **Status**: NOT STARTED
- **Spec**: `specs/phase-0.5/workbench-app.md`
- **What**: Build the desktop GPUI application that serves as component explorer, story renderer, and theme editor.
- **Sub-tasks**:
  1. Bootstrap GPUI application window in `apps/studio/src/main.rs` (fork Zed window shell)
  1a. Register fonts via `crates/assets/` initialization: embed font files with `include_bytes!` and register with `cx.text_system().add_fonts(...)` at startup. Without this, text rendering falls back to unpredictable system fonts. Follow gpui-component `assets::init(cx)` pattern.
  2. Create main layout: sidebar/navigator + content area
  3. Component explorer: list all registered story components in sidebar
  4. Story rendering: selecting a component renders its story with state matrix
  5. Theme switching UI: toggle between One Dark / One Light
  6. Token editor panel: edit individual token values with live preview
  7. Display component metadata from contracts (props, variants, states)
  8. Wire theme engine: all component previews update immediately on theme switch or token edit
  9. Verify: app launches reliably on macOS without runtime panics
  10. Verify: all 3 POC components visible and interactive in the workbench
- **Approach**: Fork Zed window shell as bootstrap. Integrate story framework for component rendering. Wire theme engine for live switching and editing.
- **Reference files to study**:
  - `.refs/zed_gpui_refs/zed-main/crates/gpui/README.md` -- GPUI application architecture
  - `.refs/zed_gpui_refs/zed-main/docs/src/development.md` -- Zed development patterns
  - `.refs/zed_gpui_refs/create-gpui-app-main/templates/default/src/main.rs` -- GPUI app bootstrap template
  - `.refs/zed_gpui_refs/create-gpui-app-main/README.md` -- create-gpui-app scaffolding
- **Dependencies**: P0.5-04 (theme engine), P0.5-07 (POC components), P0.5-08 (story framework). NOTE: If the component explorer sidebar is registry-driven (not hardcoded), P0.5-09 (registry) is also a dependency. For POC, a hardcoded component list is acceptable; registry integration can be wired when P0.5-09 completes.
- **Gate evidence**: App launches on macOS, displays all 3 POC components in story form, theme switch works, token editing propagates.

---

## Phase 0.5 Gate Criteria

All of the following must be true before Phase 1 work begins (from `specs/delivery-gates.md`):

- [ ] Design tokens frozen in Rust from Zed One Dark/One Light
- [ ] Primitives for focus, keyboard, popover, state exist and used by POC components
- [ ] Component contracts schema defined and validated against 3 POC components
- [ ] Dialog, Select, Tabs styled, pass acceptance checklist, map to frozen tokens
- [ ] Theme engine loads, switches, supports live token editing
- [ ] Story framework renders all 3 POC components with state matrix
- [ ] Workbench app launches on macOS, displays all 3 POC components
- [ ] Registry indexes all 3 POC components from source metadata
- [ ] CLI add/plan/apply work for all 3 POC components
- [ ] Plan contract JSON schema defined, deterministic output
- [ ] Governance scaffolding in place (PROVENANCE.md, ADR template, provenance metadata)

---

## Phase 1: Full Core-12 (BLOCKED -- Phase 0.5 gate not met)

### P1-01: Core-9 components
- **Status**: BLOCKED
- **Spec**: `specs/phase-1/core-9-components.md`
- **What**: Button, Input, Textarea, Checkbox, Radio, Tooltip, DropdownMenu, Popover, Toast. All Fork disposition except Tooltip (Reuse with token map).
- **Notable spec requirements**:
  - Radio: checklist-driven keyboard model -- arrow key navigation within group, disabled individual items AND entire group
  - Toast: multiple concurrent toasts (stacking), variants (info/success/warning/error), auto-dismiss timer, action buttons
  - Each component: full `ComponentContract`, story coverage with state matrix, release-mode performance evidence, provenance metadata
- **Forward-planning** (from Reconciliation Round 2):
  - Design Input/Textarea/Menu/Notification/Tabs APIs with trait-based extension points for Phase 2 extended variants (item 2)
  - Identify shared keyboard/focus behaviors across the 9 components early in design to guide primitive extraction per FR-011 (item from Phase 1-2 spec analysis)
  - Evaluate Tooltip against acceptance checklist to confirm Reuse disposition before implementation begins

### P1-02: Additional primitive extraction
- **Status**: BLOCKED
- **Spec**: `specs/phase-1/core-9-components.md` (FR-011)
- **What**: Extract new primitives where Core-9 reveals shared behavior across 2+ components.

### P1-03: CLI hardening
- **Status**: BLOCKED
- **Spec**: `specs/phase-1/cli-hardening.md`
- **What**: `update`, `remove`, `list`, `doctor`, `init` commands. Full deterministic upgrade flow. `init` uses `TemplateAdapter`.
- **Notable spec requirements**:
  - `doctor`: verify compatibility and integrity of target app (FR-012) -- check dependencies, validate installed files, report health in human and JSON format. Depends on file checksums from plan contract (see P0.5-10 sub-task 5a).
  - `init`: scaffold new GPUI app through template adapters (FR-013) -- single default template in Phase 1, multi-template in Phase 2. Design `TemplateAdapter` with multi-template extensibility from the start (Reconciliation Round 2, item from cli-hardening analysis).
  - `update`: preserve local modifications where possible during upgrade
  - All mutation commands plan-first and idempotent
  - All JSON output must use the `CliOutput<T>` envelope type from P0.5-11 (Reconciliation Round 2, item 8)
- **Forward-planning**:
  - Define Homebrew maturity criteria in governance docs (Reconciliation Round 2, item 6)
  - Begin identifying pilot app candidates for Phase 2 adoption gate (Reconciliation Round 2, item 7)

### P1-04: Full workbench + story coverage
- **Status**: BLOCKED
- **What**: Workbench renders all 12 Core components with state matrix story coverage. Performance evidence for all.

---

## Phase 2: Full Library + Distribution (BLOCKED -- Phase 1 gate not met)

### P2-01: Extended library (39 remaining components)
- **Status**: BLOCKED
- **Spec**: `specs/phase-2/extended-library.md`
- **What**: All 39 remaining components from gpui-component inventory. Includes extended variants of Phase 1 modules: `input` advanced behaviors, extended `menu` variants beyond DropdownMenu, `notification` variants beyond Toast, `tab` utilities beyond Tabs. Virtualized structures (list, table, tree, scroll) must demonstrate bounded rendering.

### P2-02: Priority components (Table, Tree, Command Palette)
- **Status**: BLOCKED
- **Spec**: `specs/phase-2/priority-components.md`
- **What**: Three high-priority components. Table/Tree are Fork, Command Palette is Rewrite.
- **Notable spec requirements**:
  - Table: virtualized rendering, 10,000+ rows without frame budget regression, column sort, row selection (single/multi). Must use GPUI `uniform_list` or equivalent (de-risked by P0.5-05 spike).
  - Tree: virtualized rendering, 10,000+ nodes without regression, expand/collapse, drag-and-drop reordering. Drag-and-drop may require a new primitive if shared by other components.
  - Command Palette: Rewrite (Zed impl too app-specific), fuzzy search 1,000+ commands with sub-frame latency. Requires a decoupled `CommandRegistry` designed independently from Zed's editor-specific system (Reconciliation Round 2, item 4).

### P2-03: Distribution
- **Status**: BLOCKED
- **Spec**: `specs/phase-2/distribution.md`
- **What**: Cargo install, Homebrew, multi-template init, pilot adoption.
- **Notable spec requirements**:
  - Documentation polish: README, getting started guide, component API docs
  - Theme import/export hardened for production (round-trip fidelity)
  - Local visual snapshot baseline and diff checks (non-cloud). Requires visual snapshot tooling selection before Phase 2 delivery begins (Reconciliation Round 2, item 5).
  - Pilot adoption: at least one external app uses 3+ installed components. Pilot identification must begin during Phase 1 (Reconciliation Round 2, item 7).
  - `gpui init` expanded to support multiple templates (depends on `TemplateAdapter` extensibility from P1-03)
  - Homebrew formula triggered by maturity criteria defined in P1-03 (Reconciliation Round 2, item 6)

---

## Spec-to-Plan Reconciliation

### Round 1 (2026-02-22)

Gaps identified by cross-referencing all 17 specs against this plan:

1. **ADDRESSED in P0.5-02**: CI provenance check gate now a concrete script deliverable (was just documentation note)
2. **ADDRESSED in P0.5-02**: WIP limit policy (max 2 active slices) and gate evidence storage convention added
3. **NOTED in P0.5-06**: FR-017 (controlled/uncontrolled docs) and FR-018 (keyboard semantics docs) are sub-tasks 4-5; acceptance validated by contract instantiation for 3 POC components
4. **ADDRESSED in P1-01**: Radio group keyboard model specifics and Toast stacking/variants now called out
5. **ADDRESSED in P2-01**: Extended variants of Phase 1 modules explicitly noted
6. **ADDRESSED in P2-02**: Tree drag-and-drop and performance thresholds (10K rows/nodes) explicitly noted
7. **ADDRESSED in P2-03**: Documentation polish deliverable and local visual snapshot baseline noted
8. **NOTED**: `TemplateAdapter` tested in P0.5-10 (trait defined), validated end-to-end in P0.5-11 (CLI uses it), expanded in P1-03 (`init` command)
9. **NOTED**: Gate evidence recording convention is now part of P0.5-02 deliverables
10. **NO GAP**: All other spec requirements are adequately covered by existing plan items

### Round 2 (2026-02-23) -- Forward-Planning Gaps

Deep spec analysis with parallel subagents identified 8 items that need earlier design attention, even though their implementation is gated by later phases:

1. **NEEDS DESIGN in P0.5-10**: Plan contract should include file manifest/checksum format. The Phase 1 `doctor` command (cli-hardening.md) needs to "validate installed component files match expected state" (AC-7). This requires file fingerprinting in the plan contract schema. Add a `file_checksums: HashMap<PathBuf, String>` field to `PlanContract` during P0.5-10 design so `doctor` doesn't need to retrofit later.

2. **NEEDS DESIGN in P1-01**: Phase 1 component APIs must be designed with extension points for Phase 2 extended variants. `extended-library.md` lines 48-51 require extended `input`, `menu`, `notification`, and `tab` variants. Design trait-based extension hooks during Phase 1 component implementation (e.g., `InputBehavior` trait that both basic Input and advanced Input variants implement). Document in ADR.

3. **NEEDS EVALUATION in P0.5-05 or P1-02**: GPUI's `uniform_list` virtualization primitive must be evaluated during Phase 0.5 or early Phase 1. Phase 2 priority components (Table, Tree) are hard-gated on `uniform_list` working for their interaction models (priority-components.md line 45). Add a sub-task to P0.5-05 or create a spike task: build a minimal virtualized list using `uniform_list` and verify it supports keyboard navigation, selection, and 10K+ item rendering within frame budget.

4. **NEEDS DESIGN in P2-02**: Command Palette requires a decoupled command registration API. This is the only Rewrite-disposition component. The registration system (priority-components.md line 26) must be designed independently from Zed's editor-specific command system. Plan a `CommandRegistry` trait in `crates/primitives/` or a new `crates/commands/` crate during Phase 2 design.

5. **NEEDS DECISION before P2-03**: Visual snapshot tooling selection. `distribution.md` line 13 requires "local visual snapshot baseline and diff checks" but names no specific tool. Decide on approach (e.g., GPUI screenshot API + pixel-diff library, or manual golden-file approach) before Phase 2 component delivery begins. Add to Phase 1 or early Phase 2 planning.

6. **NEEDS DEFINITION before P2-03**: Homebrew maturity criteria. `distribution.md` line 9 says "after Cargo path reaches maturity criteria" but those criteria are undefined. Define in P1-03 or P0.5-02 governance docs: e.g., "Cargo install works in clean environment, all Core-12 installable, no known install-time panics."

7. **NEEDS COORDINATION in P1**: Pilot app identification for Phase 2 gate. `distribution.md` AC-5 requires "at least one external app successfully installs and uses 3+ components." This can't start cold at Phase 2. Identify candidate pilot during Phase 1 work.

8. **NEEDS DESIGN in P0.5-11**: CLI JSON output schema consistency. All CLI commands across Phase 0.5 and Phase 1 must produce consistent JSON schemas (FR-003). Define a shared `CliOutput<T>` envelope type (with `success: bool`, `data: T`, `errors: Vec<CliError>`) in P0.5-11 and mandate its use in P1-03 hardening commands. Prevents schema drift between `add --json`, `list --json`, `doctor --json`.

### Round 3 (2026-02-22) -- Plan Audit

Full audit of all 17 specs against plan items. One minor gap found:

1. **ADDRESSED in P0.5-02**: Governance spec requires "weekly risk review updates in `.planning/`" (governance.md line 15). P0.5-02 was missing the `.planning/` directory creation and risk review format definition. Added as sub-task 10.

### Round 4 (2026-02-22) -- Opus Deep Audit

Full audit with Opus analysis agent. 17 specs, codebase, and all reference paths re-verified. 8 items found:

1. **ADDRESSED in P0.5-01 sub-task 2**: `open_window()` inner closure signature is `|window, cx|` (two args: `&mut Window, &mut Context<T>`), not just `|cx|`. Plan now documents the correct signature. Omitting `window` arg causes compile error.
2. **ADDRESSED in P0.5-01 sub-task 1**: Missing `resolver = "2"` in workspace `[workspace]` block. Both gpui-component and Zed use it. Now documented.
3. **ADDRESSED in P0.5-01 sub-task 9**: `crates/assets/` crate purpose clarified -- required for font embedding and registration via `cx.text_system().add_fonts(...)`. No spec covers it, but it's a silent dependency of P0.5-12 workbench text rendering.
4. **ADDRESSED in P0.5-12 sub-task 1a (new)**: Font/asset loading was completely missing from workbench plan. Without registering fonts at startup, GPUI apps fall back to unpredictable system fonts. Added explicit sub-task for font registration following gpui-component `assets::init(cx)` pattern.
5. **ADDRESSED in P0.5-12**: P0.5-12 may need P0.5-09 (registry) as a dependency if the component explorer sidebar is registry-driven. Added note: hardcoded list is acceptable for POC; registry integration wired later.
6. **ADDRESSED in P0.5-02 sub-task 10**: `.planning/` directory already exists with historical content. Reworded from "Create" to "Populate with weekly risk review format."
7. **NOTED**: `[profile.dev.package]` list in plan (6 packages) is a subset of gpui-component's full list (14 packages). Adequate for POC; expand if build times are unacceptable.
8. **NOTED**: `toml` crate dependency in P0.5-01 sub-task 13 has no reference precedent in gpui-component or Zed, but is justified by theme engine TOML import/export requirement (P0.5-04 spec).

### Round 5 (2026-02-22) -- Full ralph-plan Audit with Opus

Complete audit with 4 parallel research agents: specs analysis, codebase exploration, reference codebase analysis, and Opus plan-vs-spec audit. All 17 specs re-read, codebase confirmed 100% greenfield, reference patterns validated against actual files.

**Plan verdict: HIGH accuracy.** All 12 Phase 0.5 specs confirmed covered, dependency graph correct (0 errors), priority ordering correct (P0.5-01 is right starting point), critical path confirmed.

5 new gaps found (all Low to Low-Medium severity):

1. **NOTED on P0.5-05**: Primitives spec lists "drag semantics" in pointer behavior (line 28), but plan sub-task 5 only lists "Click, hover semantics." Drag is NOT needed by Dialog, Select, or Tabs -- acknowledged as intentionally deferred to Phase 1+ when Tree (drag-and-drop reordering) requires it.

2. **ADDRESSED in P0.5-08 (new sub-task 10 already covers this)**: FR-007 requires "every CLI-installable component has a story." Plan sub-task 10 states "adding a new story requires only implementing the trait (no manual wiring beyond registration)" -- this validates the pattern but does not enforce it. Implementation should include a test or check that validates all registry entries have corresponding story implementations. Deferred to implementation time.

3. **NOTED on P0.5-10**: `PlanContract` struct crate location not specified. Recommendation: house in `crates/registry/` alongside `RegistryIndex` and `ComponentContract`, since plan generation consumes registry metadata. Alternatively, a `crates/plan/` crate could be created, but adds workspace complexity for a single type. Decision deferred to P0.5-10 implementation.

4. **ADDRESSED in P0.5-02**: Provenance check script (sub-task 7) should validate both `PROVENANCE.md` index entries AND inline provenance metadata on adapted source files (governance spec AC-003: "every adapted source file has provenance metadata"). Script scope clarified to include both checks.

5. **NOTED on P0.5-04/P0.5-07**: Theme engine spec AC-008 requires components access tokens "through a theme context/provider (not direct token references)." No enforcement mechanism exists. Recommend adding a code review checklist item or `clippy` custom lint during P0.5-07 implementation to verify components use `cx.theme()` rather than importing token constants directly.

3 reference codebase corrections/nuances captured:

6. **CORRECTED on P0.5-01 sub-task 9**: gpui-component's `crates/assets/` embeds only SVG icons via `rust-embed`, NOT fonts. Font registration likely happens through `gpui_platform` with the `font-kit` feature, not through `include_bytes!` + `cx.text_system().add_fonts(...)`. The assets crate should focus on icons; font loading approach needs further investigation during P0.5-12 implementation. The `rust-embed` crate with `interpolate-folder-path` feature is the established pattern.

7. **NOTED on P0.5-07 Dialog**: gpui-component Dialog depends on a `Root` view architecture for dialog stacking and closing (`Root::update(window, cx, ...)`, `window.close_dialog(cx)` extension methods). Our Dialog implementation will need either: (a) a similar `Root` container view in the workbench, or (b) an alternative dialog management approach using GPUI's `Entity` system directly. Decision deferred to P0.5-07 design.

8. **NOTED on P0.5-01**: gpui-component's assets crate uses `gpui_platform::application().with_assets(Assets)` for attaching asset sources, which differs from `Application::new()`. Need to verify whether both APIs are compatible or if the workbench bootstrap should use `gpui_platform::application()` instead. Investigation deferred to P0.5-12 workbench bootstrap.

### Round 6 (2026-02-22) -- Opus Deep Audit

Full audit with Opus analysis agent. All 17 specs re-read against every plan item, dependency edge, and gate criterion. Codebase confirmed 100% greenfield. Dependency graph verified correct (0 errors). Priority ordering verified correct. All 11 Phase 0.5 gate criteria mapped to plan items with full coverage.

2 gaps found (both Low severity):

1. **NEEDS ADDITION in P0.5-06 or P0.5-07**: Component Acceptance Checklist artifact. `specs/component-contracts.md` AC-4 requires the acceptance checklist to be "codified (runnable or documented) and referenced by component contracts." The plan's `InteractionChecklist` struct (P0.5-06 sub-task 1) only covers the interaction subset. The full 4-category checklist (contract checks, design/token checks, performance gates, quality gates) needs to be produced as a documented or runnable artifact. Recommend adding a sub-task to P0.5-06: "Document the full Component Acceptance Checklist as a runnable checklist covering contract, design/token, performance, and quality gates."

2. **NEEDS ADDITION in P0.5-11**: Test target app fixture. `specs/phase-0.5/cli.md` AC-4 requires `gpui apply <plan-file>` to install into a target app layout. The plan has no explicit sub-task to create a test target directory for apply verification. Recommend adding a sub-task: "Create a test target app fixture directory for apply verification testing."

1 optimization opportunity noted (not a gap):

3. **OPTIMIZATION**: P0.5-06 dependency on P0.5-05 could be relaxed. The contracts schema needs state TYPE definitions from primitives, but not the behavioral implementations. If state types are extracted as an interface first, P0.5-06 could start after P0.5-03 alone, shortening the critical path by one step. This is a scheduling optimization for implementation time, not a correctness issue.

---

## Spec Inventory

All 17 required specs exist and are complete:

| Category | Spec file | Phase |
|---|---|---|
| Cross-cutting | `specs/design-tokens.md` | 0.5 |
| Cross-cutting | `specs/component-contracts.md` | 0.5 |
| Cross-cutting | `specs/plan-contract.md` | 0.5 |
| Cross-cutting | `specs/delivery-gates.md` | All |
| Cross-cutting | `specs/governance.md` | 0.5 |
| Phase 0.5 | `specs/phase-0.5/primitives.md` | 0.5 |
| Phase 0.5 | `specs/phase-0.5/poc-components.md` | 0.5 |
| Phase 0.5 | `specs/phase-0.5/theme-engine.md` | 0.5 |
| Phase 0.5 | `specs/phase-0.5/story-framework.md` | 0.5 |
| Phase 0.5 | `specs/phase-0.5/workbench-app.md` | 0.5 |
| Phase 0.5 | `specs/phase-0.5/registry.md` | 0.5 |
| Phase 0.5 | `specs/phase-0.5/cli.md` | 0.5 |
| Phase 1 | `specs/phase-1/core-9-components.md` | 1 |
| Phase 1 | `specs/phase-1/cli-hardening.md` | 1 |
| Phase 2 | `specs/phase-2/extended-library.md` | 2 |
| Phase 2 | `specs/phase-2/priority-components.md` | 2 |
| Phase 2 | `specs/phase-2/distribution.md` | 2 |

---

## Implementation Order and Parallelization

### Dependency Graph

```
Wave 1 (parallel):
  P0.5-01  Workspace scaffold
  P0.5-02  Governance scaffolding

Wave 2 (after P0.5-01, parallel):
  P0.5-03  Design tokens          [needs: 01]
  P0.5-05  Primitives             [needs: 01]

Wave 3 (after P0.5-03):
  P0.5-04  Theme engine            [needs: 03]

Wave 4 (after P0.5-03 + P0.5-05):
  P0.5-06  Component contracts     [needs: 03, 05]

Wave 5 (after P0.5-03 + P0.5-04 + P0.5-05 + P0.5-06):
  P0.5-07  POC components          [needs: 03, 04, 05, 06]

Wave 6 (after P0.5-07, parallel):
  P0.5-08  Story framework         [needs: 07]
  P0.5-09  Registry                [needs: 06, 07]

Wave 7 (after P0.5-09):
  P0.5-10  Plan contract           [needs: 09]

Wave 8 (after P0.5-09 + P0.5-10):
  P0.5-11  CLI                     [needs: 09, 10]

Wave 9 (after P0.5-04 + P0.5-07 + P0.5-08):
  P0.5-12  Workbench app           [needs: 04, 07, 08]

--- Phase 0.5 Gate evaluation ---

Phase 1:  P1-01, P1-02, P1-03, P1-04  [needs: Phase 0.5 gate]
Phase 2:  P2-01, P2-02, P2-03         [needs: Phase 1 gate]
```

### Critical Path

The longest sequential chain determines minimum calendar time:

```
P0.5-01 -> P0.5-03 -> P0.5-04 -> P0.5-07 -> P0.5-08 -> P0.5-12
                                      \
P0.5-01 -> P0.5-03 -> P0.5-06 -> P0.5-07 -> P0.5-09 -> P0.5-10 -> P0.5-11
                  \                                                (longest)
P0.5-01 -> P0.5-05 -/
```

The CLI path (ending at P0.5-11) is the longest chain: 01 -> 03 -> 06 -> 07 -> 09 -> 10 -> 11 (7 items deep, with 05 joining at 06).

### Parallelization Opportunities

| Wave | Items that can run simultaneously |
|---|---|
| Wave 1 | P0.5-01, P0.5-02 |
| Wave 2 | P0.5-03, P0.5-05 |
| Wave 3-4 | P0.5-04 (after 03), P0.5-06 (after 03+05) -- partial overlap possible |
| Wave 6 | P0.5-08, P0.5-09 |
| Wave 8-9 | P0.5-11 (after 09+10), P0.5-12 (after 04+07+08) -- independent paths |

### Recommended Execution Strategy

1. **Start with P0.5-01 + P0.5-02 in parallel** -- unblocks everything else
2. **Immediately start P0.5-03 + P0.5-05 in parallel** once workspace exists
3. **P0.5-04 and P0.5-06 overlap** -- 04 only needs 03; 06 needs 03+05
4. **P0.5-07 is the convergence point** -- all foundations must be ready
5. **After P0.5-07, split into two parallel tracks**:
   - Track A (workbench): P0.5-08 -> P0.5-12
   - Track B (CLI): P0.5-09 -> P0.5-10 -> P0.5-11
6. **Gate evaluation** once both tracks complete
