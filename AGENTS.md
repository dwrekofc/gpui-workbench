## Build & Run

- Language: Rust (edition 2024)
- UI Framework: GPUI (from Zed, pinned to rev `d08d98f`)
- Build: `cargo build` (default-members: studio app only), `cargo build --workspace` (all crates)
- Run: `cargo run` (launches studio workbench app)
- GPUI app bootstrap: `gpui_platform::application().run()` (NOT `Application::new()` which doesn't exist at this rev)
- Metal shaders: `runtime_shaders` feature compiles shaders at runtime (no full Xcode needed)
- If `core-text` version conflict occurs: `cargo update core-text@21.1.0 --precise 21.0.0`

## Validation

- Tests: `cargo nextest run` (fallback: `cargo test`)
- Clippy: `cargo clippy --all-targets -- -D warnings`
- Format check: `cargo fmt --all -- --check`

## Reference Material

See `.refs/zed_gpui_refs/` for reference codebases and examples:

- **awesome-gpui-main** — Curated list of GPUI apps, libraries, and resources
- **base-ui** — Unstyled, accessible React component library (from Radix/MUI creators); design pattern reference
- **create-gpui-app-main** — Scaffolding tool and starter template for new GPUI apps
- **gpui-component-main** — 60+ pre-built GPUI components (buttons, inputs, tables, docks, charts, theming)
- **primitives** — Radix Primitives; accessible low-level UI components for web; design pattern reference
- **ui** — shadcn/ui; styled React component library; design pattern reference
- **zed-main** — Zed editor source; flagship GPUI application and reference implementation

## Operational Notes

### GPUI API at Pinned Rev (d08d98f)

- `Render` trait: `fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement`
- Window open: `cx.open_window(opts, |_window, cx| cx.new(|_cx| MyView))`
- Init pattern: `pub fn init(cx: &mut gpui::App)` per crate
- Provenance check: `scripts/check-provenance.sh`

### Codebase Patterns

- `FluentBuilder` import: Components using `.when()` must `use gpui::prelude::FluentBuilder`
- Flex helpers: No `v_flex()`/`h_flex()` in GPUI — use `div().flex().flex_col()` / `div().flex().flex_row()`
- Test stack overflow: GPUI `IntoElement` derives cause stack overflow in test compilation. Use integration tests (`tests/`) for crates with `#[derive(IntoElement)]`. Set `#![recursion_limit = "2048"]`.
- Component pattern: POC components use `#[derive(IntoElement)]` + `impl RenderOnce` (stateless, consumed on render)
