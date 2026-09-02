# Statekit
An immutable state transition validator for applications that model workflow as data.

## Purpose
Many applications model workflows such as order processing, ticketing, or document approval.

When those workflows are configuration-driven or stored as data rather than hard-coded enums and `match` statements, validating legal transitions becomes repetitive.

Statekit provides an immutable state-machine definition that validates whether a transition is permitted.

## Why use Statekit?

Statekit is intended for applications where states are not known at compile time.

Examples include:

- workflows loaded from configuration
- user-defined business processes
- state machines stored in a database
- plugins that define additional states

## Status

Statekit is under active development.

Current release: v0.3.0.

Statekit follows semantic versioning. As a pre-1.0 crate, its public API may evolve between minor releases.

## Installation

```toml
[dependencies]
statekit = "0.3"
```

## Examples

### Validation Example

```rust
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

    let result = machine.validate_transition("queued", "running")?;

    assert!(machine.can_transition("queued", "running"));
    assert!(!machine.can_transition("queued", "completed"));

    Ok(())
}
```

### Inspecting a Machine

Machines can be inspected without exposing their internal storage.

```rust
for source in machine.sources() {
    println!("{source}");

    for target in machine.targets_from(source) {
        println!("  -> {target}");
    }
}
```

`targets_from()` returns an empty iterator when a state has no outgoing transitions, including unknown states and states that appear only as transition targets.

```rust
for state in machine.states() {
    println!("{state}");
}
```

`Machine::transitions()` exposes immutable `Transition` values with source and target state names.

```rust
for transition in machine.transitions() {
    println!(
        "{} -> {}",
        transition.source(),
        transition.target()
    );
}
```

Iteration order is unspecified.


## Invariants

- State names must not be empty or consist entirely of whitespace.
- State names must not begin or end with Unicode whitespace.
- State names are case-sensitive.
- Self-transitions are rejected.
- Cycles between distinct states are permitted.
- A machine must contain at least one transition.
- Duplicate transitions between the same source and target are stored as a single logical transition.

## Validation

`try_allow()` validates state names and transition relationships when they are added.

`build()` validates machine-level requirements, including that at least one transition exists.

## Features

### Error Handling

All public Statekit errors implement `std::error::Error`.

### Immutability

Once constructed, a machine cannot be modified.

This allows a machine definition to be reused safely without callers mutating its transition structure.

## What it is Not

Statekit is not:
- a process engine
- a policy engine
- a pathfinding library
- a workflow engine

Statekit can be used as a building block for these kinds of systems, but intentionally does not implement them.

## Documentation

- [Statekit Specification](docs/statekit-specification.md) — domain definitions and invariants
- [Migration Guide](MIGRATION.md) — guidance for upgrading between releases
- [Changelog](CHANGELOG.md) — notable changes by release

## License
MIT

