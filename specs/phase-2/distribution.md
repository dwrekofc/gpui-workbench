# Distribution

## Purpose
Packaging, installation paths, template expansion, and pilot adoption that take the component library from a development tool to a distributable product.

## Requirements
- Stabilize Cargo install path: `cargo install gpui-cli` (or equivalent) produces a working `gpui` binary
- Document Cargo install path with clean-environment verification
- Implement Homebrew formula/tap after Cargo path reaches maturity criteria
- Expand `gpui init` to support multiple templates beyond the default
- Support pilot adoption: at least one external app successfully uses multiple installed components
- Theme import/export hardened for production use (JSON and TOML, round-trip fidelity)
- Local visual snapshot baseline and diff checks for core interaction states (non-cloud)
- Documentation polish: README, getting started guide, component API docs

## Constraints
- Cargo install is first-class; Homebrew is secondary until Cargo is stable (NFR-006)
- No remote marketplace or registry service (out of scope)
- Multi-template `gpui init` was out of MVP scope but is Phase 2 delivery
- Visual snapshot checks are local-only, not cloud-based
- Phase 1 gate must be met before distribution work begins

## Acceptance Criteria
1. `cargo install` from the CLI crate produces a working `gpui` binary in a clean environment
2. Cargo install path is documented with step-by-step instructions
3. Homebrew formula exists and installs a working `gpui` binary
4. `gpui init` supports at least 2 templates (default + one additional)
5. At least one external app successfully installs and uses 3+ components via `gpui add`
6. Theme export → import round-trips produce identical token values
7. Local visual snapshot baseline exists for core component states

## References
- Reference: `.refs/zed_gpui_refs/create-gpui-app-main/README.md` — create-gpui-app distribution patterns
- Reference: `.refs/zed_gpui_refs/ui/packages/shadcn/README.md` — shadcn distribution and install UX
- Plan Section 11: Phase 4 (Distribution Expansion)
- Plan Section 14: AC-007 (Cargo install), AC-009 (full library)

## Decision Rationale
- Decision: Cargo first, Homebrew second
- Why: Cargo is native to the Rust ecosystem and avoids external packaging complexity until the tool is stable
- Decision: Local visual snapshots, not cloud
- Why: Cloud visual regression is out of MVP scope; local baselines provide sufficient coverage for solo/small-team workflows
