# CLI Hardening

## Purpose
Complete the CLI command surface beyond Phase 0.5's `add/plan/apply` with `update`, `remove`, `list`, `doctor`, and `init` commands, plus deterministic upgrade flow across all Core-12.

## Requirements
- Implement `update` command: upgrade an installed component to latest version, preserving local modifications where possible
- Implement `remove` command: uninstall a component's source files and clean up exports
- Implement `list` command: show all installed components in target app with version/status
- Implement `doctor` command: verify compatibility and integrity of target app (FR-012)
  - Check that required dependencies exist
  - Validate installed component files match expected state
  - Report health/issues in human and JSON format
- Implement `init` command: scaffold a new GPUI app through template adapters (FR-013)
  - Use `TemplateAdapter` abstraction
  - Align with create-gpui-app layout as default template
- All commands support JSON output for agent workflows (FR-003)
- All mutation commands (update, remove) run in plan-first mode by default (FR-001, FR-002)
- All mutation commands are idempotent (FR-004)
- Full deterministic upgrade flow: `gpui update <component>` produces a plan showing exactly what will change

## Constraints
- Lives in `apps/cli/`
- Phase 1 gate must be met for CLI hardening to be considered complete
- All 12 Core components must be manageable through CLI
- `init` starts with single default template; multi-template is Phase 2+ scope
- Cargo install path first-class (NFR-006)

## Acceptance Criteria
1. `gpui update <component> --plan` shows deterministic upgrade plan
2. `gpui update <component>` applies the upgrade
3. `gpui remove <component> --plan` shows removal plan
4. `gpui remove <component>` removes component files and cleans exports
5. `gpui list` shows all installed components with status
6. `gpui list --json` outputs machine-readable JSON
7. `gpui doctor` reports target app health and any issues
8. `gpui init` scaffolds a new GPUI app from default template
9. All commands work correctly for all 12 Core components
10. Re-running any mutation command is idempotent

## References
- Reference: `.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts` — shadcn CLI command patterns
- Reference: `.refs/zed_gpui_refs/create-gpui-app-main/README.md` — create-gpui-app init patterns
- Plan Section 6.1: CLI scope (add, update, remove, plan, apply, doctor, list)
- Plan Section 8: FR-001 through FR-006, FR-012, FR-013, FR-015
