# Performance Budgets

## Purpose
Quantified timing and resource constraints that set measurable upper bounds on interaction latency, theme operations, build overhead, and component rendering to ensure a responsive desktop experience.

## Requirements
- Enforce interaction latency budget: primary hover/click/keyboard feedback within one 60Hz frame (~16.6ms) on reference hardware (NFR-008)
- Enforce theme switching budget: workbench theme/token switch within 100ms on reference hardware (NFR-009)
- Enforce build overhead budget: component registration/registry generation adds no more than ~2 seconds to typical debug iteration loops (NFR-010)
- Enforce plan generation budget: single-component install plan completes in sub-second to low-second range (NFR-003)
- Require release-mode measurements using Zed-style workflows (`ZED_MEASUREMENTS=1`) for performance gates (NFR-007)
- Prohibit unapproved regressions versus baseline implementation for interaction latency and frame behavior (NFR-007)
- Require virtualized structures (table, list, tree, command results) to demonstrate bounded rendering under large datasets
- Record performance evidence as part of the Component Acceptance Checklist

## Constraints
- Measurements must be taken in release mode (`--release`), not debug mode
- macOS is the primary measurement platform (NFR-011)
- Performance gates block component acceptance — no exceptions
- Budgets are approximate targets on reference hardware, not hard CI thresholds [inferred]

## Acceptance Criteria
1. At least one component has documented release-mode performance evidence (AC-008)
2. Theme toggle in workbench completes within observable 100ms budget
3. `gpui add <component> --plan` completes within low-second range
4. No accepted component introduces unapproved interaction latency regressions
5. Performance evidence format is defined and referenced by component contracts

## References
- Reqs Section 5: Performance gates in Component Acceptance Checklist
- Reqs Section 9: NFR-003, NFR-007, NFR-008, NFR-009, NFR-010
- Reference: `.refs/zed_gpui_refs/zed-main/docs/src/performance.md` — Zed measurement workflows
