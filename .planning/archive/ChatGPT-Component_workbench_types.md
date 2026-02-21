# Component workbench types

#### You:
I want to build a ui component library viewer like “tweakcn” where I can put pages of all the components and test their interactions and see them and edit themes in one place but for GPUI/rust. What category of app or utility is this called?

#### ChatGPT:
This kind of tool is typically categorized as a **component workbench** (or **UI component workbench**).

Other common names you’ll see (depending on emphasis):

- **Design system workbench** / **design system studio** — if it’s explicitly for a full design system (tokens, typography, spacing, themes) plus component docs.
- **Component explorer** — if browsing and isolating components is the primary goal.
- **Component playground** / **UI playground** — if interactive testing/knobs/props editing is central.
- **Component gallery** / **pattern library** — if it’s mostly visual reference and documentation.
- **Storybook-style tool** — “Storybook” is the best-known example; people use it as a generic label even when it isn’t web/React.
- **Style guide generator** — if it generates docs pages from source + renders examples.
- **Visual test harness** — if it’s aimed at regression testing, interaction states, and snapshots.

For your description (“pages of all the components,” test interactions, and edit themes in one place), the most accurate umbrella term is:

**Design system / component workbench (Storybook-like component explorer)**.


#### You:
Well, my goal is to build out a similar library to like ShadCN using all Rust and GPUI to make those components, and I'd like to be able to see all the components, change the themes, export themes, see different states, test them. So it would be a component explorer, playground, gallery, style guide generator, and even visual test harness all in one. Maybe even connect it to some, like, dummy data, like a fake API or some sort of, like, random function generator to show graphs, you know, just like straight-up dummy data in one app so I can work there to build more components and test them and everything and make sure they look good. And then when I want to reuse them across my other actual applications, I can deploy them out from there. And I can also capture learnings and say, okay, like, I was trying to build this component, and it took me a while, and here's some of the pitfalls I ran into. Here's some of the problems. Here's how I solved it. And here's reusable patterns and snippets that I can make sure that I implement this flawlessly in the new app later.

#### ChatGPT:
Given that scope, the most standard label is:

**Design system workbench** (aka **component workbench / component studio**).

If you want a name that signals “everything in one place” (explore + theme + test + docs + dummy data), these are the terms people usually reach for:

- **Design System Studio** — implies tokens/themes + component development + documentation.
- **Component Workbench** — implies isolated rendering, state switching, interaction testing.
- **UI Kit Lab** / **UI Lab** — informal, but communicates experimentation + tooling.
- **Component Development Environment (CDE)** — a bit jargon-y, but accurate: a dedicated environment for building and validating components.
- **Visual UI Test Harness** — if you want to foreground regression/snapshots/state matrices.

If you’re describing it to others in one sentence, a clean phrasing is:

> “A Storybook-like design system workbench for GPUI/Rust, with theming, state matrices, dummy-data fixtures, and visual regression testing.”

### How your described features map to common terms

- Browse components + pages: **component explorer / gallery**
- Live prop/state controls + interactions: **playground / workbench**
- Theme tokens + edit/export: **design system studio**
- Snapshot/state grid + regression: **visual test harness**
- Notes/pitfalls/patterns/snippets: **living style guide / design system documentation**

### If you want a single “category name” for the whole app

The least awkward umbrella term (that doesn’t undersell it) is:

**Design system workbench**.

### Optional: naming it like a product (not a category)

If you’re going to brand it internally, names that match your description:

- “GPUI Studio”
- “GPUI Workbench”
- “Rust UI Lab”
- “Component Forge”
- “Kit Studio”

If you tell me whether your primary focus is **authoring components** vs **documentation/testing**, I can suggest the best “primary category” label (what you’d put in a README tagline) and the secondary descriptors.


#### You:
Okay, well, write a product spec for me for this app that I've described. I don't care what you call it. But, yeah, please, I like the product name of GPUy Studio. But yeah, please write a full product specification for this application that I just described.

#### ChatGPT:
# Product Specification: **GPUy Studio**

## 1) Overview

**GPUy Studio** is a dedicated, desktop-first design-system and component-development environment built with **Rust** and **GPUI**. It lets a developer build, browse, and validate a reusable component library (ShadCN-style), while managing themes/tokens, exploring component states, running interaction scenarios, rendering components with realistic dummy data, and capturing durable “learnings” (pitfalls, patterns, snippets) for reuse across production apps.

GPUy Studio’s primary value is **speed and confidence**: iterate on UI components and themes in one place, validate behavior and visuals across states, then export and reuse the library and themes in other GPUI/Rust applications.

---

## 2) Goals and Non-Goals

### Goals

