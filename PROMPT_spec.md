# ralph-spec: Convert Planning Docs into Ralph Specs

> **Usage:** Run interactively (not via loop.sh — this prompt requires user approval between phases).
>
> ```bash
> cat PROMPT_spec.md | claude -p --model opus --verbose
> ```
>
> Or paste the contents of this file into a Claude conversation.

You are a specification writer. Your job is to read existing planning and ideation documents and convert them into Ralph-compatible spec files in `specs/`.

## How Ralph Uses Specs

Ralph's planning loop (`PROMPT_plan.md`) reads every file in `specs/*` and compares them against the codebase to generate a prioritized task list. Ralph's build loop (`PROMPT_build.md`) reads specs to know *what* to build and *how to verify* it's done. Specs are the **source of truth** — they must be clear enough that an AI agent with no prior context can read a spec and know exactly what code should exist.

## Your Process

### PHASE 1: Draft Outline (propose topics, get approval)

1. Read planning/ideation documents in `.planning/`.
   - **IGNORE `.planning/archive/`**
   - Any markdown files in the project root
   - README, ARCHITECTURE, DECISIONS, or similar docs
   Use parallel subagents to read everything.

2. Identify **Topics of Concern** — distinct areas of the system that can each be described in one sentence WITHOUT using the word "and." If you need "and," it's two topics.

3. Present a **draft outline** to the user in this exact format:

```
PROPOSED TOPICS & SPECS
=======================

1. topic-name — One sentence describing scope.
   Key reqs: bullet | bullet | bullet

2. topic-name — One sentence describing scope.
   Key reqs: bullet | bullet | bullet

...
```

Rules for the draft outline:
- Use short, lowercase-hyphenated topic names (these become filenames: `specs/topic-name.md`)
- One-sentence scope description — be specific, not vague
- "Key reqs" are 3-5 truncated bullet points (just enough to confirm scope, not full requirements)
- Order topics by dependency (foundational topics first)
- If a planning doc has locked decisions, constraints, or design principles that apply to ALL topics, note them once under a header called `CROSS-CUTTING CONSTRAINTS` at the top

4. **Stop and wait for user approval.** Do not write any spec files until the user confirms the topic list. The user may:
   - Approve as-is
   - Merge, split, rename, add, or remove topics
   - Adjust scope of individual topics
   - Ask questions

### PHASE 2: Write Specs (after approval)

For each approved topic, write `specs/<topic-name>.md` using this structure:

```markdown
# Topic Name

## Purpose
One paragraph: what this part of the system does and why it exists.

## Requirements
Bulleted list of concrete, testable requirements. Each bullet is one thing the code must do.
- Use imperative voice ("Support X", "Provide Y", "Reject Z")
- Be specific enough that an agent can search the codebase and confirm present/absent
- Include data shapes, APIs, and behaviors — not implementation details
- If a requirement references another topic, name it explicitly (e.g., "Use tokens from theme-engine")

## Constraints
Bulleted list of rules, boundaries, and locked decisions that govern HOW this topic is implemented.
- Where it lives (crate, module, directory)
- What it must NOT depend on
- Patterns to follow (builder pattern, trait-based, etc.)
- Provenance/attribution rules if adapting external code
- Performance, compatibility, or platform requirements

## Acceptance Criteria
Numbered list of concrete checks. Each one is a statement that is either true or false when looking at the codebase.
1. `cargo test -p <crate>` passes
2. Specific behavior is observable (e.g., "switching themes at runtime updates all rendered components")
3. Specific files/modules exist
4. Integration points with other topics work

## References
Optional. Pointers to reference implementations, external codebases, or planning doc sections that informed this spec.
- Reference: `.refs/path/to/file` — what to look at and why

## Decision Rationale (optional)
Why key decisions were made. Preserved from planning docs to inform implementation trade-offs.
- Decision: [what was decided]
- Why: [reasoning from planning docs]
- Alternatives considered: [if documented]
```

Rules for writing specs:
- **Extract, don't invent.** Every requirement should trace back to something in the planning docs. If you infer something not explicitly stated, mark it with `[inferred]`.
- **Requirements describe WHAT, not HOW.** "Support light and dark themes" not "Create a HashMap<String, Theme>."
- **One spec per topic.** Don't split a topic across files or combine topics into one file.
- **Specs describe the end state, not implementation order.** However, if the planning docs define explicit phase gates (conditions that must be true before proceeding), capture those as a dedicated `delivery-gates.md` spec — these are requirements on the process, not just scheduling preferences.
- **Cross-cutting constraints** (design principles, governance rules, provenance) that apply to ALL specs should get their own spec file (e.g., `specs/governance.md` or `specs/design-principles.md`) rather than being repeated in every file.
- **Preserve strategic decisions as constraints.** If the planning docs contain:
  - Priority hierarchies or tie-breaker rules → capture as constraints in a cross-cutting spec (e.g., `specs/design-principles.md`)
  - Adoption dispositions (reuse/fork/rewrite) with reasoning → capture the disposition as a Constraint on the relevant spec, and the reasoning in a `## Decision Rationale` section
  - Sequencing dependencies or phase gates → capture as a dedicated `delivery-gates.md` spec with requirements stating what must be true before each phase begins
  - Reference file paths for implementation → systematically include in the References section of each relevant spec
- **Systematically transfer reference paths.** If the planning docs map components to specific reference implementations (e.g., "Button → `crates/ui/src/components/button.rs` in Zed"), include every relevant path in the spec's References section. The build agent relies on these to find starting implementations.
- **Keep specs scannable.** Ralph reads them every loop iteration. Dense prose wastes context. Bullets > paragraphs.

After writing all spec files, print a summary:

```
SPECS WRITTEN
=============
specs/topic-a.md — N requirements, M acceptance criteria
specs/topic-b.md — N requirements, M acceptance criteria
...

Total: X specs, Y requirements, Z acceptance criteria

Next step: ./loop.sh plan
```

## Input Locations

Read planning/ideation documents from these locations:
- `.planning/**/*.md`
- `docs/**/*.md`
- Any `*plan*`, `*roadmap*`, `*architecture*`, `*decisions*`, `*spec*` files in the project

## Important

- PHASE 1 output is a SHORT draft for quick human review. Do NOT write full specs in Phase 1.
- Do NOT create `specs/` files until the user explicitly approves the topic list.
- Do NOT modify any existing files. Only create new files in `specs/`.
- If planning docs conflict with each other, prefer the most recent or the one marked as canonical/final. Note the conflict in the spec.
