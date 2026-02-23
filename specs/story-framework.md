# Story Framework

## Purpose
The trait-based system for registering and rendering component demonstrations with state matrix coverage, enabling systematic visual validation of every component variant and state.

## Requirements
- Define a `Story` trait with methods for name, description, contract, and rendering [observed from code]
- Provide a `StoryRegistry` global that stores all registered stories [observed from code]
- Support story lookup by name and sorted listing [observed from code]
- Provide a `StateMatrix` that generates a grid from a `ComponentContract` showing all variant-by-state combinations [observed from code]
- StateMatrix shall render a header row of state labels, one row per variant, and a token dependency pill list [observed from code]
- Every CLI-installable component shall have a corresponding story (FR-007)
- Stories shall demonstrate variants, sizes, states (including disabled), and interactive behaviors
- Story/state matrix coverage is a quality gate in the Component Acceptance Checklist

## Constraints
- Lives in `crates/story/`
- Depends on `components`, `theme`, and `primitives` crates
- Stories are registered at app startup via `story::init(cx)`
- `StoryRegistry` is stored as a GPUI global
- `StateMatrix` is built from `ComponentContract`, not manually specified [observed from code]

## Acceptance Criteria
1. `Story` trait exists with `name()`, `description()`, `contract()`, and `render_story()` methods
2. `StoryRegistry` can register, list, and look up stories by name
3. All 12 Core-12 components have story implementations
4. `StateMatrix::from_contract()` produces a grid covering all declared states and variants
5. Stories demonstrate variant sections, state sections, and a full state matrix grid

## References
- Reqs Section 5: Quality gates — story/state matrix coverage
- Reqs Section 6.1: MVP In Scope — story rendering with state matrix support
- Reqs Section 8: FR-007
