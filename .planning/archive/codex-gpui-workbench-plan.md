# GPUI Workbench Plan (Interview-Aligned)

## 1) Locked Decisions

- Build model: `Dual-track`
  - CLI and workbench progress in parallel.
  - Priority rule: `CLI leads, workbench validates`.
- Consumption model: `Hybrid`
  - Default source-copy installs (shadcn-like), optional crate-based consumption path.
- Foundation: `Clean-room primitives`
  - Start with a minimal Zed-style workbench shell.
  - Use pattern-clone and selective fork approaches where useful.
  - Incorporate new components by pattern-cloning proven `gpui-component` patterns.
- Initial component set (MVP library): `Core-12`
  - Button, Input, Textarea, Select, Checkbox, Radio, Dialog, Tooltip, Tabs, DropdownMenu, Popover, Toast.
- Repository shape: `One monorepo`.
- Provenance policy: `Strict provenance`
  - Track copied/adapted code with source, commit, license, and modifications.
- Registry/source of truth: `Rust code first`.
- CLI mutation behavior: `Plan-first default`
  - `--plan` style output by default; explicit `--apply` to mutate files.
- Distribution priority: `Cargo-first`.
- Workbench MVP boundary: `Viewer + token editor`.
- Template base for `gpui init`: `Deferred`.

---

## 2) North-Star Architecture

Single monorepo containing:

- `gpui` CLI (agent-friendly planning/apply pipeline).
- Workbench desktop app (component explorer + token editor + state validation).
- Clean-room primitives + pre-styled components.
- Rust-native registry/metadata pipeline (generated from code).
- Templates and install mappings (template decision deferred, abstraction added now).

### Core architectural constraints

- Deterministic plan/apply operations.
- Idempotent component install/update/remove.
- Strict provenance and licensing hygiene.
- Template abstraction to avoid locking into one bootstrap strategy now.

---

## 3) Execution Plan (Phased)

## Phase 0: Guardrails and Standards

Goals:
- Define boundaries and rules before building features.

Deliverables:
- Monorepo conventions doc.
- Provenance format (`PROVENANCE.md`) and attribution header standard.
- Component contract template (props, states, tokens, accessibility notes).
- CLI JSON output contract (`plan`, `apply`, `doctor`).

Exit criteria:
- All future work references one shared standards doc.

## Phase 1: Skeleton (No heavy feature work yet)

Goals:
- Establish repo skeleton and crate boundaries that match decisions.

Deliverables:
- Workspace with crates/apps for:
  - CLI app
  - Workbench app
  - Schema/registry core
  - Primitives
  - Components
- Shared error/result types and logging conventions.
- Placeholder template abstraction interfaces.

Exit criteria:
- Workspace builds.
- No hardcoded assumptions about one app template.

## Phase 2: CLI Core (Priority Track)

Goals:
- Ship agent-friendly workflow early.

Deliverables:
- `gpui add <component>` with plan-first behavior.
- `gpui apply <plan>` execution with post-state summary.
- `gpui update`, `gpui remove` basic support.
- Registry ingestion from Rust source metadata.
- Provenance capture during installation.

Exit criteria:
- Core-2 components install end-to-end via plan/apply.
- Re-running same operation is idempotent.

## Phase 3: Workbench MVP Shell

Goals:
- Validate component quality and theme behavior quickly.

Deliverables:
- Minimal Zed-style shell: sidebar, component list, preview pane.
- Story/state viewer for registered components.
- Token editor (live), theme switcher, theme import/export.
- State matrix view and basic snapshot capture.

Exit criteria:
- CLI-installed components render correctly in workbench.
- Token changes reflect live across stories.

## Phase 4: Core-12 Components

Goals:
- Build a usable initial library with clean-room primitives.

Deliverables:
- Core-12 components with:
  - Story coverage
  - State matrices
  - Token wiring
  - Accessibility checklist notes
- CLI install mappings for each component.

Exit criteria:
- All 12 components install and render consistently.
- Baseline snapshot set captured.

## Phase 5: Packaging + Early Adoption

Goals:
- Make it usable in real downstream apps.

Deliverables:
- Cargo distribution hardening.
- `gpui doctor` checks for app structure compatibility.
- Hybrid consumption docs (source-copy default + crate option).
- “Pattern clone with attribution” playbook.

Exit criteria:
- At least one downstream app successfully consumes multiple components.

---

## 4) Monorepo Scaffold Blueprint (Planned)

```text
gpui-workbench/
  apps/
    gpui-cli/
    gpui-workbench/
  crates/
    gpuiwb-schema/
    gpuiwb-registry/
    gpuiwb-primitives/
    gpuiwb-components/
  templates/
    (deferred concrete base, keep abstraction now)
  provenance/
    PROVENANCE.md
  docs/
    architecture.md
    contracts.md
    cli-json-contract.md
    roadmap.md
```

Note: this is a target blueprint, not executed yet.

---

## 5) Immediate Task Queue (First 2 Iterations)

Iteration 1 (foundation):
- Write architecture/contracts/provenance docs.
- Create workspace skeleton.
- Implement registry model and component metadata traits.
- Implement CLI `plan` plumbing for one component (`button`).

Iteration 2 (vertical slice):
- Implement `add button --plan` + `--apply`.
- Add workbench viewer for `button` + theme switch + token editing.
- Add snapshot capture baseline for one component.
- Validate idempotency and provenance output.

---

## 6) Deferred Decisions

- `gpui init` template base:
  - Option still open (`create-gpui-app + overlays` vs `Zed-derived` vs clean-room template).
  - Keep template abstraction until decision is made.

---

## 7) Risks and Mitigations

- Risk: fork debt from deep Zed reuse.
  - Mitigation: prefer pattern-clone by default; use selective fork with strict provenance.
- Risk: drift between CLI behavior and workbench rendering.
  - Mitigation: registry-driven single source of truth + shared schema crate.
- Risk: install conflicts in downstream apps.
  - Mitigation: deterministic plan output, conflict detection, explicit apply step.
- Risk: over-scoping early workbench features.
  - Mitigation: enforce MVP boundary (viewer + token editor only).

---

## 8) Definition of Success (MVP)

- `gpui` CLI can plan and apply component installation safely and repeatedly.
- Workbench can visualize component states and live token changes.
- Core-12 components exist with consistent tokenized styling and install mappings.
- Provenance records are complete for copied/adapted assets.
- At least one external app consumes components through the hybrid model.