1. **Single workspace** to develop and validate a GPUI component library.
2. **Component Explorer**: organized catalog of all components with stories/examples.
3. **State & Interaction Testing**: simulate and verify UI states and interactions.
4. **Theming**: edit tokens, preview changes live, and **export/import** themes.
5. **Dummy Data Fixtures**: generate realistic data to populate tables, charts, forms, etc.
6. **Reusable Output**: consume the component library and themes in other applications with minimal friction.
7. **Knowledge Capture**: document learnings, pitfalls, patterns, and snippets per component.

### Non-Goals (initially)

- Web-based viewer (target is desktop).
- Full Figma plugin parity.
- Fully automated end-to-end testing for production apps (focus is the library + studio).
- Multi-user real-time collaboration.

---

## 3) Target Users and Use Cases

### Primary user

- Rust developer building a reusable UI kit in GPUI.

### Secondary users

- Teams standardizing UI across multiple Rust/GPUI apps.

### Key use cases

1. Build a new component (e.g., Button, DataTable), iterate visually, validate variants/states.
2. Switch theme tokens and confirm readability/contrast and layout consistency.
3. Run a state matrix (hover/active/disabled/loading/error) across components.
4. Populate component demos with dummy API data to mimic real usage.
5. Capture learnings and reusable patterns while developing a component.
6. Export a theme package and component crate for use in another app.

---

## 4) Product Requirements

### 4.1 Component Library Model

**Concepts**

- **Component**: a reusable UI element (Button, Modal, Chart, etc.).
- **Example/Story**: an isolated demo of a component configuration.
- **Variant**: named configuration (size, intent, density).
- **State**: interactive/behavioral states (hover, pressed, focused, disabled, loading, error).
- **Fixture**: data provider used by an example (mock API, random generator, static JSON).

**Requirements**

- Components are discoverable via:
  - Tree navigation (categories)
  - Search by name/tags
- Each component page includes:
  - Overview + description
  - Examples (stories) with controls
  - State matrix view
  - Accessibility/contrast checks (at least token-level + basic heuristics)
  - Notes / learnings
  - Snippets

---

### 4.2 Explorer / Navigation

**Left Sidebar**

- Component categories (Atoms/Molecules/Organisms or custom)
- Favorites / Recent
- Search bar
- Theme selector

**Main Area**

- Component view
- Documentation and learnings panels
- State matrix and visual test views

**Requirements**

- Hotkeys for navigation, search, theme switching, toggling panels.
- Deep link support inside the app (e.g., “open Button → Primary variant”).

---

### 4.3 Component Playground

**Capabilities**

- Render component stories in isolation.
- Allow interactive manipulation of inputs:
  - Props/parameters controls (“knobs”)
  - Theme tokens live edit
  - Container constraints (width/height, padding, background)
  - Platform-like environment toggles (font scale, DPI scaling, reduced motion)

**Requirements**

- Controls panel auto-generated from story metadata (with overrides).
- Persist per-story settings (last used props, container size).
- “Copy as code” for the current configuration (Rust snippet).

---

### 4.4 Theme System (Tokens)

**Core concepts**

- **Theme**: a named set of tokens (colors, typography, spacing, radii, shadows).
- **Token**: value that components consume; supports referencing other tokens.

**Requirements**

- Theme Editor:
  - Tree view of tokens
  - Live preview across current component/story
  - Import/export themes (JSON/TOML recommended; optional Rust module export)
  - Theme diffs (compare theme A vs theme B)
- Multiple themes:
  - Light/Dark pairs
  - Arbitrary theme slots (e.g., “Brand A”, “High Contrast”)
- Token constraints:
  - Type-checked tokens (color vs number vs font)
  - Optional validation rules (e.g., min contrast threshold)

**Export formats**

- `theme.json` (canonical)
- `theme.toml` (ergonomic for Rust projects)
- Optional: `theme.rs` (generated constants or builder)

---

### 4.5 State Matrix View

A dedicated view that renders a grid of a component across combinations of variants/states.

**Requirements**

- Configurable axes:
  - X-axis: variants (size, intent)
  - Y-axis: states (default, hover, active, focus, disabled, loading, error)
- Snapshot capture per cell.
- Keyboard navigation across cells.
- “Diff mode”:
  - Compare against baseline snapshots.
  - Highlight changed cells.

---

### 4.6 Interaction Scenarios

For components with complex behavior (Modal flows, drag/drop, multi-step forms).

**Requirements**

- Scenario runner:
  - Step-based scripts (declarative) to trigger interactions.
  - Assertions for key outputs (e.g., text visible, state changed).
- Minimal scripting format:
  - Rust-based scenario definitions (preferred)
  - Optional: lightweight DSL or TOML config for simple flows
