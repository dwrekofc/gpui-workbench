# CLI

## Purpose
The `gpui` command-line binary that provides plan-first, deterministic component installation and management for downstream GPUI applications, with JSON output for agent workflows.

## Requirements
- Implement `add` command: install a component's source files into target app
- Implement `plan` command / `--plan` flag: output deterministic JSON mutation plan without modifying files (FR-001)
- Implement `apply` command: execute a previously generated plan to mutate files (FR-002)
- Produce machine-readable JSON output for all major commands (FR-003)
- Support idempotent re-runs: running `add` for an already-installed component is conflict-aware (FR-004, AC-005)
- Capture provenance metadata for copied/adapted files during install (FR-005)
- Plan output contains deterministic file mutation list enabling agent reconstruction of resulting file tree (FR-016, AC-010)
- Default install target layout:
  - Component source under `src/shared/ui/<component>/`
  - Update `src/shared/ui/mod.rs` exports deterministically
  - Inject required theme tokens into shared theme token files
- Support `TemplateAdapter` abstraction for different target app layouts

## Constraints
- Lives in `apps/cli/`
- For Phase 0.5: only `add`, `plan`, and `apply` need to work (for Dialog, Select, Tabs)
- Plan-first by default: mutation commands show plan and require explicit apply
- Consumes registry metadata from `crates/registry/`
- Consumes plan contract schema from plan-contract spec
- Cargo install path is first-class (NFR-006)
- Plan generation sub-second to low-second range (NFR-003)
- Apply failures recoverable with post-failure state report (NFR-002)
- Identical inputs yield identical plans (NFR-001)

## Acceptance Criteria
1. `gpui add dialog --plan` outputs valid JSON matching plan contract schema
2. `gpui add select --plan` outputs valid JSON matching plan contract schema
3. `gpui add tabs --plan` outputs valid JSON matching plan contract schema
4. `gpui apply <plan-file>` installs component source into target app layout
5. Re-running `gpui add dialog` on an already-installed dialog is idempotent and conflict-aware
6. JSON output is parseable by standard tools (jq, serde)
7. Provenance metadata is written for installed component files
8. `cargo install` from the CLI crate produces a working `gpui` binary

## References
- Reference: `.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts` — shadcn add command (distribution workflow reference)
- Reference: `.refs/zed_gpui_refs/create-gpui-app-main/README.md` — create-gpui-app CLI patterns
- Plan Section 7.3: Default Target App Layout
- Plan Section 8: Functional Requirements (FR-001 through FR-006, FR-015, FR-016)

## Decision Rationale
- Decision: Plan-first by default, explicit apply step
- Why: Enables agent workflows, human review, determinism, and recoverability
- Decision: shadcn-style source-copy distribution
- Why: Gives downstream apps full ownership of component source; enables per-project customization without upstream dependency coupling
- Alternatives considered: Crate-based consumption only (available as optional path, but source-copy is the default for shadcn-style DX)
