---
project: statekit
category: planning
version: 1
status: draft
review-status: needs-review
---

# Statekit Implementation Roadmap

## Purpose
To define how statekit will evolve after version 1.0.

## Phases

### Phase 1 - Realign (v1.1)

**Goal:** Realign statekit to be a validation and analysis library.

Plan:
- Update project core direct new values and Invariants

Consider:
- sources_to(state)
- roots()
- terminals()

### Phase 2 - Analysis Module (v1.2)

**Goal:** Introduce separate analysis module for extensive transition queries.

Consider:
 - can_reach(from, to)
 - reachable_from(state)
 - instance.reachable_states()
 - instance.can_reach(D)

### Other possibilities 

- successors(state)
- predecessors(state)
- can_reach_terminal(state)
- has_predecessors(state)
- has_successors(state)
- in_degree(state)
- out_degree(state)
- has_cycles()
- cycles()
- is_cyclic(state)


