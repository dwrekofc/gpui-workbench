# Delivery Gates

## Purpose
Phase gate conditions that must be true before the project progresses from one phase to the next. These are requirements on the delivery process itself, not just the code.

## Requirements

### Phase 0.5 Gate (Proof of Concept — Dialog, Select, Tabs)
- Design tokens are frozen from Zed One Dark/One Light and represented in Rust
- Primitives for focus, keyboard, popover, and state management exist and are used by POC components
- Component contracts schema is defined and validated against all 3 POC components
- Dialog, Select, and Tabs are styled, pass the acceptance checklist, and map to frozen tokens
- Theme engine loads themes, switches at runtime, and supports live token editing
- Story framework renders all 3 POC components with state matrix coverage
- Workbench app launches reliably on macOS and displays all 3 POC components in story form
- Registry indexes all 3 POC components from source metadata
- CLI `add`, `plan`, and `apply` work for all 3 POC components
- Plan contract JSON schema is defined and produces deterministic output
- Governance scaffolding is in place (PROVENANCE.md, ADR template, provenance metadata on adapted files)

### Phase 1 Gate (Full Core-12)
- All 9 remaining Core-12 components (Button, Input, Textarea, Checkbox, Radio, Tooltip, DropdownMenu, Popover, Toast) pass acceptance checklist
- Additional primitives extracted where 2+ components share behavior
- CLI `update`, `remove`, `list`, and `doctor` commands are functional
- Full deterministic upgrade flow works across all 12 components
- All 12 components are installable via CLI, rendered in workbench, and have performance evidence
- `gpui init` works through template adapters

### Phase 2 Gate (Full Library + Distribution)
- All 39 remaining components from the gpui-component inventory pass acceptance checklist
- Table, Tree, and Command Palette are delivered with Zed-aligned interaction semantics
- Shared primitives extracted from proven duplication across the full library
- Theme import/export hardened (JSON and TOML)
- Cargo install path is stable and documented
- Homebrew packaging available after Cargo maturity criteria pass
- `gpui init` template expansion supports multiple templates
- At least one external app successfully uses multiple installed components

## Constraints
- No phase progression without passing its exit criteria (NFR-005)
- Gate evidence must be recorded (test results, checklist approvals, perf measurements)
- Gates are evaluated against the acceptance criteria, not subjective judgment

## Acceptance Criteria
1. Each phase gate has a documented checklist that can be evaluated as pass/fail
2. Phase 0.5 gate is met before Phase 1 work begins on remaining 9 components
3. Phase 1 gate is met before Phase 2 work begins on extended library
4. Gate evidence (test output, perf measurements, checklist results) is recorded per phase

## References
- Plan Section 11: Roadmap and Phase Gates
- Plan Section 14: MVP Acceptance Criteria
