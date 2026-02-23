# ADR-001: Cargo Workspace Monorepo Layout

**Date**: 2026-02-22
**Status**: Accepted

## Context

We need a project structure that supports multiple binary applications (workbench studio, CLI tool) and multiple library crates (components, primitives, theme, registry, story, assets) with shared dependencies. The project follows the GPUI desktop UI framework from Zed.

Reference codebases (gpui-component, Zed) both use Cargo workspace monorepos with `resolver = "2"` and workspace-level dependency inheritance.

## Decision

Adopt a Cargo workspace monorepo with:

- `apps/studio/` — Desktop workbench application (GPUI)
- `apps/cli/` — Command-line tool for component management
- `crates/components/` — Styled UI components (Dialog, Select, Tabs, etc.)
- `crates/primitives/` — Unstyled behavior primitives (focus, keyboard, popover, state)
- `crates/theme/` — Design tokens and theme engine
- `crates/registry/` — Component registry and metadata index
- `crates/story/` — Story framework for component rendering
- `crates/assets/` — Embedded binary assets (fonts, icons)
- `templates/` — File templates for CLI install targets (non-crate)

Workspace configuration:
- `default-members = ["apps/studio"]` so bare `cargo build` builds only the workbench
- `resolver = "2"` for correct feature unification
- GPUI pinned to a known-working Zed revision (matching gpui-component)
- Dev profile optimizes heavy rendering crates at `opt-level = 3`

## Consequences

- **Easier**: Shared dependency versions, unified build, cross-crate type sharing
- **Easier**: `cargo build --workspace` validates everything compiles together
- **Harder**: Adding a new crate requires updating `[workspace]` members list
- **Trade-off**: Pinning GPUI to a specific Zed revision avoids upstream breakage but requires manual updates
