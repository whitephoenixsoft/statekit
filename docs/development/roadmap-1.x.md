---
project: statekit
category: planning
version: 1
status: draft
review-status: needs-review
---

# Statekit Implementation Roadmap Post 1.0

## Purpose

To define how statekit will evolve after version 1.0.

All decisions here are likely to change at any time.

## Phases

### Phase 1 - Structural Introspection (v1.1)

**Goal:** Statekit validates, inspects, and safely traverses data-defined state machines.

Plan:
- Update the Project Core to reflect new values and invariants.

Consider:
- roots() - states with zero incoming transitions.
- terminals() - states with zero outgoing transitions.

Feature organization: 
- Consider separating the foundational machine API from optional instance and analysis capabilities using Cargo features. Preserve the full API under default features. Evaluate dependency size, build performance, and feature-combination testing before finalizing the configuration.

### Phase 2 - Analysis Additions (v1.2)

**Goal:** Introduce separate analysis functions for extensive transition queries.

Consider:
 - can_reach(from, to) 
note: would also return true if from and to are equal.

 - reachable_from(state) 
note: will also include itself as a reachable state


### Other possibilities 

These will be deferred as direction becomes clearer.

Methods:
- can_reach_terminal(state)
- has_predecessors(state)
- has_successors(state)
- in_degree(state)
- out_degree(state)
- has_cycles() - machine level check
- cycles()
- is_cyclic(state) - whether state is in a cycle 
- instance.reachable_states()
- instance.can_reach(D)

Approach:
- Compatibility: Preserve existing public methods unless a change is independently justified.
- Performance: Ordinary machine construction and instance traversal should not pay for unused analysis.
- Semantics: Analysis reports structural facts, not domain judgments.
- Reuse: Instance-relative queries should delegate to machine-level analysis.
- Dependencies: Petgraph remains an implementation option, not a requirement.
- Testing: Each analysis feature should have focused unit tests and property tests where useful, with traceability back to the Project Core.


