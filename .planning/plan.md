# GPUI Studio: Master Requirements and Delivery Plan
**Product Vision**: A high-performance, desktop-first design system workbench and CLI for the GPUI/Rust ecosystem. It enables developers (and AI agents) to build, browse, and validate a reusable component library (ShadCN-style) while providing a CLI for scaffolding apps and installing components as source.

## 1. Product Definition

GPUI Studio is a desktop-first GPUI/Rust component workbench plus a companion CLI.

Primary outcomes:
- Build and validate a reusable GPUI component system (shadcn-style distribution, Radix/Base-style primitive layering, Zed-aligned GPUI correctness/performance).
- Enable deterministic `plan -> apply` workflows for both humans and agents.
- Deliver reusable source-installed components with strict provenance, repeatable upgrades, and low rework risk.

## 2. Highest-Truth Decisions (Authoritative)

This section is the highest truth for this repository. If any other section conflicts, this section wins and the conflicting section must be updated to match.

- Delivery model: dual-track with priority rule `CLI leads, workbench validates`.
- Consumption model: hybrid.
  - Default: source-copy installation into app codebases (shadcn-style distribution).
  - Optional: crate-based consumption for teams that want centralized updates.
- Source-of-truth order by domain:
  - GPUI correctness and performance: Zed implementation and GPUI crate patterns.
  - Interactions and primitive contracts: Radix/Base behavior patterns.
  - Breadth and acceleration: `gpui-component-main`.
  - Distribution workflow: shadcn CLI/registry model.
  - Feature scouting: Awesome GPUI.
- Theme policy:
  - Freeze design tokens from Zed One Dark and One Light first.
  - Map all component colors/surfaces/states to these frozen tokens before accepting components.
- Component adoption policy:
  - Reuse from `gpui-component-main` when it passes the acceptance checklist.
  - Fork when behavior is correct but styling/token mapping differs.
  - Rewrite when behavior, semantics, or performance does not pass gates.
- Registry policy:
  - Implement a shadcn-style local registry/CLI so add/upgrade/remove is deterministic and versionable.
  - Registry metadata remains Rust-first and generated from source.
- Performance policy:
  - Enforce Zed-style measurement workflows and performance gates before admitting components to shared library.
- Awesome GPUI policy:
  - Use only for targeted feature spikes and implementation discovery.
  - Normalize all adopted code to internal contracts/tokens before integration.
- Bootstrap/scaffold policy:
  - Use `create-gpui-app` and GPUI crate docs as baseline scaffold patterns while following Rust best practices.
- Provenance policy:
  - Strict provenance for any copied or adapted code remains mandatory.

## 2.1 Influence SWOT Matrix

| Influence | Strengths | Weaknesses | Opportunities | Threats | Strongest Dimensions |
| --- | --- | --- | --- | --- | --- |
| shadcn/ui | Proven local-distribution CLI (`init`/`add`), predictable install flow, easy per-project theming/versioning | Web/Tailwind assumptions are not directly GPUI-native | Mirror CLI ergonomics for GPUI component install/upgrade | Overfitting to web patterns can leak non-GPUI abstractions | Distribution workflow, DX, deterministic component install |
| Radix + Base UI primitives | Mature accessibility and interaction modeling, strong primitive layering | React/web event system differs from GPUI desktop event model | Use as canonical interaction contracts/checklists | Blind copy can break desktop conventions and GPUI idioms | Interaction semantics, keyboard/focus/state model quality |
| Zed IDE + GPUI crate | Best production GPUI reference, proven performance and rendering patterns, canonical One Dark/Light baseline | Pre-1.0 GPUI can break between versions; some components are app-specific | Use as correctness/perf source of truth and visual baseline | Tight coupling to Zed internals if copied naively | GPUI correctness, performance, visual baseline |
| create-gpui-app | Official bootstrap/scaffold for GPUI apps, fast startup path | Minimal template; not a full architecture blueprint | Standardize app scaffolding and workspace bootstrap conventions | Treating starter template as final architecture | Scaffolding conventions, initial project setup |
| gpui-component-main | Broad component surface area, shadcn-inspired design, virtualization and many practical components | Third-party quality varies by component; token model differs from Zed defaults | Accelerate delivery via reuse/fork path and reduce blank-slate work | Importing behavior bugs or style drift without gates | Breadth, speed, reusable starting implementations |
| Awesome GPUI | Broad ecosystem examples and feature-specific references | Curation quality and maintenance vary across linked repos | Fast scouting for niche features (video player, etc.) | Cargo-culting random patterns into core library | Feature scouting, targeted spike discovery |

