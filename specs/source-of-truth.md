# Source of Truth Hierarchy

## Purpose
The decision framework that resolves conflicts between upstream influences (Zed, Radix/Base UI, gpui-component, shadcn, Awesome GPUI) by assigning each domain a primary and secondary authority with explicit tie-breaker rules.

## Requirements
- Assign GPUI architecture and rendering correctness to Zed GPUI as primary source
- Assign performance measurement and acceptance to Zed measurement workflows as primary source
- Assign interaction primitives (focus, keyboard, dismiss, state) to Radix + Base UI patterns as primary source
- Assign visual tokens and theme baseline to Zed One Dark/One Light as primary source
- Assign component breadth and implementation bootstrap to gpui-component-main as primary source
- Assign distribution/install UX to shadcn CLI model as primary source
- Assign feature expansion scouting to Awesome GPUI links as primary source
- Enforce tie-breaker order: Zed GPUI correctness/perf > Radix/Base interaction model > gpui-component breadth > shadcn distribution model > Awesome GPUI examples
- Translate web interaction semantics to GPUI desktop idioms — do not copy API shape blindly
- Normalize all adopted code from external sources to internal contracts/tokens before integration
- Use Awesome GPUI for targeted feature spikes only, not as an integration source

## Constraints
- This hierarchy is authoritative — if any spec or implementation conflicts, this hierarchy wins
- Applies to all adoption decisions: Reuse, Fork, and Rewrite dispositions
- Web patterns (React, Tailwind, DOM events) must be translated to GPUI desktop equivalents

## Acceptance Criteria
1. Every component adoption decision references which source-of-truth domain applies
2. No component blindly copies web API shapes without GPUI translation
3. Conflict between sources is resolved by the documented tie-breaker order
4. All adopted code is normalized to internal contracts and frozen tokens

## References
- Reqs Section 2: Highest-Truth Decisions
- Reqs Section 2.1: Influence SWOT Matrix
- Reqs Section 3: Source of Truth by Domain table
- Reqs Section 15: Citations and Reference Index