- Determinism:
  - Seeded random for reproducibility.
  - Record/replay input sequences.

---

### 4.7 Dummy Data & Fixtures

**Goals**

- Make it easy to build “real-feeling” demos: tables, lists, charts, forms.

**Fixture types**

1. **Static**: embedded JSON/TOML
2. **Generated**: seeded random generator (Faker-like)
3. **Mock API**: local fake endpoints or in-process “service”

**Requirements**

- Fixture library browser
- Per-story fixture binding
- Global seed control (per workspace + per story override)
- Built-in generators:
  - People, addresses, transactions
  - Time series for charts
  - Logs/events stream
- Ability to define custom generators in Rust and register them.

---

### 4.8 Documentation / Learnings / Snippets

**Per component**

- Markdown notes
- “Pitfalls” section
- “Patterns” section
- “Decisions” section (why a design was chosen)
- Snippet library:
  - Rust/GPUI patterns
  - Copy button
  - Tags and search

**Workspace-wide**

- Global “Knowledge Base” view aggregating learnings across components.
- Link notes to components and tokens.

---

### 4.9 Packaging and Reuse

**Outputs**

- A Rust crate (or workspace) containing:
  - Components
  - Theme system + token types
  - Optional generated theme modules
- Theme packages:
  - Versioned theme files
  - Migration notes/diffs

**Requirements**

- One-click “Export”:
  - Build and validate the library
  - Export theme files
  - Generate changelog summary (optional)
- Consumption pattern:
  - Other apps add the crate as a dependency
  - Load theme package at runtime (or compile-time)

---

## 5) UX / UI Specification

### Layout

- **Top Bar**: workspace, build status, theme selector, search, export, snapshot controls
- **Left Sidebar**: component tree + favorites + fixtures
- **Center**: canvas (component rendering area)
- **Right Panel**: knobs/props + theme tokens (toggle tabs)
- **Bottom Drawer**: logs, scenario runner, snapshot diffs

### Key screens

1. **Home / Dashboard**
   - Recent components
   - Baseline status (snapshots passing/failing)
   - Theme summary
2. **Component Page**
   - Examples, knobs, documentation
3. **State Matrix**
4. **Theme Editor**
5. **Fixtures Library**
6. **Knowledge Base**
7. **Visual Diff Review**

---

## 6) Technical Specification (High Level)

### Architecture

- **Host app**: GPUy Studio (GPUI desktop)
- **Library workspace**: Rust workspace containing component crate(s)
- **Registry**: compile-time or runtime registration of:
  - components
  - stories
  - fixtures
  - scenarios

### Component registration model

- Story definitions in Rust:
  - Name, tags, description
  - Controls schema (typed)
  - Render function producing GPUI view
  - Optional: fixture binding
- Use macros to reduce boilerplate:
  - `#[story]`, `#[component]`, `#[fixture]`

### Rendering isolation

- Each story renders in a sandbox container with:
  - theme context
  - environment settings
  - fixture provider

### Snapshots and diffs

- Render offscreen to an image buffer.
- Store baseline snapshots per story/state cell.
- Diff algorithm:
  - pixel diff with thresholds
  - optional perceptual metrics later

### Data determinism

- Seeded RNG for generated fixtures and scenarios.
- Saved seeds in workspace config.

---

## 7) Data Model

### Workspace configuration

- `workspace.toml`
  - theme list and paths
  - default theme
  - snapshot settings
  - fixture seeds
  - component categorization

### Theme format (canonical JSON)

- Metadata: name, version, author
- Tokens: typed map with optional references
- Optional: semantic tokens (e.g., `color.text.primary` referencing base palette)

### Story metadata

- `id`, name, tags
- control schema
- environment defaults
- fixture bindings
- scenario list
- snapshot matrix settings

---

## 8) MVP Scope

### MVP Features

- Component explorer with search
- Story rendering + basic knobs
- Theme switcher + basic token editor
- Import/export theme JSON/TOML
- Basic fixture generator (seeded) + static JSON fixtures
- State matrix rendering (manual list of states/variants)
- Snapshot capture + baseline compare
- Per-component notes (markdown) + snippet copy

### Out of MVP

- Advanced scenario runner assertions
- Perceptual diffs
- Multi-workspace management
- Plugin ecosystem
- Collaboration features

---

## 9) Quality Requirements

### Performance

- Smooth interaction at 60fps for typical components.
- Fast theme switching (<200ms perceived for typical stories).
- Snapshot generation scalable (batch mode with progress).

### Reliability

- Deterministic renders under fixed seed + theme.
- Crash-safe workspace files (atomic writes).

### Developer experience

- Minimal ceremony to add a new component story.
- Strong typing for tokens and controls.
- Clear error messages when registration fails.

