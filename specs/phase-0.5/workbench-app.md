# Workbench App

## Purpose
The desktop GPUI application shell that serves as component explorer, story renderer, theme editor, and visual validation environment for the component library.

## Requirements
- Launch a functional GPUI window on macOS
- Provide a component explorer/viewer listing all registered components
- Render selected component stories with state matrix support
- Provide theme switching UI (toggle between One Dark / One Light)
- Provide a token editor panel for live theme token editing
- Update all component previews immediately when tokens change
- Display component metadata from contracts (props, variants, states)
- Support navigating between components in the explorer

## Constraints
- Lives in `apps/studio/`
- Built on GPUI framework (Zed's UI toolkit)
- Bootstrap pattern: fork/adapt Zed main canvas/app window shell for the studio
- Minimum first output: app launches into a blank but functional workbench window
- Depends on: `crates/theme/` (theme engine), `crates/story/` (story framework), `crates/components/` (styled components)
- macOS first-class runtime target (NFR-011)
- Must launch reliably without runtime errors

## Acceptance Criteria
1. `cargo run` from `apps/studio/` launches a GPUI window on macOS
2. Component explorer lists all registered story components
3. Selecting a component renders its story with state matrix
4. Theme switch toggles between One Dark and One Light with all components updating
5. Token editor allows editing a token value and components update in real time
6. No runtime panics or errors on launch and during basic interaction

## References
- Reference: `.refs/zed_gpui_refs/zed-main/crates/gpui/README.md` — GPUI application architecture
- Reference: `.refs/zed_gpui_refs/zed-main/docs/src/development.md` — Zed development patterns
- Reference: `.refs/zed_gpui_refs/create-gpui-app-main/templates/default/src/main.rs` — GPUI app bootstrap template
- Reference: `.refs/zed_gpui_refs/create-gpui-app-main/README.md` — create-gpui-app scaffolding

## Decision Rationale
- Decision: Fork Zed window shell as workbench bootstrap
- Why: Zed is the canonical GPUI application; starting from its patterns ensures correctness
- Alternatives considered: Build from create-gpui-app template (possible but provides less structure; Zed shell is a better starting point for a complex app)
