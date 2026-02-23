# Theme Engine

## Purpose
Runtime theme loading, switching, live token editing, and theme import/export that powers the workbench's visual customization and validates that components correctly consume design tokens.

## Requirements
- Load theme definitions from frozen token sets (One Dark, One Light)
- Switch between themes at runtime with all rendered components updating immediately
- Support live token editing: changing a token value updates all component previews in real time (FR-008)
- Support theme import from JSON and TOML files (FR-009)
- Support theme export to JSON and TOML files (FR-009)
- Provide a theme context/provider that components access for token resolution
- Theme switching must complete within 100ms on reference hardware (NFR-009)
- Token edits must propagate to rendered components without requiring manual refresh

## Constraints
- Lives in `crates/theme/`
- Powered by the frozen design token set (depends on `design-tokens` spec)
- Primary source of truth for visual tokens: Zed One Dark/One Light
- Secondary compatibility reference: gpui-component theme schema
- Must integrate with GPUI's rendering/layout system for live updates
- macOS first-class, architecture portable

## Acceptance Criteria
1. Theme engine loads One Dark and One Light themes from token definitions
2. Switching themes at runtime updates all rendered components within 100ms
3. Editing a token value in the workbench updates component previews in real time
4. Theme can be exported to JSON file
5. Theme can be exported to TOML file
6. Theme can be imported from JSON file
7. Theme can be imported from TOML file
8. Components access tokens through a theme context/provider (not direct token references)

## References
- Reference: `.refs/zed_gpui_refs/zed-main/crates/settings_content/src/theme.rs` — Zed theme settings
- Reference: `.refs/zed_gpui_refs/zed-main/assets/themes/one/one.json` — Zed One Dark/Light theme source
- Reference: `.refs/zed_gpui_refs/zed-main/docs/src/appearance.md` — Zed appearance documentation
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/schema.rs` — gpui-component theme schema
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/default-theme.json` — gpui-component default theme

## Decision Rationale
- Decision: Live token editing as a first-class workbench feature
- Why: Validates that the token system works end-to-end and enables rapid design iteration
- Decision: Support both JSON and TOML for import/export
- Why: JSON for machine workflows and agent interop; TOML for human-readable Rust ecosystem convention
