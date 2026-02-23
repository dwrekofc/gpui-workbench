## Build & Run

- Language: Rust (edition 2024)
- UI Framework: GPUI (from Zed)
- Build: `cargo build`
- Run: `cargo run`

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

### Codebase Patterns
