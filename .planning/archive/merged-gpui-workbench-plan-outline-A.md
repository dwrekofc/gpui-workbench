# Merged GPUI Workbench Plan (High-Level Outline)

## 1) Strategy

Build a dual-track monorepo where:
- CLI is the control plane (`plan`/`apply`, idempotent, machine-readable).
- Workbench is the fast visual loop (first demo early, then validate components/themes).

## 2) Locked Decisions

- Dual-track delivery with priority rule: CLI leads, workbench validates.
- Hybrid consumption: source-copy default + optional crate-based path.
- Clean-room primitives with pattern-clone/fork hybrid (with strict provenance).
- Workbench MVP: viewer + token editor.
- Strong governance: ADRs, phase gates, risk checks.
- Bootstrap app template: **deferred**.

## 3) Explicit Compromises

- Keep Claude’s fast demo scaffold detail (crate/file structure, concrete shell path).
- Keep Codex’s governance rigor (plan-first CLI, idempotency, provenance, deferred template).
- Primitive strategy:
  - Build styled components quickly.
  - Extract primitives only after 2+ real component consumers.
- Workbench scope:
  - Build concrete shell now.
  - Hold boundary at viewer + token editor for MVP.

## 4) Phases (Short Form)

1. Phase 0: Guardrails + Skeleton
- Monorepo structure, registry contracts, provenance schema, CLI JSON contract.
- Add template abstraction interfaces; no concrete bootstrap template chosen.

2. Phase 1: First Demo Slice (fast)
- Zed-style shell, story viewer, theme switch, basic token editor.
- CLI vertical slice: `gpui add button --plan` + `gpui apply`.

3. Phase 2: CLI Core (priority)
- `add/update/remove`, conflict detection, idempotency guarantees.
- Provenance capture on all install/update actions.

4. Phase 3: Component Expansion
- Core-12 baseline, then expand count in waves.
- Primitive extraction gates enforced (no speculative primitives).

5. Phase 4: Hardening + Adoption
- Snapshot baselines, `doctor`, cargo-first packaging, downstream trial apps.

## 5) Deferred Bootstrap Template Policy

- Deferred decision: choose `gpui init` base later.
- Immediate requirement: all installer logic targets a `TemplateAdapter` interface, not fixed paths.
- Decision trigger: pick concrete template only after CLI plan/apply + 3 components are stable.

## 6) First Demo Success Check

- Workbench opens with component/story navigation.
- Token edit updates preview live.
- CLI plan/apply installs one component deterministically.
- Installed component renders in workbench with active theme tokens.
