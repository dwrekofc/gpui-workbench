# Registry

## Purpose
The local component index that is generated from Rust source metadata, providing the authoritative catalog of available components, their contracts, and versioning information for deterministic CLI operations.

## Requirements
- Generate registry entries from Rust source metadata, not hand-maintained manifests (FR-006)
- Support deterministic component add/upgrade/remove semantics (FR-015)
- Store per-component metadata: name, version, disposition, variants, states, props, token dependencies, required files [observed from code]
- Provide case-insensitive component lookup [observed from code]
- Provide sorted listing of all registered components [observed from code]
- Support JSON serialization/deserialization of the full registry index [observed from code]
- Validate all component contracts during registry generation [observed from code]
- Enumerate all component contracts via `all_contracts()` function [observed from code]
- Registry metadata remains Rust-first and generated from source

## Constraints
- Lives in `crates/registry/`
- Registry entries are derived from `ComponentContract` structs in `crates/components/`
- JSON is the serialization format (serde-serializable Rust types)
- Registration must not add more than ~2 seconds to debug iteration loops (NFR-010)
- Registry is local — no remote marketplace/registry service in MVP

## Acceptance Criteria
1. `generate_registry()` produces a `RegistryIndex` with entries for all 12 Core-12 components
2. `generate_registry_validated()` returns errors for any invalid contracts
3. Registry entries contain all fields needed by the plan engine (name, version, variants, states, props, token deps, files)
4. `RegistryIndex::to_json()` and `from_json()` round-trip without data loss
5. Case-insensitive lookup works (e.g., "Button" and "button" resolve to the same entry)

## References
- Reqs Section 7.2: RegistryContract description
- Reqs Section 7.4: Machine-Readable Schema Examples (registry entry JSON)
- Reqs Section 8: FR-006, FR-015