## 3. Source of Truth by Domain (Inline SOURCE_OF_TRUTH.md)

| Domain | Primary source of truth | Secondary source | Use rule |
| --- | --- | --- | --- |
| GPUI architecture and rendering correctness | Zed GPUI + Zed app code | create-gpui-app template | If conflict exists, follow Zed GPUI semantics and runtime behavior |
| Performance measurement and acceptance | Zed measurement/profiling workflows | gpui-component measurement hooks | Component cannot be accepted without passing perf gates |
| Interaction primitives (focus, keyboard, dismiss, state) | Radix + Base UI patterns | Zed component behavior | Translate web semantics to GPUI; do not copy API shape blindly |
| Visual tokens and theme baseline | Zed One Dark/One Light | gpui-component theme schema compatibility | Freeze Zed tokens first, then map component tokens |
| Component breadth and implementation bootstrap | gpui-component-main | Zed component implementations | Prefer reuse/fork from gpui-component if checklist passes |
| Distribution/install UX | shadcn CLI model | Existing CLI contracts in this repo | Local registry + deterministic add/update/remove is required |
| Feature expansion scouting | Awesome GPUI links | Direct source repos linked there | Spike only, then normalize to internal contracts |

Tie-breaker rule:
- If two sources disagree, apply this order: `Zed GPUI correctness/perf > Radix/Base interaction model > gpui-component breadth > shadcn distribution model > Awesome GPUI examples`.

## 4. Systematic, Repeatable Adoption Plan

1. Define source-of-truth order by domain:
   GPUI correctness/perf = Zed, interactions = Radix/Base patterns, breadth = gpui-component, distribution workflow = shadcn, feature scouting = Awesome GPUI.
2. Freeze design tokens from Zed One Dark/Light first, then map all component colors/surfaces to those tokens.
3. Build a primitive contract checklist per component:
   focus behavior, keyboard model, pointer behavior, state model, disabled/read-only semantics.
4. Reuse from `gpui-component-main` when it passes checklist; fork when behavior is right but styling/token mapping differs; rewrite when behavior/perf is off.
5. Implement a shadcn-style local registry/CLI for GPUI components so add/upgrade is deterministic.
6. Enforce perf gates using Zed-style measurement workflows before accepting components into shared library.
7. Use Awesome GPUI only for targeted feature spikes, then normalize code to internal contracts/tokens before adoption.
8. Require provenance metadata and acceptance evidence for each admitted component.

## 5. Component Acceptance Checklist (Inline COMPONENT_ACCEPTANCE_CHECKLIST.md)

Every component (reused, forked, or rewritten) must pass this checklist before being marked accepted.

Contract checks:
- Focus behavior:
  - Predictable focus entry/exit, focus trap where modal, focus return on dismiss.
- Keyboard model:
  - Arrow/tab/escape/enter semantics align with component type.
  - Shortcut handling is explicit and tested.
- Pointer behavior:
  - Click, hover, drag, and outside-click dismissal behaviors are defined and deterministic.
- State model:
  - Controlled/uncontrolled behavior (if applicable), selected/active/disabled/error/open states documented.
- Disabled/read-only semantics:
  - Disabled blocks interaction and communicates state visually.
  - Read-only allows focus where appropriate but blocks mutation.

Design/token checks:
- Component surfaces and colors are mapped to frozen Zed One tokens (dark + light).
- No hard-coded colors outside approved token exceptions.

