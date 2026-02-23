# Workbench

## Purpose
The desktop GPUI application (Studio) that serves as the visual environment for browsing, previewing, and validating components with live theme editing and metadata inspection.

## Requirements
- Render every CLI-installable component in story form (FR-007)
- Support live theme token edits with immediate preview updates (FR-008)
- Support theme import/export in JSON and TOML formats (FR-009)
- Provide a sidebar for browsing and selecting component stories [observed from code]
- Provide a toolbar with theme toggle (Dark/Light), token editor toggle, and metadata panel toggle [observed from code]
- Provide a token editor panel that lists all token paths grouped by category, displays color swatches, and allows inline hex editing [observed from code]
- Provide a metadata panel showing component contract details: props, states, interaction checklist, token dependencies [observed from code]
- Display the current theme name in the sidebar [observed from code]
- Launch reliably on macOS into a functional window (Phase 1 gate requirement)
- Support window size of 1280x800 as default [observed from code]

## Constraints
- Binary crate lives in `apps/studio/`
- Depends on all library crates: theme, primitives, components, story, assets
- Uses GPUI runtime via `gpui_platform::application().run()`
- macOS is first-class runtime target (NFR-011)
- Theme/token switch must complete within 100ms on reference hardware (NFR-009)
- Token editor panel is 280px wide [observed from code]
- Sidebar is 220px wide [observed from code]

## Acceptance Criteria
1. Workbench app launches on macOS without runtime errors
2. All 12 Core-12 component stories are navigable from the sidebar
3. Clicking a story renders it in the content area
4. Toggling Dark/Light theme updates all component previews immediately
5. Editing a token hex value in the token editor updates component previews live (AC-003, AC-011)
6. Metadata panel displays contract info for the selected component
7. Theme import/export round-trips through JSON and TOML without data loss

## References
- Reqs Section 6.1: MVP In Scope — Workbench
- Reqs Section 8: FR-007, FR-008, FR-009
- Reqs Section 14: AC-003, AC-011
