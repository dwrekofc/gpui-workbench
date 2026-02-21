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

### CLI "Agent Hook" Protocol
Every mutation command (add, update, remove) must support:
1.  `--json`: Output machine-readable result.
2.  `--plan`: Dry-run that returns a JSON list of file mutations.
3.  `--apply <PLAN_FILE>`: Executes a previously generated plan.
4.  `--seed <N>`: Ensures deterministic generation/scaffolding.

### Theme Token Schema
Themes are defined as typed JSON objects:
*   **Base**: Raw values (e.g., `blue-500`).
*   **Semantic**: Use-case values (e.g., `button.background` -> `primary`).
*   **State-Aware**: (e.g., `button.hover.background`).

### Provenance & Licensing
To maintain "Clean Room" integrity:
*   Every file adapted from `zed` or `gpui-component` must include a top-level comment referencing the original source, commit hash, and license.
*   `PROVENANCE.md` tracks these mappings at the crate level.

---

## 5. Success Criteria (MVP)
*   **CLI**: `gpui add button --apply` creates a working Button in a fresh app.
*   **Workbench**: Changing `primary.background` in the Token Editor updates all buttons instantly.
*   **Agentic**: An LLM can read the `--plan` output and correctly predict the resulting file tree.
*   **Determinism**: Snapshots are identical across different developer machines given the same seed.