---

## 10) Risks and Mitigations

1. **GPUI limitations** (rendering offscreen, snapshot tooling)
   - Mitigation: start with minimal snapshot pipeline; expand with GPUI capabilities.
2. **Boilerplate in story definitions**
   - Mitigation: macros + derive-based schemas for knobs.
3. **Theme token complexity**
   - Mitigation: start with small typed token set; add references/diffs later.
4. **Dummy data realism vs complexity**
   - Mitigation: a few high-impact generators first; allow custom Rust generators.

---

## 11) Milestones

1. **M1: Explorer + Stories**
   - registry, navigation, render canvas
2. **M2: Theme System**
   - tokens, editor, import/export
3. **M3: State Matrix**
   - grid rendering, presets
4. **M4: Fixtures**
   - static + generated fixtures, seeded
5. **M5: Snapshots**
   - baseline capture, diff viewer
6. **M6: Learnings**
   - notes/snippets, knowledge base view
7. **M7: Export/Reuse**
   - packaging workflow, consumption docs

---

## 12) Acceptance Criteria (MVP)

- Can browse and open any registered component and its stories.
- Can modify story props live and copy generated Rust config snippet.
- Can switch themes and edit tokens with immediate preview.
- Can export a theme and load it again with identical rendering.
- Can render a state matrix for at least one component and capture snapshots.
- Can diff current snapshots against baseline and identify changes.
- Can attach and search notes/snippets per component.
- Can bind a story to deterministic dummy data and reproduce it via seed.

---

## 13) Naming and Positioning

**GPUy Studio**: “A GPUI-first design system workbench for Rust: build components, themes, fixtures, and visual regression checks in one desktop app.”

If you want, I can also produce:

- a workspace folder structure (Rust crates + theme packages + snapshot folders),
- a story/fixture macro design sketch,
- and a concrete MVP backlog (epics → tickets) in the format you use (GitHub issues, Linear, etc.).


#### You:
Okay, a couple more things. Rename the app to GPUI, not GPUY. Second is, the development and consumption of this app, both writing it and consuming the components after it's completed, is going to be led by AI agents, so make sure this is agentic development and agentic consumption-friendly. And I want this application to be able to spin up a app boilerplate for GPUI and Rust that follows like vertical slice architecture. I have this documented somewhere else that I will provide. And then I want this to be like a package that I can install from like a crate or a homebrew package that I can install, and then I can use terminal commands like I would with ShadCN or basic UI, those unstyled components that ShadCN is based off of, and I can run a command to install the component in my app, and it would, you know, add it to the application and add the code or whatever, and it would follow kind of base UI and ShadCN conventions, like whatever the best practices are there. This way, it's really easy for me to reuse these components over and over across different applications that I build, because this is my favorite tech stack.

#### ChatGPT:
\[Unsupported Content]


#### ChatGPT:
\[Unsupported Content]


#### ChatGPT:
# Product Specification: **GPUI Studio**

## 1) Product Definition

**GPUI Studio** is a desktop-first, Rust + GPUI application for developing and validating a reusable component library (ShadCN-style), plus a companion CLI that can **scaffold new GPUI apps (vertical slice architecture)** and **install/update components into apps** via terminal commands.

A core design constraint: both the *building* of the library and the *consumption* of components/themes in downstream apps are expected to be driven primarily by **AI agents**. GPUI Studio therefore provides **agent-friendly interfaces**: deterministic outputs, machine-readable metadata, idempotent operations, and structured command output.

---

## 2) Goals

1. **Component development environment**: browse all components, render stories, test interactions, view state matrices, and run visual checks.
2. **Theme system**: edit tokens live, preview across components, import/export themes.
3. **Dummy data fixtures**: generated/static/mock-API data for realistic demos (tables, charts, dashboards).
4. **Visual test harness**: snapshot baselines + diffs, runnable locally and in CI.
5. **Knowledge capture**: per-component learnings, pitfalls, patterns, snippets.
6. **ShadCN-like consumption workflow**: install components into apps via CLI (copy source + conventions), track provenance, support updates.
7. **App scaffolding**: create new GPUI + Rust apps using a **vertical slice architecture** template.

---

## 3) Non-Goals (initial scope)

- Web-based Storybook clone.
- Collaborative multi-user editing.
- Full end-to-end app testing outside the component/studio ecosystem.
- Tight coupling to any one external design tool.

---

## 4) Users

### Primary

- **Agent-driven developer**: uses AI agents to generate/modify components, themes, and demos; expects stable automation surfaces.

### Secondary

- **Human developer**: occasionally hand-edits components, reviews diffs, and curates patterns/notes.

### CI consumer

