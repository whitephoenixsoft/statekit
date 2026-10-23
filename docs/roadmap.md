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
- analysis of product identity and direction (done)
- add traceability to application invariants (done)
- additional doc tests (done)
- add/modify tests to cover core to invariants (done)
- make current readme and migration examples part of the executable tests. (done)
- add historical document examples to preserve traceability. Mark each file with provenance information.

### Phase 7 - Data Structure Conversion (v0.5)

**Goal:** Establish Statekit as a general, immutable state-transition data structure without imposing consumer-specific configuration restrictions.

Consider: 
- updating the project core to remove opinionated restrictions
- adding `try_state` to add an independent list of states
- adding `sources_to` for incoming states
- adding `is_isolated` for orphaned single states
- updating iterators to support new infrastructure
- updating specification to match the new architecture
- adding additional property tests to cover new invariants

Todo:
- Self-transitions are valid
- A machine may have zero transitions
- States may exist independently of transitions
- A completely empty machine is valid
- Machine equality includes both states and transitions
- An instance still requires a known initial state
- Transition endpoints may continue to introduce state implicitly

### Phase 8 - Data Structure Tests and Benchmarks (v0.5.1)

**Goal:** Update property tests and benchmarks

Consider:
- adding new memory benchmarks
- expanding property tests for better coverage
- adding new performance benchmarks
- updating benchmark documents

### Phase 9 - Multiple Representations (v0.6)

**Goal:** Update the public runtime model to support different state identities

Consider:
- exploring one shared structural core with string and generic identity-facing APIs.
- adding additional property tests to cover new implementation

Todo:
Runtime state identity / lowering
- keep original Machine public API
- define generic IdentityMachine\<S\> as an alternative to compare, not the default assumption
- reference MachineInner and make it generic 
- potentially change Machine to call MachineInner\<S\> with StateName as the type

## Phase 10 - Multiple Representation Tests and Benchmarks (v0.6.1)

**Goal:** Update property tests and benchmarks

Consider:
- adding new memory benchmarks
- expanding property tests for better coverage
- adding new performance benchmarks
- updating benchmark documents

### Phase 11 - Optimization (v0.7)

**Goal:** Runtime-oriented internal representation
Evidence-driven indexing and memory tradeoffs

Consider:
- performance/indexing changes for that benchmarks justify them.
- keeping only indexes that justify the tradeoff
- publishing updated benchmark document

Todo:
Runtime indexing / representation optimization. Examples include:
- transition membership index
- adjacency/source index
- state membership index
- cached states/sources
- memory benchmarking
- construction/runtime/memory tradeoff analysis

### Phase 12 - API Freeze (v0.9)

No new features.

Answer questions:
- Should this type exist?
- Is this the right name?
- Should this method return `Result`?
- Should this be &str or `StateName`?
- Is this the API I'd be happy maintaining for five years?


### Phase 13 - Production polish (v1.0)

Only when the API seems acceptable.

Consider:
- publishing post 1.x direction
- removing all deprecated API functions

