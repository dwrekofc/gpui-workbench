# CLI

## Purpose
The `gpui` command-line tool that enables developers and agents to install, update, remove, scaffold, and validate components in target applications through a deterministic plan-then-apply workflow.

## Requirements
- Provide `add` command to install a component into a target app (FR-001, FR-002)
- Provide `update` command to upgrade an installed component to a newer version (FR-004)
- Provide `remove` command to uninstall a component from a target app (FR-004)
- Provide `plan` command (or `add --plan`) to preview mutations without applying them (FR-001)
- Provide `apply` command to execute a previously saved plan file (FR-002)
- Provide `list` command to show available components from the registry
- Provide `doctor` command to verify compatibility and integrity of target apps (FR-012)
- Provide `init` command to scaffold a new GPUI app using template adapters (FR-013)
- Run mutation commands in plan-first mode by default — no file changes without explicit apply (FR-001, FR-002)
- Produce machine-readable JSON output for all major commands (FR-003)
- Support idempotent re-runs of add/update/remove operations (FR-004)
- Capture provenance metadata for copied/adapted files during install/update (FR-005)
- Support `--plan` flag on `add` to output a plan without applying
- Support `-d <dir>` flag to specify target directory
- Wrap all output in a structured envelope: `{ success, data, errors }` [observed from code]
- Write `.provenance.json` files beside installed component files [observed from code]
- Detect existing files in target directory for conflict detection before apply [observed from code]

## Constraints
- Binary crate lives in `apps/cli/`
- Binary name is `gpui`
- Depends on `registry` crate only — does not depend on GPUI runtime
- JSON is the output format for agent workflows
- Plan-first, apply-second: `plan` reads; `apply` writes
- `init` must work through the `TemplateAdapter` abstraction (FR-013)
- Cargo install path is first-class; Homebrew is secondary (NFR-006)

## Acceptance Criteria
1. `gpui add button --plan` returns deterministic JSON mutation plan (AC-001)
2. `gpui apply <plan>` installs button into a target app successfully (AC-002)
3. Re-running `gpui add button` on an already-installed button is idempotent (AC-005)
4. `gpui list` outputs JSON listing all registered components
5. `gpui doctor` reports compatibility status of a target app
6. `gpui init` scaffolds a new app using a template adapter
7. All commands produce JSON-structured output parseable by agents
8. Provenance records are written for all installed files (AC-006)
9. Cargo install path is documented and works in clean environment (AC-007)

## References
- Reference: `.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts` — shadcn add command (distribution workflow reference)
- Reqs Section 6.1: MVP In Scope — CLI commands
- Reqs Section 8: FR-001 through FR-005, FR-012, FR-013, FR-015
- Reqs Section 14: AC-001, AC-002, AC-005, AC-006, AC-007, AC-010