Performance gates:
- Run release-mode measurements using Zed-style workflows (`ZED_MEASUREMENTS=1` baseline and profile comparisons).
- No avoidable regressions versus baseline implementation for interaction latency/frame behavior.
- Virtualized structures (table/list/tree/command results) must demonstrate bounded rendering under large datasets.

Quality gates:
- Story/state matrix coverage exists in workbench.
- Accessibility/interaction tests exist for critical keyboard and focus flows.
- Provenance metadata is complete for copied/adapted code.

Disposition rules:
- `Reuse` if all checks pass without modification.
- `Fork` if behavior passes but token/styling/contract integration needs local adaptation.
- `Rewrite` if interaction semantics or perf gates fail.

## 6. Scope

### 6.1 MVP In Scope

- CLI:
  - `add`, `update`, `remove`, `plan`, `apply`, `doctor`, `list`.
  - JSON output for agent workflows.
  - Idempotent operations and deterministic patch planning.
  - Local registry contract to support deterministic add/upgrade flows.
- Workbench:
  - Component explorer/viewer.
  - Story rendering with state matrix support.
  - Theme switch + token editor + theme import/export.
- Component library:
  - Core-12 components:
    - Button, Input, Textarea, Select, Checkbox, Radio, Dialog, Tooltip, Tabs, DropdownMenu, Popover, Toast.
- Governance:
  - Provenance tracking, ADR log, phase gates, risk register.

### 6.2 MVP Out of Scope

- Multi-template support for `gpui init` at launch.
- Full visual regression cloud pipeline and goldens at massive scale.
- Team workflow features such as approvals/roles in the workbench.
- Remote marketplace/registry service.

### 6.3 Delivery Phasing (Authoritative)

- Phase 1 component scope is exactly the Core-12 listed in Section 10.1.
- Phase 2 component scope is the full remaining GPUI component library listed in Section 10.2, plus command palette in Section 10.3.
- The complete planned library surface is therefore: Core-12 (Phase 1) + remaining inventory (Phase 2).

## 7. Architecture

### 7.1 Monorepo Layout

```text
gpui-workbench/
  apps/
    studio/                 # Workbench binary
    cli/                    # gpui CLI binary
  crates/
    components/             # Styled components (public install targets)
    primitives/             # Unstyled behavior engines
    registry/               # Rust metadata extraction and component index
    theme/                  # Token types, theme loading, import/export
    story/                  # Story traits and registration
    assets/                 # Fonts/icons/static resources
  templates/                # Bootstrap templates and adapter contracts
  docs/
    ARCHITECTURE.md
    CONTRACTS.md
    PROVENANCE.md
    ADR/
  .planning/
    plan.md
```

### 7.2 Required Internal Contracts

- `TemplateAdapter`:
  - Abstracts file paths and insertion points for installer/scaffolder logic.
  - Starts aligned with create-gpui-app layout; extensible to future templates.
- `ComponentContract`:
  - Required metadata for installability and workbench validation.
  - Includes props, states, tokens, interaction checklist hooks, performance evidence links, and required files.
  - Standard API conventions for installable components:
    - Builder-pattern composition for predictable usage.
    - Shared identifiers and agent hooks (`id`, `tooltip`, optional `metadata` map).
    - Explicit controlled vs uncontrolled state behavior for stateful components.
- `PlanContract`:
  - Canonical JSON schema for `plan` and `apply` payloads.
  - Must include deterministic file mutation list and provenance actions.
- `RegistryContract`:
  - Local component registry format and versioning metadata for deterministic add/upgrade/remove.

### 7.3 Default Target App Layout (CLI Install Target)

When installing components into downstream apps, the default target shape is a feature-first vertical slice layout:

```text
src/
  app/                    # App shell, routing, global state
  shared/
    ui/                   # Shared install targets for reusable components
    theme/                # Theme tokens and theme registry
    utils/                # Shared utilities
  features/
    <feature_name>/
      ui/                 # Feature-specific views
      domain/             # Logic, types, state machines
      data/               # Adapters and fixtures
      tests/              # Feature integration tests
```

