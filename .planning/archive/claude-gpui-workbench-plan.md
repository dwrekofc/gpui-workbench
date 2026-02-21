# GPUI Studio — Project Roadmap & Repo Scaffold

## Context

**Problem**: There is no Rust/GPUI equivalent of the React component library ecosystem (Radix primitives + shadcn styled components + Storybook workbench). Building GPUI apps today means either writing UI from scratch or pulling from scattered, tightly-coupled examples.

**Goal**: Build a layered GPUI component library with a visual workbench for developing, testing, and showcasing components — modeled after React best practices but fully native Rust.

**Approach**: Start with a minimal workbench app based on Zed's proven UI patterns. Build components directly into it so every addition is immediately visible and interactive. Extract behavioral primitives as shared patterns naturally emerge. Add theming and CLI distribution later.

**GPUI dependency**: Git dependency on the Zed repo (`gpui = { git = "https://github.com/zed-industries/zed" }`). This is what gpui-component and create-gpui-app both use — the published crate (0.2.2) lags behind.

---

## Repo Structure

```
gpui-studio/
├── Cargo.toml                     # Workspace root
├── rust-toolchain.toml
├── CLAUDE.md
├── README.md
│
├── crates/
│   ├── studio/                    # Workbench GUI app (binary)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs            # Entry point, window setup
│   │       ├── app.rs             # App state, init cascade
│   │       ├── gallery.rs         # Sidebar + main panel layout
│   │       ├── title_bar.rs       # Title bar with theme toggle
│   │       ├── story_container.rs # Renders selected story
│   │       └── stories/           # One file per component story
│   │           ├── mod.rs
│   │           ├── welcome_story.rs
│   │           ├── button_story.rs
│   │           └── ...
│   │
│   ├── components/                # Component library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # Public API, re-exports, init()
│   │       ├── traits.rs          # Disableable, Selectable, Sizable
│   │       ├── styled.rs          # h_flex, v_flex, layout helpers
│   │       ├── size.rs            # Size enum
│   │       ├── button/
│   │       │   ├── mod.rs
│   │       │   └── button.rs
│   │       ├── input/
│   │       │   ├── mod.rs
│   │       │   └── input.rs
│   │       ├── label.rs
│   │       ├── icon.rs
│   │       ├── divider.rs
│   │       └── ...
│   │
│   ├── primitives/                # Behavioral primitives (grows organically)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── (empty initially — populated in Phase 3)
│   │
│   ├── theme/                     # Theme engine
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs             # Theme struct, ActiveTheme trait
│   │   │   ├── colors.rs          # Semantic color tokens
│   │   │   └── registry.rs        # Load, switch, watch themes
│   │   └── themes/
│   │       └── default.json       # Default light/dark theme
│   │
│   ├── story/                     # Story framework (trait + registry)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # Story trait, StoryContainer, section()
│   │       └── registry.rs        # inventory-based auto-registration
│   │
│   ├── assets/                    # Icons, fonts
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   │
│   └── cli/                       # CLI tool (Phase 5, skeleton only initially)
│       ├── Cargo.toml
│       └── src/main.rs
│
└── examples/                      # Standalone example apps
    └── hello_world/
```

### Crate Dependency Graph

```
studio (bin) ──→ components ──→ primitives
    │                │               │
    │                └──→ theme ←────┘ (no theme dep for primitives)
    │
    ├──→ story ──→ components
    ├──→ theme
    └──→ assets
```

---

## Phase 1: Minimal Workbench Shell

**Goal**: A running GPUI desktop app with sidebar + main panel that renders component stories. Proves the full pipeline end-to-end.

### What to build
1. **`crates/theme/`** — Adapt gpui-component's theme system (ThemeColor, ActiveTheme trait, registry, default JSON theme). Needed immediately since all rendering uses theme tokens.
   - Reference: `zed_gpui_refs/gpui-component-main/crates/ui/src/theme/`
2. **`crates/assets/`** — Minimal asset loader for icons/fonts.
   - Reference: `zed_gpui_refs/gpui-component-main/crates/assets/`
3. **`crates/story/`** — Story trait + StoryContainer + `section()` helper + inventory-based registry.
   - Reference (story trait): `zed_gpui_refs/gpui-component-main/crates/story/src/lib.rs`
   - Reference (inventory registration): `zed_gpui_refs/zed-main/crates/component/src/component.rs`
4. **`crates/studio/`** — Gallery binary: sidebar listing stories by category, main panel rendering selected story, title bar with light/dark toggle.
   - Reference: `zed_gpui_refs/gpui-component-main/crates/story/src/main.rs`
5. **2-3 placeholder stories** (WelcomeStory, PlaceholderStory) to validate rendering.

