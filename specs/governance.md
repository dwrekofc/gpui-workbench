# Governance

## Purpose
Provenance tracking, architectural decision records, and quality gate enforcement that ensure every merged contribution meets the project's integrity, attribution, and evidence standards.

## Requirements
- Maintain `docs/PROVENANCE.md` indexing all copied/adapted code by crate and file
- Every copied/adapted file shall include: source repository URL, source commit hash, source license, summary of local modifications
- Provide ADR templates in `docs/ADR/` for major architecture changes
- Require acceptance checklist results with every merged slice
- Require performance gate evidence with every merged slice
- Require provenance updates when adapted/copied code is included
- Require contract/docs updates when interfaces change
- Enforce WIP limit of max 2 active vertical slices at once
- Perform weekly risk review updates in `.planning/`
- Require an ADR for every major architecture decision
- No phase progression without passing phase exit criteria (NFR-005)
- Every phase must be executable in small vertical slices of 1-3 day units (NFR-004)
- Record gate evidence (test output, perf measurements, checklist results) per phase
- CLI shall write `.provenance.json` files beside installed component files [observed from code]
- Provenance check script exists at `scripts/check-provenance.sh` [observed from code]

## Constraints
- Lives in `docs/` (PROVENANCE.md, ADR/, CONTRACTS.md, ARCHITECTURE.md)
- CI shall block merges that add adapted files without provenance metadata
- ADR format must be lightweight enough for solo-dev workflow
- Provenance is mandatory — no exceptions for "small" adaptations

## Acceptance Criteria
1. `docs/PROVENANCE.md` exists and indexes adapted files by crate and file
2. `docs/ADR/` contains at least one ADR template
3. Every adapted source file in the codebase has provenance metadata (URL, commit, license, modifications)
4. CI configuration includes a provenance check gate

## References
- Reference: `.refs/zed_gpui_refs/zed-main/docs/` — Zed documentation structure patterns
- Plan Section 12: Provenance and Legal Requirements
- Plan Section 13: Governance Model (Solo-Friendly)

## Decision Rationale
- Decision: Strict provenance for all adapted code
- Why: Legal compliance and ability to track upstream changes for updates
- Alternatives considered: Loose attribution (rejected — insufficient for deterministic upgrades and legal safety)