Default CLI install behavior in this layout:
- Create component source under `src/shared/ui/<component>/`.
- Update `src/shared/ui/mod.rs` exports deterministically.
- Inject required theme tokens into shared theme token files via deterministic mutations.

### 7.4 Machine-Readable Schema Examples

Reference example for registry entries (illustrative):

```json
{
  "name": "button",
  "version": "1.0.0",
  "variants": ["primary", "secondary", "ghost"],
  "states": ["hover", "active", "focused", "disabled"],
  "props": {
    "label": "string",
    "size": "enum[sm, md, lg]"
  },
  "files": ["src/shared/ui/button/button.rs", "src/shared/ui/button/mod.rs"]
}
```

Reference example for `--plan` output (illustrative):

```json
{
  "operation": "add",
  "component": "button",
  "mutations": [
    { "action": "create", "path": "src/shared/ui/button/mod.rs" },
    { "action": "modify", "path": "src/shared/ui/mod.rs", "strategy": "append_export" }
  ],
  "conflicts": []
}
```

## 8. Functional Requirements (FR)

- FR-001: CLI mutation commands shall run in plan-first mode by default.
- FR-002: CLI shall require explicit apply step to mutate files.
- FR-003: CLI shall produce machine-readable JSON for all major commands.
- FR-004: CLI shall support idempotent re-runs of add/update/remove operations.
- FR-005: CLI shall capture provenance metadata for copied/adapted files.
- FR-006: Registry shall be generated from Rust source metadata, not hand-maintained manifests.
- FR-007: Workbench shall render every CLI-installable component in story form.
- FR-008: Workbench shall support live theme token edits with immediate preview updates.
- FR-009: Workbench shall support theme import/export in JSON and TOML.
- FR-010: System shall ship Core-12 as Phase 1 before any Phase 2 component expansions.
- FR-011: Primitive extraction shall occur only when at least two components share behavior.
- FR-012: `gpui doctor` shall verify compatibility and integrity of target apps.
- FR-013: `gpui init` shall be implemented through template adapters.
- FR-014: All accepted components shall pass the inline Component Acceptance Checklist in this plan.
- FR-015: Local registry/CLI shall support deterministic component add/upgrade semantics similar to shadcn workflow.
- FR-016: `plan` output shall provide enough deterministic mutation detail for an agent to predict the resulting file tree.
- FR-017: Stateful components shall document and test controlled versus uncontrolled behavior in their contract metadata.
- FR-018: Component contracts shall include explicit keyboard semantics for `Tab`, `Enter/Space`, arrow navigation (where applicable), and `Escape` dismissal (where applicable).

## 9. Non-Functional Requirements (NFR)

- NFR-001 Determinism: identical inputs (seed, theme, component version) shall yield identical plans and output trees.
- NFR-002 Reliability: CLI apply failures shall be recoverable and leave a clear post-failure state report.
- NFR-003 Performance: plan generation for a single component install should complete in sub-second to low-second range on a typical dev machine.
- NFR-004 Usability (solo dev): every phase must be executable in small vertical slices (1-3 day units).
- NFR-005 Governance: no phase progression without passing its exit criteria and updating ADR/provenance records.
- NFR-006 Portability: Cargo install path is first-class; Homebrew is secondary until Cargo path is stable.
- NFR-007 Performance gates: component acceptance requires release-mode measurements and no unapproved regressions against baseline.
- NFR-008 Interaction latency budget: primary hover/click/keyboard interaction feedback should complete within one 60Hz frame budget (~16.6ms) on reference hardware.
- NFR-009 Theme switching budget: workbench theme/token switch should complete within 100ms on reference hardware.
- NFR-010 Build overhead budget: component registration/registry generation should not add more than ~2 seconds to typical debug iteration loops on reference hardware.
- NFR-011 Platform posture: first-class runtime target is macOS, with architecture and contracts kept portable for Linux/Windows follow-on.

## 10. Component Adoption Backlog

