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

Stable API for v0.1.

Future releases will expand functionality while maintaining semantic versioning.

## Installation

```toml
[dependencies]
statekit = "0.1"
```

## Example

```rust
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let result = machine.validate_transition("queued", "running");

    assert!(result.is_ok());

    Ok(())
}
```

## Invariants
- Validation occurs when `build()` is called.
- State names must not be empty.
- State names are case-sensitive.
- Self-transitions are rejected.
- A machine must define at least one transition.

## Features

### Error
All public errors implement `std::error::Error`

### Immutabilitu
Once constructed, a machine cannot be modified.

This makes it inexpensive to share safely between threads and application components.

## What it is Not

Statekit is not:
- a process engine
- a policy engine
- pathfinding code
- workflow engine

It can be used as a building block for these kinds of systems, but it intentionally does not implement them.

## License
MIT

