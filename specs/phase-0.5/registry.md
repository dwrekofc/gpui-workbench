# Registry

## Purpose
A local component registry generated from Rust source metadata that indexes all installable components for deterministic CLI add/upgrade/remove operations.

## Requirements
- Generate registry from Rust source metadata, not hand-maintained manifests (FR-006)
- Index each component with: name, version, variants, states, props (with types), required files
- Support deterministic versioning for component add/upgrade semantics
- Registry generation must not add more than ~2 seconds to typical debug iteration loops (NFR-010)
- Provide lookup API for CLI to resolve component metadata during plan generation
- Support listing all registered components with metadata summary

## Constraints
- Lives in `crates/registry/`
- Registry is Rust-first: source types are Rust structs, serializable to JSON via serde
- Reads `ComponentContract` metadata from component source to populate entries
- Must be regenerable from source at any time (no stale cache as source of truth)
- Local-only — no remote marketplace or registry service in scope

## Acceptance Criteria
1. Registry crate exists at `crates/registry/`
2. Registry generates entries from component contract metadata in source
3. All 3 POC components (Dialog, Select, Tabs) are indexed in the registry
4. Registry entries contain name, version, variants, states, props, and required files
5. CLI can query registry to resolve a component by name
6. Registry regeneration completes within 2 seconds on reference hardware

## References
- Reference: `.refs/zed_gpui_refs/ui/packages/shadcn/src/commands/add.ts` — shadcn registry/add workflow reference
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/lib.rs` — gpui-component module inventory (structure reference)
- Plan Section 7.2: RegistryContract description
- Plan Section 7.4: Machine-Readable Schema Examples

## Decision Rationale
- Decision: Generated from source, not hand-maintained
- Why: Eliminates drift between code and registry; ensures single source of truth
- Decision: Local-only registry
- Why: Remote marketplace is out of MVP scope; local registry satisfies all deterministic CLI requirements