### 10.1 Phase 1: Core-12 Adoption Matrix (Comprehensive)

| Core-12 Component | Zed/GPUI reference candidates | `gpui-component-main` candidates | Recommendation | Why this disposition | Phase |
| --- | --- | --- | --- | --- | --- |
| Button | `crates/ui/src/components/button.rs` | `crates/ui/src/button/button.rs` | Fork | Behavior is mature in both; fork to lock Zed token mapping and internal contract shape | Phase 1 |
| Input | `crates/ui_input/src/input_field.rs`, `crates/gpui/examples/input.rs` | `crates/ui/src/input/input.rs`, `crates/ui/src/input/state.rs` | Fork | `gpui-component` has richer editing behavior; align focus/keyboard semantics to Zed patterns | Phase 1 |
| Textarea | `crates/gpui/examples/input.rs` (pattern source) | `crates/ui/src/input/state.rs` (multiline model) | Fork | Reuse multiline behavior, then map tokens and readonly/disabled semantics to checklist | Phase 1 |
| Select | `crates/ui/src/components/dropdown_menu.rs`, `crates/ui/src/components/popover.rs` | `crates/ui/src/select.rs` | Fork | Use existing select implementation; normalize open/close/focus semantics to Radix/Base contract | Phase 1 |
| Checkbox | `crates/ui/src/components/toggle.rs` | `crates/ui/src/checkbox.rs` | Fork | Zed toggle behavior is solid; keep gpui-component as fallback reference | Phase 1 |
| Radio | `crates/ui/src/components/toggle.rs` (group/state pattern only) | `crates/ui/src/radio.rs` | Fork | No direct Zed radio component; use gpui-component base with checklist-driven group keyboard model | Phase 1 |
| Dialog | `crates/ui/src/components/modal.rs`, `crates/ui/src/components/notification/alert_modal.rs` | `crates/ui/src/dialog.rs` | Fork | Merge Zed modal shell semantics with reusable dialog component API | Phase 1 |
| Tooltip | `crates/ui/src/components/tooltip.rs` | `crates/ui/src/tooltip.rs` | Reuse (with token map) | Behavior is straightforward; adopt fastest path if checklist passes unchanged | Phase 1 |
| Tabs | `crates/ui/src/components/tab.rs`, `crates/ui/src/components/tab_bar.rs` | `crates/ui/src/tab/tab.rs`, `crates/ui/src/tab/tab_bar.rs` | Fork | Zed tab ergonomics + gpui-component breadth; consolidate into single contract | Phase 1 |
| DropdownMenu | `crates/ui/src/components/dropdown_menu.rs`, `crates/ui/src/components/popover_menu.rs` | `crates/ui/src/menu/dropdown_menu.rs`, `crates/ui/src/menu/popup_menu.rs` | Fork | Zed menu interaction model is strong; keep gpui-component API convenience where compatible | Phase 1 |
| Popover | `crates/ui/src/components/popover.rs` | `crates/ui/src/popover.rs` | Fork | Enforce dismiss/focus contracts and token consistency | Phase 1 |
| Toast | `crates/ui/src/components/notification/announcement_toast.rs` | `crates/ui/src/notification.rs` | Fork | `gpui-component` is more general-purpose; borrow Zed visual semantics where needed | Phase 1 |

### 10.2 Phase 2: Full Remaining GPUI Component Library

Policy:
- After Phase 1 Core-12 completion, all remaining exported `gpui-component-main` UI modules are Phase 2 scope.
- This section intentionally inventories the complete remainder so the entire library is explicitly planned.
- Inventory basis: `crates/ui/src/lib.rs` exports 52 modules total; Phase 1 covers the Core-12 slice, and Phase 2 covers the full remainder.

