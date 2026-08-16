# Statekit Implementation Roadmap
State: Active (Locked)
Current Version: v0.1

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

### Phase 3 - Better domain model (v0.3)

**Goal:** Refactor for better maintenance

Introduce:
- `Transition`
- `Transitions`

These will help correct the domain concepts and start reading like English.

### Phase 4 - Stateful execution (v0.4)

**Goal** Add a machine instance for relative transitions.

Go from:
> "Can this transition occur?"

To:
> "I am currently in this state."

Define:
- `MachineInstance` for execution
- `MachineDefinition` to store the allowed transitions

### Phase 5 - API Freeze (v0.9)

No new features.

Answer questions:
- Should this type exist?
- Is this the right name?
- Should this method return `Result`?
- Should this be &str or `StateName`?
- Is this the API I'd be happy maintaining for five years?



### Phase 5 - Production polish (v1.0)

Only when the API seems acceptable.