- Runs snapshots/diffs headlessly, validates library changes.

---

## 5) System Components

1. **GPUI Studio (GUI app)**
   - Component explorer + playground + theme editor + state matrix + diff viewer + knowledge base.

2. **`gpui` CLI (automation surface)**
   - App scaffolding (vertical slice)
   - Install/update/remove components into apps
   - Theme import/export
   - Snapshot capture/diff
   - Agent-oriented modes (`--json`, non-interactive, deterministic)

3. **Component Library Source Registry**
   - A repository (local folder or git repo) containing:
     - component source templates
     - story definitions
     - fixtures
     - theme tokens + export formats
     - docs/learnings/snippets

---

## 6) Agentic Development & Consumption Requirements

### 6.1 Determinism and Reproducibility

- All generators (fixtures, scenario scripts) must be seedable.
- Snapshot generation must be stable given identical inputs (theme, seed, DPI, fonts).
- CLI must support `--seed`, `--lock-environment` (where feasible), and environment reporting.

### 6.2 Machine-Readable Interfaces

- Every CLI command supports:
  - `--json` structured output (no human prose mixed in)
  - stable exit codes
  - `--quiet` to suppress non-essential logs
- A **project manifest** records installed components, versions, origins, and local modifications.

### 6.3 Idempotent Code Installation

- `gpui add <component>` is idempotent:
  - If already installed and unchanged → no-op
  - If modified → explicit strategy (`--overwrite`, `--three-way-merge`, `--fail-on-diff`)
- `gpui update` supports safe patching with conflict markers and clear diff outputs.

### 6.4 “Agent Hooks”

- Provide a “planning mode” for agents:
  - `gpui add button --plan --json` outputs a patch plan (files to create/modify, diffs, reasons)
- Provide an “apply mode”:
  - `gpui add button --apply` executes the plan and returns a post-state summary.

### 6.5 Minimal Human Interaction

- Commands must run fully non-interactive via flags.
- Any prompts must have a `--yes` override and default behavior in `--non-interactive`.

---

## 7) Core Features (GUI)

### 7.1 Component Explorer

- Tree/category navigation + search + tags.
- Per-component page:
  - overview + props/variants
  - stories/examples
  - states matrix
  - fixtures binding
  - notes/learnings/snippets

### 7.2 Playground (Stories + Controls)

- Render component stories in isolation.
- Controls panel:
  - generated from typed metadata
  - supports component props + environment toggles (font scale, reduced motion, container constraints)
- “Copy as Rust snippet” for current story configuration.

### 7.3 Theme System

- Token-based themes (colors, typography, spacing, radii, elevations).
- Live edit tokens with immediate preview.
- Import/export:
  - canonical `theme.json`
  - optional `theme.toml`
  - optional generated Rust module (`theme.rs`) for compile-time use
- Theme diff view (token-level).

### 7.4 Dummy Data / Fixtures

- Fixture types:
  - static (JSON/TOML)
  - generated (seeded)
  - mock API (in-process; optionally HTTP for realism)
- Fixture library browser + per-story binding.
- Built-in generators for:
  - people/contacts
  - commerce transactions
  - logs/events
  - time series (charts)

### 7.5 State Matrix + Visual Regression

- Render grid across variants × states.
- Capture baseline snapshots.
- Diff viewer:
  - per-cell diffs
  - thresholds
  - batch run results summary
- Export report artifact for CI.

### 7.6 Knowledge Base

- Per-component Markdown notes:
  - pitfalls
  - patterns
  - decisions
- Snippet library with tags and copy.
- Workspace-wide aggregation (“show all pitfalls tagged ‘focus management’”).

---

## 8) CLI Feature Set (`gpui`)

### 8.1 Installation & Distribution

- Distribute as:
  - **crate** providing `gpui` binary (`cargo install gpui-studio` or similar naming if needed for crate namespace)
  - entity\["organization","Homebrew","macos package manager"] formula for macOS (`brew install gpui-studio`)
- The GUI app can be installed alongside the CLI, or CLI-only for CI usage.

### 8.2 Command Overview

#### App scaffolding (vertical slice architecture)

- `gpui init <app-name> [--template vertical-slice] [--non-interactive] [--json]`
- Outputs a new GPUI Rust app with:
  - baseline theme/token wiring
  - example feature slice(s)
  - story harness integration points
  - test/snapshot plumbing hooks

#### Component installation (ShadCN-style “copy into app”)

- `gpui add <component> [--variant <name>] [--yes] [--dry-run] [--json]`
- Installs component source into the target app:
  - writes Rust source files into conventional locations
  - updates module exports
  - optionally adds stories/examples in an “examples” or “stories” folder
  - updates tokens if required