### Key patterns to establish
- **Registration via `inventory` crate** (Zed's pattern) — agents add a component file and it auto-registers. No central list to maintain. This replaces gpui-component's manual `Vec` approach.
- **`init()` cascade**: `theme::init(cx)` → `story::init(cx)` → app init
- **Builder pattern** for all components: `Button::new("id").variant(Primary).size(Small).on_click(handler)`

### Verification
- `cargo run -p gpui-studio` opens a window with sidebar and renders WelcomeStory
- Clicking sidebar items switches the main panel content
- Light/dark theme toggle works

---

## Phase 2: First 10 Components

**Goal**: Build real components in `crates/components/` with matching stories in `crates/studio/src/stories/`. Every component is immediately visible and interactive in the workbench.

### Component order (dependency-driven)
| # | Component | Why this order |
|---|-----------|---------------|
| 1 | **Label** | Simplest. Establishes the pattern (builder, Render, theme tokens). |
| 2 | **Icon** | Needed by Button and many others. |
| 3 | **Button** | Primary interactive component. Variants: primary/secondary/ghost/destructive/outline. States: hover/active/disabled/loading. Sizes: sm/md/lg. |
| 4 | **Divider** | Trivial. Needed for layout sectioning. |
| 5 | **Input** | First stateful component — introduces `Entity<InputState>` pattern. |
| 6 | **Checkbox** | Toggle input. Establishes controlled/uncontrolled pattern. |
| 7 | **Switch** | Toggle variant with different visual. |
| 8 | **Tooltip** | First overlay. Tests floating positioning. |
| 9 | **Badge** | Small status indicator. Exercises theme tokens. |
| 10 | **Spinner** | Loading indicator. Tests animation in GPUI. |

### Per-component structure
- Component source: `crates/components/src/<name>/` (or `<name>.rs` for simple ones)
- Story: `crates/studio/src/stories/<name>_story.rs`
- Each story shows: all variants, all sizes, all states, interactive controls where relevant

### Verification
- Each component renders correctly in the workbench across light/dark themes
- Interactive components respond to clicks/keyboard
- Stories show all meaningful variant × state combinations

---

## Phase 3: Primitive Extraction

**Goal**: After building 10+ styled components, extract shared behavioral patterns into `crates/primitives/`.

### Patterns to watch for (from studying Radix/Base UI)
| Pattern | Trigger to extract | Needed by |
|---------|--------------------|-----------|
| **Focus trap** | Dialog or Sheet component built | Dialog, Sheet, Modal |
| **Dismissable layer** | 2+ overlay components exist | Tooltip, Popover, Dialog, Menu |
| **Roving focus** | RadioGroup or TabList built | RadioGroup, TabList, Menu, ListBox |
| **Overlay positioning** | 2+ floating components exist | Tooltip, Popover, Select, HoverCard |
| **Collection/selection** | List or Select built | List, Select, ComboBox, Menu, Tree |

### Rules
- Do NOT create primitives speculatively — extract only after 2+ components share the pattern
- Primitives have **zero theme dependency** — pure behavior engines
- Extraction = move behavioral logic to `crates/primitives/`, leave styled wrappers in `crates/components/`

---

## Phase 4: Theming Layer

**Goal**: Evolve the theme system from "works with defaults" to "fully customizable token system."

### Token categories
- **Color**: background, foreground, primary, secondary, muted, accent, destructive, border, ring (semantic tokens)
- **Spacing**: standardized scale (0, 1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64)
- **Radius**: none, sm, md, lg, xl, full
- **Typography**: font families, sizes, weights, line heights
- **Shadows**: elevation levels (sm, md, lg, xl)

### Theme editor (studio feature)
- Right panel or bottom drawer in the workbench
- Tree view of tokens by category
- Color pickers, numeric steppers
- Live preview — changes reflect immediately in the active story
- Export to JSON/TOML

### Theme files
Follow gpui-component's proven JSON format with `{ "themes": [{ "name", "mode", "colors", ... }] }`.

---

## Phase 5: CLI Distribution

**Goal**: `gpui` CLI for scaffolding apps and installing components as source code (shadcn model).

### Commands
```
gpui init <app-name>                    # Scaffold new GPUI app
gpui add <component> [--yes] [--json]   # Copy component source into app
gpui update <component|all> [--json]    # Update with 3-way merge
gpui remove <component> [--json]        # Remove component
gpui theme export [--format json|toml]  # Export theme
gpui theme import <path>                # Import theme
gpui doctor [--json]                    # Verify project health
gpui list [--json]                      # List available components
```

### Agent-friendly design
- All commands support `--json` structured output
- `--dry-run` / `--plan` shows what would change
- `--yes` for unattended operation
- Idempotent — re-running is safe
- Manifest (`gpui.lock.json`) tracks installed components

---

## What to do first

Phase 1 is the immediate next step. Scaffold the Cargo workspace and get a window on screen with the gallery layout. Everything after that is incremental — each new component is a self-contained addition that's immediately visible in the workbench.
