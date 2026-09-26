# Statekit Implementation Roadmap
State: Active 
Current Version: v0.4.x

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

### Phase 5 - Stateful execution (v0.4)

**Goal** Add a machine instance for relative transitions.

Go from:
> "Can this transition occur?"

To:
> "I am currently in this state."

Consider:
- `MachineInstance` for stateful execution
- whether `Machine` should remain the definition type or be renamed
- Adding memory benchmarks preparation for indexing changes.

Stateful execution:
- MachineInstance
- current state
- transition_to()
- can_transition_to()
- failed transitions do not mutate state
- settle ownership model

### Phase 6 - Multiple Representations (v.5)

> Should Statekit's public runtime model distinguish symbolic state names from internal/runtime state identity?

Consider:
A. Keep strings as state identity
B. Make Machine generic over state identity
C. Keep string-oriented construction but lower to Statekit-owned StateId

Todo:
Runtime state identity / lowering
- distinguish symbolic state names from runtime identity
- investigate StateId-style representation
- decide whether public APIs should expose IDs
- decide how name ↔ ID mapping works
- evaluate whether strings remain the construction surface
- keep generic Machine\<S\> as an alternative to compare, not the default assumption

### Phase 7 - Optimization (v0.6)

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
### Phase 8 - API Freeze (v0.9)

No new features.

Answer questions:
- Should this type exist?
- Is this the right name?
- Should this method return `Result`?
- Should this be &str or `StateName`?
- Is this the API I'd be happy maintaining for five years?


### Phase 9 - Production polish (v1.0)

Only when the API seems acceptable.