#### Update / remove

- `gpui update <component|all> [--three-way-merge] [--json]`
- `gpui remove <component> [--json]`

#### Themes

- `gpui theme export [--format json|toml|rs] [--out <path>]`
- `gpui theme import <path>`
- `gpui theme diff <themeA> <themeB> [--json]`

#### Snapshots

- `gpui snapshot capture [--all|--component <name>] [--seed <n>]`
- `gpui snapshot diff [--baseline <ref>] [--json]`

#### Agent utilities

- `gpui doctor [--json]` (environment + project sanity checks)
- `gpui plan <operation...> --json` (explicit plan output)
- `gpui apply <plan-file>` (apply a previously generated plan)

---

## 9) Conventions: Component Installation Model

This mirrors entity\["organization","ShadCN","ui component collection"] conventions (source-copy approach rather than opaque binaries):

### 9.1 Source-of-Truth Approach

- “Installed” components live in the consuming app’s repo (editable).
- GPUI Studio maintains canonical templates and best-practice patterns.

### 9.2 Provenance Tracking

- The consuming app stores a manifest, e.g. `gpui.components.json`:
  - component name
  - template version/hash
  - installed file list
  - local modification status (hash per file)
  - required tokens/fixtures/stories
- Updates use this to determine safe patching strategies.

### 9.3 Patch/merge Strategy

- Default: 3-way merge using recorded base template.
- If conflict:
  - emit conflict markers
  - return machine-readable conflict report (`--json`)

---

## 10) Vertical Slice Architecture Scaffolding

### 10.1 Output Structure (example)

- `src/app/` (app shell)
- `src/shared/` (design system primitives, tokens, utilities)
- `src/features/<feature_name>/`
  - `ui/` (views/components specific to the feature)
  - `domain/` (types, rules, state machines)
  - `data/` (repositories, fake API adapters, fixtures bindings)
  - `tests/` (unit + scenario tests)
  - `stories/` (optional local stories/examples)

### 10.2 Template Inputs

- The scaffolder supports a template descriptor (file-based) so you can swap in your documented slice rules later without changing the CLI contract:
  - `gpui.template.toml` (paths, naming, modules, feature boundaries)
- GPUI Studio ships with a default “vertical-slice” template and a schema-validating loader.

### 10.3 Slice Compliance Tooling

- `gpui doctor` validates:
  - feature boundaries
  - forbidden imports (e.g., cross-feature UI dependencies)
  - missing exports or registration
- Output includes actionable remediation steps in JSON.

---

## 11) Technical Architecture (High-Level)

### 11.1 Registration & Metadata

- Components, stories, fixtures, and scenarios register via Rust macros/traits.
- Metadata is serializable:
  - for GUI rendering controls
  - for CLI installation/patch planning
  - for agent tooling

### 11.2 Headless Rendering

- Snapshot tooling supports headless rendering (CI):
  - deterministic environment config
  - fixed font assets where possible
  - fixed DPI settings

### 11.3 Theme Engine

- Typed tokens (color, size, font, elevation).
- Token references supported (semantic → base palette).
- Export pipeline produces stable output ordering for diff friendliness.

---

## 12) MVP Scope

### MVP (must)

- GUI: explorer, story rendering, basic controls, theme switching/editing, fixture binding (static + seeded generated), state matrix, snapshot capture/diff viewer, notes/snippets.
- CLI: `init`, `add`, `update`, `remove`, `theme import/export`, `snapshot capture/diff`, `doctor`, `--json`, `--dry-run`.

### Post-MVP (likely)

- Rich interaction scenario runner with assertions.
- Plugin system for external generators/linters.
- Perceptual diffs and more robust rendering sandboxing.

---

## 13) Acceptance Criteria (MVP)

1. `gpui init` produces a runnable GPUI Rust app that follows the vertical slice template.
2. `gpui add button` installs component source into a fresh app and updates exports successfully.
3. Re-running `gpui add button` is a no-op (idempotent) unless flags request changes.
4. Modifying an installed component and then running `gpui update button` yields either a clean merge or a structured conflict report.
5. Theme edits in the GUI reflect immediately in story rendering.
6. Theme export/import round-trips without semantic changes.
7. Snapshot capture/diff works both in GUI and headless CLI mode with deterministic seed.
8. All CLI operations have `--json` output suitable for AI agent consumption.

---


#### You:
I know shadcn uses radix or base UI. help me understand what the difference is between shadcn and the primitive components libraries

#### ChatGPT:
The simplest way to think about it:

- **Primitive libraries (Radix, Base UI, etc.) = behavior + accessibility + structure**
- **shadcn = styled, opinionated, copy-pasteable implementations built on top of primitives**

Below is the precise breakdown.

