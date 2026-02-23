# GPUI Studio: Master Development Plan

**Product Vision**: A high-performance, desktop-first design system workbench and CLI for the GPUI/Rust ecosystem. It enables developers (and AI agents) to build, browse, and validate a reusable component library (ShadCN-style) while providing a CLI for scaffolding apps and installing components as source.

---

## 1. Design Philosophy

1.  **Agent-First**: Every CLI command and Workbench feature must be machine-readable (JSON-first) and deterministic. Operations follow a `Plan -> Apply` model.
2.  **Source as Distribution**: Following the ShadCN model, components are installed into applications as raw source code. This ensures full customizability and zero-dependency bloat.
3.  **Clean Separation (Kernel vs. Skin)**:
    *   **Primitives**: Low-level, unstyled behavioral logic (Radix/Base UI style).
    *   **Components**: Opinionated, styled implementations built on top of primitives.
4.  **Workbench-Validated**: The Workbench is the "gold standard" environment. If a component is available in the CLI, it must be fully interactive and testable in the Studio.
5.  **Vertical Slice Architecture**: All scaffolding and component installation follows a strict vertical slice pattern to ensure scalability and agent readability.

---

## 2. System Architecture

### Monorepo Structure (`gpui-workbench/`)
```text
├── apps/
│   ├── studio/          # Workbench GUI (Binary)
│   └── cli/             # 'gpui' CLI tool (Binary)
├── crates/
│   ├── components/      # Styled component library (The "ShadCN" layer)
│   ├── primitives/      # Behavioral logic (The "Radix" layer)
│   ├── registry/        # Component metadata, schemas, and discovery
│   ├── theme/           # Token system (colors, spacing, typography)
│   ├── story/           # Workbench harness & auto-registration macros
│   └── assets/          # Icons, fonts, and static resources
├── templates/           # Scaffolding templates (Vertical Slice)
├── docs/
│   ├── ARCHITECTURE.md  # Deep dives into system decisions
│   ├── CONTRACTS.md     # JSON schemas for CLI & Agent hooks
│   └── PROVENANCE.md    # Licensing and attribution tracking
└── PLAN.md              # This document (Single Source of Truth)
```

---

## 3. Detailed Roadmap

### Phase 1: Workbench Shell & "Inline Governance"
**Goal**: Get pixels on screen and establish the development loop.
*   **Theme Engine**: Implement `ThemeColor` and `ActiveTheme` traits. Load from JSON.
*   **Story Harness**: Build the `Story` trait and `inventory`-based auto-registration.
*   **Studio App**: Sidebar navigation, Main canvas, Light/Dark toggle.
*   **Governance**: Initialize `PROVENANCE.md` with the first component (Button) attribution.

### Phase 2: CLI Control Plane & Core Components
**Goal**: Enable the "agentic consumption" loop.
*   **CLI Core**: Implement `gpui add <component> --plan` and `--apply`.
*   **Registry**: Build a metadata extractor that reads Rust source and generates JSON manifests.
*   **Core-4**: Build `Label`, `Icon`, `Button`, and `Divider`.
*   **Vertical Slice**: Define the default folder structure for installed components.

### Phase 3: The "Core-12" Library & Token Editor
**Goal**: A complete, usable UI kit.
*   **Expansion**: Build `Input`, `Checkbox`, `Switch`, `Tooltip`, `Select`, `Tabs`, `Dialog`, `Toast`.
*   **Workbench MVP**: 
    *   **Token Editor**: Live-edit theme tokens with immediate UI updates.
    *   **State Matrix**: A grid view showing every component variant/state combination.
    *   **Fixture Binding**: Bind components to seeded, deterministic dummy data.

### Phase 4: Primitive Extraction (The "Radix" Shift)
**Goal**: Move from "styled-only" to "behavior-driven."
*   **Trigger**: Once 2+ components share complex logic (e.g., Tooltip and Dialog both need Overlay management).
*   **Extraction Targets**: 
    *   `FocusTrap`
    *   `DismissableLayer` (Outside click/Escape)
    *   `RovingFocus` (Keyboard navigation)
    *   `OverlayPositioning` (Anchored popovers)

### Phase 5: Distribution & Adoption
**Goal**: Make it a standard tool.
*   **Scaffolding**: `gpui init` to spin up fresh Vertical Slice apps.
*   **Packaging**: Homebrew formula and `cargo install` distribution.
*   **Visual Regression**: Snapshot capture and CI-driven diffing.
*   **Validation**: Successfully consume the library in at least one external production app.

