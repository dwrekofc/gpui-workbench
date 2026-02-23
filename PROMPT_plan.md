<!-- description: Read-only gap analysis — compare specs against code, update plan -->
0a. Study `specs/*` with up to 250 parallel Sonnet subagents to learn the application specifications.
0b. Study @IMPLEMENTATION_PLAN.md (if present) to understand the plan so far.
0c. Study `crates/*` with up to 250 parallel Sonnet subagents to understand shared crates, libraries & components.
0d. For reference, the application source code is in `crates/*` and `apps/*`. This is a Cargo workspace.

1. Study @IMPLEMENTATION_PLAN.md (if present; it may be incorrect) and use up to 500 Sonnet subagents to study existing source code in `crates/*` and `apps/*` and compare it against `specs/*`. Use an Opus subagent to analyze findings, prioritize tasks, and create/update @IMPLEMENTATION_PLAN.md as a bullet point list sorted in priority of items yet to be implemented. Ultrathink. Consider searching for TODO, minimal implementations, placeholders, skipped/flaky tests, and inconsistent patterns. Study @IMPLEMENTATION_PLAN.md to determine starting point for research and keep it up to date with items considered complete/incomplete using subagents.

IMPORTANT: Plan only. Do NOT implement anything. Do NOT assume functionality is missing; confirm with code search first. Shared logic belongs in dedicated crates under `crates/`. Prefer consolidated, idiomatic implementations there over ad-hoc copies.

ULTIMATE GOAL: We want to achieve a fully functional GPUI component workbench and CLI that delivers the Core-12 components (Phase 1) with deterministic install/upgrade, story-based validation, and Zed-aligned theming. Consider missing elements and plan accordingly. If an element is missing, search first to confirm it doesn't exist, then if needed author the specification at specs/FILENAME.md. If you create a new element then document the plan to implement it in @IMPLEMENTATION_PLAN.md using a subagent.

Respect delivery phase ordering from specs. If a `delivery-gates.md` spec exists, do not plan Phase N+1 items until Phase N gate criteria are met in the codebase. Prioritize items within the current active phase.

When a spec includes adoption dispositions (Reuse/Fork/Rewrite) in its Constraints, plan the implementation approach accordingly: Reuse = install/copy with minimal changes; Fork = copy then modify to meet constraints; Rewrite = implement from scratch using referenced patterns.