Complete Phase 2 inventory (all non-Core-12 modules from `crates/ui/src/lib.rs`):
- `accordion`
- `alert`
- `animation`
- `avatar`
- `badge`
- `breadcrumb`
- `chart`
- `clipboard`
- `collapsible`
- `color_picker`
- `description_list`
- `divider`
- `dock`
- `form`
- `group_box`
- `highlighter`
- `history`
- `hover_card`
- `kbd`
- `label`
- `link`
- `list`
- `pagination`
- `plot`
- `progress`
- `rating`
- `resizable`
- `scroll`
- `setting`
- `sheet`
- `sidebar`
- `skeleton`
- `slider`
- `spinner`
- `stepper`
- `switch`
- `table`
- `tag`
- `text`
- `theme`
- `tree`

Phase 2 remainder from partially-covered Phase 1 modules:
- `input` advanced/non-Core-12 behaviors beyond basic Input/Textarea contract
- `menu` variants and utilities beyond Core-12 `DropdownMenu`
- `notification` variants beyond Core-12 `Toast`
- `tab` utilities beyond Core-12 `Tabs`

### 10.3 Phase 2 Priority Mapping (Zed/GPUI aligned)

| Component | Zed/GPUI reference candidates | `gpui-component-main` candidates | Recommendation | Why this disposition | Phase |
| --- | --- | --- | --- | --- | --- |
| Table | `crates/ui/src/components/data_table.rs`, `crates/gpui/src/elements/uniform_list.rs` | `crates/ui/src/table/mod.rs`, `crates/ui/src/virtual_list.rs` | Fork | `gpui-component` has strong table feature breadth; apply Zed-style perf gates/virtualization checks | Phase 2 |
| Tree | `crates/ui/src/components/tree_view_item.rs`, `crates/gpui/examples/tree.rs` | `crates/ui/src/tree.rs` | Fork | Use Zed tree interaction semantics and apply gpui-component structural helpers as needed | Phase 2 |
| Command Palette | `crates/command_palette/src/command_palette.rs` | No direct equivalent | Rewrite (Zed-patterned) | Strongly app-specific behavior in Zed; implement local version using same architectural pattern | Phase 2 |

## 11. Roadmap and Phase Gates

### Phase 0: Guardrails, Tokens, and Contracts

Deliver:
- Repo skeleton, ADR templates, provenance format, JSON contracts.
- Freeze Zed One Dark/One Light token set and publish mapping table in code.
- Finalize Source-of-Truth hierarchy and acceptance checklist in this plan.

Gate:
- Buildable workspace, approved contracts, token freeze complete.

### Phase 1: Core-12 Delivery

Deliver:
- Explicit bootstrap step: fork/rewrite the Zed main canvas/app UI/window shell for GPUI Studio.
  - Minimum required output: app launches into a blank but functional workbench window (no stories required yet).
- Minimal Zed-style workbench shell with story rendering.
- Theme switch and token editor powered by frozen token set.
- Deliver all Core-12 components (Section 10.1) with checklist and perf evidence.
- CLI `add/update/remove` for Core-12 with deterministic plans.

Gate:
- Workbench app launches reliably and can preview a blank window shell/canvas without runtime errors.
- All Core-12 are installable through CLI, rendered in workbench, and accepted by checklist/perf gates.

### Phase 2: Full Library Completion and Registry Hardening

Deliver:
- Deliver full remaining GPUI library scope from Section 10.2.
- Deliver high-priority mapped items from Section 10.3 (table, tree, command palette).
- `add/update/remove/list/doctor` with JSON and deterministic plans across full library.
- Local component registry and shadcn-style deterministic upgrade flow.
- Provenance capture in install/update flows.

Gate:
- Entire planned library surface is installable, story-renderable, and phase-gated.

### Phase 3: Hardening and Pilot Adoption

Deliver:
- Shared primitives extracted from proven duplication.
- Pilot app adoption.
- Theme import/export hardening and docs polish.
- Local visual snapshot baseline and diff checks for core interaction states (non-cloud).

Gate:
- At least one external app successfully uses multiple installed components.

### Phase 4: Distribution Expansion

Deliver:
- Cargo release hardening and docs.
- Homebrew packaging after Cargo maturity criteria pass.
- `gpui init` template expansion as needed.

Gate:
- Stable install path and successful template-backed init in pilot usage.

