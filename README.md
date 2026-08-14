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

Stable API for v0.1.1.

In development of v0.2.0.

Future releases will expand functionality while maintaining semantic versioning.

## Installation

```toml
[dependencies]
statekit = "0.2.0"
```

## Examples
```rust
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

    let result = machine.validate_transition("queued", "running");

    assert!(result.is_ok());

    Ok(())
}
```

## Invariants
- Validation of the transition occurs when `try_allow()` is called.
- Validation of the state machine occurs when `build()` is called.
	- A machine must define at least one transition.
- `StateName` will be used as the basis for the states and will ensure:
	- State names must not be empty.
    	- State names are case-sensitive.
	- State names must start and end with visible characters
- Self-transitions are rejected.

## Features

### Error
All public errors implement `std::error::Error`

### Immutability
Once constructed, a machine cannot be modified.

This makes it inexpensive to share safely between threads and application components.

## Specification

Statekit's domain definitions and invariants are documented in the [Statekit Specification](docs/statekit-specification.md).

## What it is Not

Statekit is not:
- a process engine
- a policy engine
- pathfinding code
- workflow engine

It can be used as a building block for these kinds of systems, but it intentionally does not implement them.

## Documentation

- [Statekit Specification](docs/statekit-specification.md)
- [Migration Guide](MIGRATION.md)
- [Change Log](CHANGELOG.md)
## License
MIT

