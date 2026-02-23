# Architecture

## Workspace Layout

```
gpui-workbench/
├── apps/
│   ├── studio/          # Desktop workbench app (GPUI window, component explorer, theme editor)
│   └── cli/             # CLI tool (gpui add/plan/apply commands)
├── crates/
│   ├── components/      # Styled UI components (Dialog, Select, Tabs, Button, etc.)
│   ├── primitives/      # Unstyled behavior primitives (focus, keyboard, popover, state)
│   ├── theme/           # Design tokens and theme engine (load, switch, live edit)
│   ├── registry/        # Component registry (source metadata index)
│   ├── story/           # Story framework (trait-based component rendering with state matrix)
│   └── assets/          # Embedded binary assets (fonts, icons)
├── templates/           # File templates for CLI install targets
├── docs/                # Governance docs (PROVENANCE, ADRs, ARCHITECTURE, CONTRACTS)
└── specs/               # Application specifications (17 specs across 3 phases)
```

## Dependency Graph (Crates)

```
studio ──> components ──> primitives
  │            │              │
  │            ├──> theme ────┘
  │            └──> (gpui)
  ├──> story ──> components
  ├──> theme
  └── assets

cli ──> registry ──> (serde, serde_json)
  └──> (clap)
```

## Key Patterns

- **`pub fn init(cx: &mut App)`**: Centralized initialization per crate, called at app bootstrap
- **Builder pattern**: Component APIs use method chaining (matches GPUI idioms)
- **`RenderOnce` + `IntoElement`**: Components consumed on render (zero-cost), not persistent views
- **`gpui_platform::application().run()`**: App bootstrap using platform-specific entry point
- **`runtime_shaders`**: Metal shaders compiled at runtime (avoids requiring full Xcode)