## 12. Provenance and Legal Requirements

- Every copied/adapted file from external sources shall include:
  - source repository URL
  - source commit hash
  - source license
  - summary of local modifications
- `docs/PROVENANCE.md` shall index these mappings by crate and file.
- CI shall block merges that add adapted files without provenance metadata.

## 13. Governance Model (Solo-Friendly)

- ADR required for major architecture changes.
- Weekly risk review updates in `.planning`.
- WIP limit: max 2 active vertical slices at once.
- Every merged slice must include:
  - acceptance checklist results
  - performance gate evidence
  - provenance updates if applicable
  - contract/docs updates when interfaces change

## 14. MVP Acceptance Criteria

- AC-001: `gpui add button --plan` returns deterministic JSON mutation plan.
- AC-002: `gpui apply <plan>` installs button into a target app successfully.
- AC-003: Workbench displays installed button and updates live when a token changes.
- AC-004: Core-12 components are installable, story-renderable, and checklist-approved.
- AC-005: Re-running installs is idempotent and conflict-aware.
- AC-006: Provenance records are complete for adapted/copied assets.
- AC-007: Cargo install path is documented and works in clean environment tests.
- AC-008: Component acceptance includes release-mode performance evidence.
- AC-009: Phase 2 completes the full remaining library inventory listed in Section 10.2 and Section 10.3.
- AC-010: For a representative install (`gpui add button --plan`), an agent can reconstruct the resulting file mutation set from JSON alone.
- AC-011: Workbench token/theme changes update previews within target budget on reference hardware.

## 15. Citations and Reference Index

Primary repos:
- Zed: <https://github.com/zed-industries/zed>
- GPUI crate docs entrypoint: <https://www.gpui.rs>
- create-gpui-app: <https://github.com/zed-industries/create-gpui-app>
- gpui-component: <https://github.com/longbridge/gpui-component>
- shadcn/ui: <https://github.com/shadcn-ui/ui>
- Radix Primitives: <https://github.com/radix-ui/primitives>
- Base UI: <https://github.com/mui/base-ui>
- awesome-gpui: <https://github.com/longbridge/awesome-gpui>

Local reference files used for this plan:
- Legacy plan carryover source:
  - `/Volumes/CORE/dev/projects/gpui-workbench/.planning/gpui-studio-master-plan.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/gpui/README.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/docs/src/development.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/docs/src/performance.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/docs/src/appearance.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/settings_content/src/theme.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/assets/themes/one/one.json`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/button.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/dropdown_menu.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/popover.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tooltip.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tab.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tab_bar.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/modal.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/toggle.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/tree_view_item.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/data_table.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui/src/components/notification/announcement_toast.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/ui_input/src/input_field.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/command_palette/src/command_palette.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/gpui/examples/input.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/zed-main/crates/gpui/examples/tree.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/create-gpui-app-main/README.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/create-gpui-app-main/templates/default/src/main.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/README.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/lib.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/schema.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/default-theme.json`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/button/button.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/input/input.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/input/state.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/select.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/checkbox.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/radio.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/dialog.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tooltip.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tab/tab.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tab/tab_bar.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/menu/dropdown_menu.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/menu/popup_menu.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/popover.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/notification.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/table/mod.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/tree.rs`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/base-ui/README.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/base-ui/packages/react/src/switch/root/SwitchRoot.tsx`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/primitives/README.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/primitives/packages/react/progress/src/progress.tsx`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/ui/packages/shadcn/README.md`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts`
- `/Volumes/CORE/dev/projects/gpui-workbench/.refs/zed_gpui_refs/awesome-gpui-main/README.md`

## 16. Requested Follow-Ons Now Covered

- The concrete component adoption backlog for button, input, select, dialog, table, tree, and command palette is included in Section 10.
- `SOURCE_OF_TRUTH.md` and `COMPONENT_ACCEPTANCE_CHECKLIST.md` are intentionally embedded inline in Sections 3 and 5 to keep a single canonical planning document.