---

# 1) What “primitive component libraries” are

Examples:

- entity\["organization","Radix UI"]
- entity\["organization","Base UI"]

These libraries provide **low-level UI building blocks**.

They focus on:

✅ accessibility (keyboard nav, ARIA, focus traps)\
✅ interaction logic (menus, popovers, dialogs, comboboxes)\
✅ correct event/state handling\
✅ unstyled or minimally styled output

They deliberately **do NOT provide a finished design system**.

## Example: Dialog primitive

A primitive dialog gives you:

- open/close logic
- ESC handling
- focus trapping
- screen reader semantics
- portal rendering
- scroll locking

But visually:

- no colors
- no layout
- no spacing
- no animation decisions
- no typography

You style everything yourself.

---

# 2) What shadcn actually is

entity\["organization","shadcn/ui"] is NOT a traditional component library.

It is:

> A collection of pre-built, styled components that wrap primitives and are copied into your project as source code.

Key properties:

✅ uses Radix primitives internally\
✅ fully styled (Tailwind-based)\
✅ includes layout, spacing, tokens, variants\
✅ ships as copy-paste source files\
✅ intended to be modified

So instead of:

```
npm install component-library
```

You do:

```
npx shadcn add dialog
```

…and it **writes dialog source code into your repo**.

You own the component.

---

# 3) The core conceptual stack

Here’s the real architecture:

```
Application UI
↑
shadcn components
↑
primitive library (Radix/Base UI)
↑
DOM + browser APIs
```

---

# 4) Why primitives exist at all

Because accessibility and interaction logic are extremely hard.

Examples:

### Dropdown menu complexity

A real dropdown must:

- support keyboard arrows
- support typeahead search
- close on outside click
- close on escape
- handle nested submenus
- manage focus correctly
- work with screen readers
- prevent scroll issues
- manage z-index layering

This is hundreds of edge cases.

Primitive libraries solve this once.

---

# 5) What shadcn adds on top

shadcn turns primitives into:

- complete design tokens
- consistent spacing system
- consistent animations
- consistent variants
- predictable file structure
- reusable composition patterns

Example:

Primitive:

```
<Dialog.Root>
<Dialog.Trigger>
<Dialog.Content>
```

shadcn:

```
<Dialog>
<DialogHeader>
<DialogTitle>
<DialogFooter>
```

The second is:

- opinionated layout
- ready-to-use structure
- production-ready styling

---

# 6) Radix vs Base UI (quick difference)

### Radix

- extremely mature
- very accessibility-focused
- very granular primitives
- React-first ecosystem
- popular for production apps

### Base UI

- newer approach
- similar philosophy (unstyled, accessible primitives)
- often slightly simpler abstraction model
- aims for flexibility and modern architecture

Conceptually, though:

**They fill the exact same role.**

Both are:

> behavior engines for UI components.

---

# 7) The real mental model you should use

Think in layers:

## Layer 1 — Behavior engine

Radix / Base UI\
Handles:

- accessibility
- focus
- keyboard logic
- event ordering
- portal mechanics

---

## Layer 2 — Design system implementation

shadcn-style library

Handles:

- layout
- tokens
- variants
- spacing
- visual structure

---

## Layer 3 — App-specific components

Your business UI:

- dashboards
- domain widgets
- workflows

---

# 8) Why this matters for your GPUI project

If you’re building a **Rust GPUI equivalent of shadcn**, then:

You probably want the same separation:

### Primitive layer (you may need to build this first)

Reusable low-level constructs like:

- FocusTrap
- Portal
- Overlay manager
- Keyboard navigation helpers
- Popover positioning
- Listbox state machine

These are the “Radix equivalent”.

---

### Component layer

Then your shadcn-like components:

- Button
- Dialog
- Dropdown
- Select
- Tabs
- Table
- Toast

These wrap primitives + styling.

---

### Studio layer (your GPUI Studio)

Then your studio is where:

- primitives get tested
- components get built
- themes get validated
- fixtures simulate usage

---

# 9) The shortest possible explanation

If reduced to one sentence:

> Primitive libraries give you the brain of a component; shadcn gives you the finished body built on top of that brain.

---

If you want, I can also explain:

- the **exact architecture shadcn uses internally**
- how Radix primitives are structured under the hood
- what primitives you’d absolutely need first for GPUI
- the correct order to build primitives vs components vs studio

Tell me if you want that deeper architectural breakdown next.


#### You:
and do radix or base provide the source code for their primitives? for my GPUI component library and workbench, I want to be able to reference as much of the raw js or source code so I canreuse the same patterns and best practices. which path do you recommend?

#### ChatGPT:
\[Unsupported Content]


