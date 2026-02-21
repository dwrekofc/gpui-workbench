# GPUI Workbench — Unified Plan (Draft Outline)

> **Synthesis of**: Claude plan (workbench-first, detailed scaffold) + Codex plan (CLI-leads, governance-first)
> **Optimized for**: Fast demo, fast CLI, solo dev, AI-friendly, low rework risk

---

## Design Principles

1. **Dual-track, workbench-leads-then-CLI-catches-up** — Invert Codex's "CLI leads" to "CLI follows closely." Ship a visible demo in Phase 1, but CLI is Phase 2 (not Phase 5).
2. **Governance without paralysis** — Provenance + contracts defined as *living docs alongside code*, not as a blocking Phase 0. Standards ship with the first component, not before it.
3. **Claude's scaffold, Codex's rigor** — Use Claude's detailed crate tree and file layout, but add Codex's schema/registry crate and provenance tracking.
4. **Extract, don't speculate** — Clean-room primitives extracted after 2+ components share behavior (Claude's rule), built with pattern-clone + attribution (Codex's governance).
5. **Plan-first CLI** — Codex's `--plan` / `--apply` model is non-negotiable for AI friendliness.
6. **Deferred bootstrap template** — No `gpui init` template and no abstraction layer until Phase 5. Avoid building interfaces for things that don't exist yet.

---

## Repo Structure (Claude's scaffold + Codex additions)

```
gpui-workbench/
├── crates/
│   ├── studio/          # Workbench binary (Claude's gallery layout)
│   ├── components/      # Styled component library
│   ├── primitives/      # Behavioral primitives (extracted, not speculative)
│   ├── theme/           # Theme engine + token system
│   ├── story/           # Story framework + registry
│   ├── assets/          # Icons, fonts
│   ├── schema/          # ← From Codex: component metadata, registry model
│   └── cli/             # ← Promoted from skeleton to Phase 2 deliverable
├── provenance/          # ← From Codex: PROVENANCE.md, attribution tracking
├── docs/
│   ├── contracts.md     # Component contract template (ships with first component)
│   └── cli-json.md      # CLI output schema (ships with CLI)
└── examples/
```

**Key compromise**: Claude's simple crate names (`theme/`, `components/`) over Codex's prefixed names (`gpuiwb-*`). Codex's `schema/` and `provenance/` added as new concerns.

---

## Phase Plan (5 phases, compressed timeline)

### Phase 1: Workbench Shell + Inline Governance
> **From Claude**: Get pixels on screen immediately
> **From Codex**: But ship governance artifacts alongside, not before

**Build:**
- Theme crate (adapted from gpui-component patterns)
- Story framework with inventory-based auto-registration
- Gallery binary: sidebar, preview pane, light/dark toggle
- 2-3 placeholder stories (Welcome, Button sketch)

**Also ship (inline, not blocking):**
- `PROVENANCE.md` with first attribution entries
- Component contract template (defined by the first real component)
- `init()` cascade pattern established

**Exit**: `cargo run -p studio` opens a window, renders stories, toggles theme.

---

### Phase 2: CLI Core + First 4 Components (Dual-Track Begins)
> **From Codex**: CLI is priority track, plan-first behavior
> **From Claude**: Components are dependency-ordered and immediately visible

**CLI track:**
- `gpui add <component> --plan` → shows what would change
- `gpui add <component> --apply` → executes the plan
- `gpui list --json` → registry from Rust source metadata
- Schema crate: component metadata model, install mappings

**Component track (parallel):**
1. Label → 2. Icon → 3. Button → 4. Divider
- Each gets: story, all variants/states, contract entry, provenance note

**Exit**: `gpui add button --plan` produces valid output. 4 components render in workbench.

---

### Phase 3: Core Components + Workbench MVP Boundary
> **From Codex**: Viewer + token editor boundary (no scope creep)
> **From Claude**: Concrete component ordering continues

**Component track (continues):**
5. Input → 6. Checkbox → 7. Switch → 8. Tooltip → 9. Select → 10. Tabs → 11. Dialog → 12. Toast

**Workbench enhancements (scoped to Codex's MVP boundary):**
- Token editor (live preview, no full theme designer yet)
- State matrix view per component
- Theme import/export (JSON)

**CLI track (continues):**
- `gpui update`, `gpui remove`
- `gpui doctor` basic checks
- Provenance capture during install

**Exit**: 12 components render. Token editor works live. CLI installs all 12.

---

### Phase 4: Primitive Extraction
> **From Claude**: Extract only after 2+ components share the pattern
> **From Codex**: Clean-room with strict provenance

**Extract as patterns emerge (not before):**
| Primitive | Trigger |
|-----------|---------|
| Dismissable layer | Tooltip + Dialog + Popover exist |
| Focus trap | Dialog + Sheet exist |
| Overlay positioning | Tooltip + Select + Popover exist |
| Roving focus | Tabs + Radio + Menu exist |
| Collection/selection | Select + List + Menu exist |

**Rules:**
- Zero theme dependency in primitives
- Each extraction gets a provenance entry
- Pattern-clone from Zed/gpui-component with attribution, never blind fork

**Exit**: Shared behaviors live in `primitives/`, components are thinner wrappers.

---

### Phase 5: Distribution + Theming + Adoption
> **From Codex**: Hybrid consumption, downstream validation
> **From Claude**: Full theme editor, CLI polish

- `gpui init` — bootstrap template decided *now*, informed by 4 phases of real usage (no speculative abstraction built earlier)
- Full theme editor in workbench (color pickers, export)
- Hybrid consumption: source-copy default, crate option
- `gpui theme export/import`
- **Validation gate**: At least one external app consumes components

**Exit**: External app works. CLI is agent-complete. Theme system is customizable.

---

## Decisions & Compromises Summary

| Dimension | Decision | Source | Compromise |
|-----------|----------|--------|------------|
| First demo | Phase 1 (immediate) | Claude | None — this is strictly better |
| CLI timing | Phase 2 (early) | Codex | Moved up from Claude's Phase 5 |
| Governance | Inline with code, not blocking | Hybrid | Codex wanted Phase 0; we ship docs *with* first artifacts instead |
| Component count | 12 (Core-12) | Codex | Claude had 10; we add Select, Tabs, Dialog, Toast; drop Badge, Spinner, Popover to later |
| Scaffold detail | Claude's crate tree + file layout | Claude | Added Codex's schema/ and provenance/ |
| Primitive strategy | Extract after reuse signals | Claude | With Codex's clean-room + provenance governance |
| CLI mutation model | Plan-first (`--plan`/`--apply`) | Codex | None — this is strictly better for AI |
| Naming | Simple (`theme/`, `components/`) | Claude | Over Codex's `gpuiwb-*` prefix style |
| Bootstrap template | Fully deferred — no template, no abstraction layer | Hybrid | Codex wanted abstraction now + decision later; we defer *both* to Phase 5 |
| Workbench scope | Viewer + token editor (MVP) | Codex | Full theme editor deferred to Phase 5 (Claude wanted Phase 4) |
| Registration | `inventory` crate auto-registration | Claude | Codex didn't specify; this is Claude's proven pattern |
| Risk management | Explicit (from Codex) | Codex | Added to plan as ongoing concern, not separate phase |

---

## Risk Mitigations (from Codex, applied throughout)

- **Fork debt**: Pattern-clone by default, selective fork with provenance
- **CLI/workbench drift**: Shared schema crate as single source of truth
- **Install conflicts**: Deterministic plan output, explicit apply step
- **Scope creep**: MVP boundaries enforced per phase
- **Rework risk**: Contracts defined alongside first component, not after 10

---

## Verification Strategy

- **Phase 1**: `cargo run -p studio` → window with sidebar, stories, theme toggle
- **Phase 2**: `gpui add button --plan` → valid JSON plan; 4 components in workbench
- **Phase 3**: All 12 components render; token editor reflects live changes
- **Phase 4**: Primitives extracted; components still pass all stories
- **Phase 5**: External app consumes components via `gpui add` + `--apply`
