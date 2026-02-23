# Plan Contract

## Purpose
The canonical JSON schema for `plan` and `apply` payloads that enables deterministic, agent-readable file mutations when installing, updating, or removing components.

## Requirements
- Define a JSON schema for plan output containing:
  - Operation type (add, update, remove)
  - Target component name and version
  - Ordered list of file mutations, each with: action (create, modify, delete), file path, mutation strategy (e.g., append_export, insert_use, replace_section)
  - Conflict detection results (list of conflicts, empty if none)
  - Provenance actions (files requiring attribution metadata)
  - File checksums (FNV-1a) for deterministic verification [observed from code]
  - Target layout identifier [observed from code]
  - Optional content field per mutation [observed from code]
  - Optional description field per mutation [observed from code]
- Plan output must contain enough detail for an agent to reconstruct the resulting file tree from JSON alone (FR-016, AC-010)
- Plan generation for a single component install should complete in sub-second to low-second range (NFR-003)
- Identical inputs (component, version, target layout) shall yield identical plans (NFR-001)
- Apply failures shall be recoverable with a clear post-failure state report (NFR-002)
- Provide an `ApplyFailureReport` struct capturing which mutation failed, which completed, and which remain [observed from code]
- Support the default target app layout (feature-first vertical slice):
  - Component source under `src/shared/ui/<component>/`
  - Export updates to `src/shared/ui/mod.rs`
  - Token injection into shared theme token files
- Provide `scan_existing_files()` to detect files already present in the target directory for conflict detection [observed from code]
- Define `TemplateAdapter` trait with methods: `component_dir()`, `module_file()`, `export_line()`, `theme_tokens_file()` [observed from code]
- Provide `DefaultLayout` as the concrete `TemplateAdapter` implementation [observed from code]

## Constraints
- Schema defined in `crates/registry/` or a dedicated contract module
- JSON is the wire format; Rust structs are the source types (serde-serializable)
- Must be deterministic: same inputs always produce same plan
- Must support the `TemplateAdapter` abstraction for different target app layouts
- Plan does not mutate files — only `apply` does (FR-001, FR-002)

## Acceptance Criteria
1. Plan JSON schema is defined as Rust types with serde serialization
2. `gpui add <component> --plan` produces valid JSON matching the schema
3. An agent (or test) can parse the plan JSON and predict every file that will be created/modified/deleted
4. Running `gpui add <component> --plan` twice with identical inputs produces identical output
5. `gpui apply <plan>` executes the mutations and reports success/failure
6. Apply failure leaves a post-failure state report

## References
- Reference: `.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts` — shadcn add command implementation (distribution workflow reference)
- Plan Section 7.2: PlanContract description
- Plan Section 7.4: Machine-Readable Schema Examples

## Decision Rationale
- Decision: Plan-first, apply-second workflow with full JSON intermediary
- Why: Enables agent workflows (FR-003), allows human review before mutation, ensures determinism and recoverability
- Alternatives considered: Direct mutation without plan step (rejected — not deterministic, not agent-friendly, not recoverable)
