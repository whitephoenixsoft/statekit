---
project: statekit
category: planning
version: 0.4
status: active
review-status: needs-review
---

# Statekit Implementation Roadmap

## Purpose
To define how statekit will evolve over next versions.

## Phases

### Phase 1 - Publishable MVP (v0.1.x) -- COMPLETED

**Goal:** A small, polished crate that does one thing well.

Focus on:
- idiomatic Rust
- Excellent documentation
- Tests
- Clippy clean
- Initial code and file documentation
- README
- CI (GitHub Actions)
- crates.io publication

Expose Models:
- Machine
- MachineBuilder
- StateError

### Phase 2 - Strengthen the model (v0.2) -- COMPLETED

**Goal:** Introduce stronger invariants; make invalid state impossible.

- Further define `StateName`
- `TryFrom`
- Finalize `try_allow()`
- earlier validation
- deprecate `allow()`
- deprecate `targets()`
- add `targets_from()`
- add `sources()`
- add `states()`

### Phase 3 - Better domain model (v0.3) -- COMPLETED

**Goal:** Refactor for better maintenance

Introduce:
- `Transition`
- `Transitions`

These will help correct the domain concepts and start reading like English.

### Phase 4 - Benchmarking -- COMPLETED
**Goal:** Introduce benchmarking to see if it's worth adding indexing

Introduce:
- Benchmarking to measure current design properly
- Property testing 

### Phase 5 - Stateful execution (v0.4) -- COMPLETED

**Goal** Add a machine instance for relative transitions.

Go from:
> "Can this transition occur?"

To:
> "I am currently in this state."

Consider:
- `MachineInstance` for stateful execution
- whether `Machine` should remain the definition type or be renamed
- Adding memory benchmarks preparation for indexing changes.
- Benchmarking for instances

Stateful execution:
- MachineInstance
- current state
- transition_to()
- can_transition_to()
- failed transitions do not mutate state
- settle ownership model

### Phase 6 - Tighten Product Direction (v0.4.1)

**Goal:** Review Core Project document and align direction with evidence.

Consider:
- add/modify tests to cover core to invariants
- add traceability to application invariants
- additional doc tests 
- make readme and migration examples part of the executable tests

### Phase 7 - Multiple Representations (v.5)

> Should Statekit's public runtime model distinguish symbolic state names from internal/runtime state identity?

Consider:
- Explore one shared structural core with string and generic identity-facing APIs.
- Removing self transition validation to as a way to better support generic structures

Todo:
Runtime state identity / lowering
- keep original Machine public API
- define generic IdentityMachine\<S\> as an alternative to compare, not the default assumption
- reference MachineInner and make it generic 
- change Machine to call MachineInner\<S\> with StateName as the type

### Phase 8 - Data Structure Clean up (v0.5.1)

**Goal:** refine project core to focus statekit as a reusable data structure.

Consider: 
- Fix tests and documents to make statekit allow self transitions

### Phase 9 - Optimization (v0.6)

**Goal:** Runtime-oriented internal representation
Evidence-driven indexing and memory tradeoffs

Consider:
- Performance/indexing changes for that benchmarks justify them.

Todo:
Runtime indexing / representation optimization
- transition membership index
- adjacency/source index
- state membership index
- cached states/sources
- memory benchmarking
- construction/runtime/memory tradeoff analysis
### Phase 10 - API Freeze (v0.9)

No new features.

Answer questions:
- Should this type exist?
- Is this the right name?
- Should this method return `Result`?
- Should this be &str or `StateName`?
- Is this the API I'd be happy maintaining for five years?


### Phase 11 - Production polish (v1.0)

Only when the API seems acceptable.

Consider:
- Post 1.x direction