#### ChatGPT:
\[Unsupported Content]


#### ChatGPT:
Yes—**both Radix Primitives and Base UI publish their primitive source code** (they’re open-source on GitHub, MIT-licensed).

- Radix Primitives repo (MIT): citeturn0search0turn0search6turn0search3
- Base UI repo (MIT): citeturn0search1turn0search2

So if your goal is “read the raw source to reuse patterns and best practices,” you can.

---

## How shadcn differs from primitives (in terms of *source you’ll study*)

### Radix/Base UI source teaches you:

- focus management patterns (roving tabindex, focus trap)
- keyboard interaction maps (arrow keys, typeahead, escape, tab loops)
- accessible semantics and ARIA wiring
- layering/portal/overlay patterns
- event ordering edge cases
- state machines and controlled/uncontrolled component patterns

### shadcn source teaches you:

- how to **compose** primitives into “finished” components
- how to structure variants + class composition
- how to present a consistent component API
- how to integrate tokens/themes and app-level conventions
- how to package “copy into your repo” components + a CLI workflow

For your GPUI ecosystem, you want *both sets of lessons*, but they apply to **different layers**.

---

## Recommended path for GPUI Studio + your component library

### 1) Treat primitives as a “behavior kernel” you build first

In web land, Radix/Base UI are the kernel. In GPUI land, you’ll likely need your own equivalents (even if smaller at first):

**Must-have primitive capabilities**

- Focus management primitives:
  - focus trap
  - roving focus for menus/listboxes
  - focus restoration on close
- Overlay primitives:
  - modal layering + stacking manager
  - dismissable layer (outside click / escape)
  - portal-equivalent pattern (render above)
- Positioning primitives:
  - popover/tooltip anchor positioning (Floating UI is the web standard inspiration)
- Input/gesture primitives:
  - consistent pointer/keyboard handling abstractions

This is where reading Radix/Base UI code pays off most.

### 2) Build your shadcn-like layer as “source-installed components”

This matches your stated consumption model:

- users (agents) run `gpui add dialog`
- it copies component source into the target app
- it updates exports, theme tokens, and any required primitives
- it records provenance in a manifest for updates/merges

That’s the shadcn playbook, but targeting Rust/GPUI.

### 3) Make GPUI Studio the integration + validation environment

Studio should stress-test:

- primitives in isolation
- composed components with variants/states
- themes + token changes
- fixtures/dummy data scenarios
- snapshots/diffs (CI)

---

## Radix vs Base UI: which source is better to study for your purpose?

Both are valuable, but they bias differently:

### If you want “battle-tested patterns to port”

**Radix** is a strong reference because it’s been widely used in production for years and has a lot of solved edge cases in the open. citeturn0search0turn0search6

### If you want “newer architecture + composability emphasis”

**Base UI** is explicitly positioned as unstyled, composable building blocks from the same broader ecosystem (Radix/Floating UI/MUI folks). citeturn0search2turn0search1

**Practical recommendation:**\
Use **Radix as your primary “edge-case encyclopedia”** and **Base UI as a secondary “architecture/style reference.”** You’ll end up borrowing patterns from both.

---

## What to copy vs what not to copy (for GPUI)

### Good to port conceptually

- roving focus algorithms
- dismissable layer logic
- controlled/uncontrolled state patterns
- state machine decomposition (logic separate from view)
- deterministic event handling (pointer vs keyboard)
- “slot/asChild” style composition ideas (adapted to Rust)

### Not directly portable

- DOM/ARIA specifics (you’ll need GPUI equivalents)
- portal specifics (GPUI’s scene graph differs)
- CSS/layout assumptions (you’ll express tokens differently)

---

## Agent-friendly approach (fits your “AI-led dev + consumption” constraint)

If agents are driving edits and installs, optimize for:

- **small, typed primitives** with tight contracts
- **declarative metadata** for stories/variants/states (so agents can generate them)
- **idempotent CLI installs** with a manifest + 3-way merge strategy
- **golden tests**: snapshot grids for variants/states + deterministic fixtures

That combination tends to keep agent loops stable.

---

## Concrete “best” approach for your stack

1. **Start by implementing a minimal primitives set** (focus/overlay/positioning). Use Radix/Base UI source as a reference library.
2. **Ship components as source** (shadcn model) installed via `gpui add`.
3. **Bake Studio + CLI into one product**:
   - Studio for human+agent iteration
   - CLI for scaffolding vertical slice apps + installing/updating components
4. **Make themes and fixtures first-class** so you can validate components under real-ish conditions.

If you paste your vertical-slice documentation when you’re ready, I can incorporate it into the scaffolder section of the spec (commands, folder layout, constraints, and the exact file mutations `gpui add` should perform).
