# Statekit
A simple immutable state-machine that validates allowed predefined transitions. 

## Purpose
There are times when an application decides to uncouple state from code and treat it as data. 

This crate was created so that this work can easily be bootstrapped into the code.

## Status

IN DEVELOPMENT

## Installation

TBD

## Example

```rust
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.transition("queued", "running");

    assert!(state.is_ok());

    Ok(())
}
```

## Error Behavior
TBD

## What it is Not

Statekit is not:
- a process engine
- a policy engine
- pathfinding code

It can support these effort but will not implement them.

## License
MIT