---

## 4. Technical Specifications

### 4.1 Component API & "The Contract"
To ensure consistency and agent-readability, all components must implement a standard trait-based interface inspired by Zed's `ui` crate:

*   **Builder Pattern**: Every component uses a fluid builder API.
    ```rust
    Button::new("id")
        .variant(ButtonVariant::Primary)
        .size(Size::Sm)
        .icon(IconName::Plus)
        .on_click(|_, cx| { /* ... */ })
    ```
*   **Common Props**: All components support `id`, `tooltip`, and `metadata` (for agent tracking).
*   **State Management**: Stateful components (Inputs, Selects) use the `Entity<T>` pattern or the `Value` trait to handle controlled vs. uncontrolled states.

### 4.2 Accessibility (a11y) Requirements
GPUI components must be "accessible by default," mirroring Radix UI's behavioral engine:
*   **Keyboard Navigation**:
    *   `Tab` loops for Modals (Focus Trap).
    *   `Arrow Keys` for navigation within Groups (Roving Tabindex).
    *   `Enter/Space` for activation.
    *   `Esc` for dismissal (Overlays, Popovers).
*   **Semantic Mapping**: Use GPUI's `Accessible` trait to map components to standard ARIA roles (e.g., `Button`, `Dialog`, `Status`).
*   **Screen Reader Support**: All interactive elements must have a descriptive label (via `aria_label` or child text).

### 4.3 Vertical Slice Architecture Mapping
The `gpui` CLI scaffolds apps and adds components using a rigid "Feature-First" structure:
```text
src/
├── app/                  # App shell, router, and global state
├── shared/
│   ├── ui/               # Global components (Button, Input, etc.)
│   ├── theme/            # Design tokens and theme registry
│   └── utils/            # Shared helpers
└── features/
    └── <feature_name>/   # e.g., 'workspace' or 'chat'
        ├── ui/           # Feature-specific views
        ├── domain/       # Logic, types, and state machines
        ├── data/         # API adapters and fixtures
        └── tests/        # Feature-level integration tests
```
**CLI Behavior**: `gpui add button` will:
1.  Write source to `src/shared/ui/button/`.
2.  Update `src/shared/ui/mod.rs` to export the new module.
3.  Inject any required tokens into `src/shared/theme/tokens.json`.

### 4.4 Machine-Readable Schemas (Agent Hooks)
Agents interact with the system via stable JSON contracts.

#### **Component Registry (`registry.json`)**
```json
{
  "name": "button",
  "version": "1.0.0",
  "variants": ["primary", "secondary", "ghost"],
  "states": ["hover", "active", "focused", "disabled"],
  "props": {
    "label": "string",
    "size": "enum[sm, md, lg]"
  },
  "files": ["src/shared/ui/button/button.rs", "src/shared/ui/button/mod.rs"]
}
```

#### **CLI Execution Plan (`plan.json`)**
Generated by `gpui add <name> --plan`:
```json
{
  "operation": "add",
  "component": "button",
  "mutations": [
    { "action": "create", "path": "src/shared/ui/button/mod.rs" },
    { "action": "modify", "path": "src/shared/ui/mod.rs", "strategy": "append_export" }
  ],
  "conflicts": []
}
```

---

## 5. Non-Functional Requirements

### 5.1 Performance & Environment
*   **Zero-Lag Rendering**: Interaction feedback (hover/click) must be processed within one frame (16.6ms at 60Hz).
*   **Fast Theme Switching**: Swapping themes in the Workbench must complete in <100ms.
*   **Target Runtime**: macOS (Native), with an architectural path toward cross-platform (Metal/Vulkan).
*   **Build Integrity**: The `inventory` macro-based registration must not add more than 2 seconds to total compile time in debug mode.

### 5.2 Security & Safety
*   **Strict Provenance**: All code "cloned" or "adapted" from Zed/GPUI-Components must be clearly attributed in `PROVENANCE.md` to maintain MIT license compliance.
*   **Idempotency**: All CLI mutations must be safe to run multiple times without corrupting the file system.

---

## 6. Success Criteria (MVP)
*   **CLI**: `gpui add button --apply` creates a working Button in a fresh app.
*   **Workbench**: Changing `primary.background` in the Token Editor updates all buttons instantly.
*   **Agentic**: An LLM can read the `--plan` output and correctly predict the resulting file tree.
*   **Determinism**: Snapshots are identical across different developer machines given the same seed.
