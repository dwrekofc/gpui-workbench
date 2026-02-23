# Design Tokens

## Purpose
A frozen set of design tokens extracted from Zed One Dark and One Light themes that serves as the single source of truth for all component colors, surfaces, and states across the system.

## Requirements
- Freeze token values from Zed One Dark theme
- Freeze token values from Zed One Light theme
- Represent tokens as Rust types in the `theme` crate
- Map all component colors, surfaces, and visual states to frozen tokens
- Prohibit hard-coded colors outside explicitly approved token exceptions
- Support semantic token categories: foreground, background, border, surface, accent, state (hover, active, focused, disabled, error)
- Provide a mapping table from Zed theme JSON keys to internal token identifiers
- Token values must be resolvable at runtime for theme switching
- Provide `parse_hex_color()` to convert `#RRGGBB`/`#RRGGBBAA` strings to GPUI color types [observed from code]
- Provide `set_token_by_path()` to mutate individual tokens at runtime by dot-path [observed from code]
- Provide `all_token_paths()` listing all ~50 addressable token dot-paths [observed from code]
- Provide `TOKEN_MAPPING` static table mapping internal dot-paths to Zed JSON keys [observed from code]
- Support theme import/export in JSON format (FR-009) [observed from code]
- Support theme import/export in TOML format (FR-009) [observed from code]
- Store `Theme` and `ThemeRegistry` as GPUI globals [observed from code]
- Provide `ActiveTheme` extension trait on `gpui::App` for `.theme()` access [observed from code]
- Provide `Theme::change(name, cx)` to switch active theme by name with window refresh [observed from code]
- Token structs include: `BorderTokens`, `SurfaceTokens`, `ElementTokens`, `GhostElementTokens`, `TextTokens`, `IconTokens`, `StatusTokens`, `TabTokens`, `PanelTokens`, `ChromeTokens`, `ScrollbarTokens`, `PlayerTokens`, `LinkTokens` [observed from code]

## Constraints
- Lives in `crates/theme/`
- Primary source of truth: Zed One Dark/One Light theme files
- Secondary source: gpui-component theme schema for compatibility reference
- Tokens must be frozen (locked values) before any component work begins
- Token types must be compatible with GPUI's styling system
- Theme switching with frozen tokens must complete within 100ms (NFR-009)
- macOS first-class, architecture portable for Linux/Windows

## Acceptance Criteria
1. Rust token types exist in `crates/theme/`
2. One Dark token values are frozen and loadable
3. One Light token values are frozen and loadable
4. No component in the codebase uses hard-coded color values outside token exceptions
5. A mapping table from Zed theme JSON keys to internal tokens exists in code or documentation
6. Token resolution supports runtime theme switching

## References
- Reference: `.refs/zed_gpui_refs/zed-main/assets/themes/one/one.json` — Zed One Dark/Light source values
- Reference: `.refs/zed_gpui_refs/zed-main/crates/settings_content/src/theme.rs` — Zed theme settings structure
- Reference: `.refs/zed_gpui_refs/zed-main/docs/src/appearance.md` — Zed appearance/theme documentation
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/schema.rs` — gpui-component theme schema (compatibility reference)
- Reference: `.refs/zed_gpui_refs/gpui-component-main/crates/ui/src/theme/default-theme.json` — gpui-component default theme values

## Decision Rationale
- Decision: Freeze Zed tokens first, then map components to them
- Why: Prevents style drift and ensures visual consistency with Zed ecosystem; provides a stable contract for component authors
- Alternatives considered: Using gpui-component token schema directly (rejected — differs from Zed defaults, would require reconciliation later)
