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

Current release: v0.4.

Statekit follows semantic versioning. As a pre-1.0 crate, its public API may evolve between minor releases.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
statekit = "0.4"
```

## Examples

### Validation Example

There are two ways to use Statekit. 

It can be used statelessly:

```rust
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

    machine.validate_transition("queued", "running")?;

    assert!(machine.can_transition("queued", "running"));
    assert!(!machine.can_transition("queued", "completed"));

    Ok(())
}
```

Or it can be used statefully:

```rust
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

   let mut instance = machine.instance("queued")?;

   instance.transition_to("running")?;

   assert!(instance.can_transition_to("completed"));
   assert!(!instance.can_transition_to("queued"));

   Ok(())
}
   
```

### Inspecting a Machine

Statekit exposes transition information so applications can build additional functionality around their state-machine definitions.

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

### Terminality

`Machine::is_terminal(state)` determines whether a specified state has outgoing transitions. `MachineInstance::is_terminal()` checks the instance's current state.

```rust
let machine = Machine::builder()
        .try_allow("queued", "running")?
        .build()?;

let instance = machine.instance("running")?;

assert!(machine.is_terminal("running"));
assert!(instance.is_terminal());
```

## Invariants

- State names must not be empty or consist entirely of whitespace.
- State names must not begin or end with Unicode whitespace.
- State names are case-sensitive.
- Self-transitions are rejected.
- Cycles between distinct states are permitted.
- A machine must contain at least one transition.
- Duplicate transitions between the same source and target are stored as a single logical transition.
- A transition attempt is rejected unless it is allowed by the machine definition.
- Instances must be initialized with an existing state.
- Instances can transition only to target states permitted by the machine definition.

## Validation

`try_allow()` validates state names and transition relationships when they are added.

`build()` validates machine-level requirements, including that at least one transition exists.

`validate_transition()` and `transition_to()` validate transitions from the allowed list.

## Features

### Error Handling

All public Statekit errors implement `std::error::Error`.

### Immutability

Once constructed, a machine cannot be modified.

This allows a machine definition to be reused safely without callers mutating its transition structure.

### Concurrency Support

An instance can be created from the stateless machine definition to traverse the states permitted by the machine definition. 

These instances share the machine definition and can work concurrently even if the machine definition handle has been dropped. 

Synchronization of mutable instances is the responsibility of the host application.

### Semantic Equality

Two machines are equal when their machine definitions are equal. Two instances are equal when they reference equal machine definitions and are on the same traversal state.

## What It Is Not

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
- [Benchmarks](docs/benchmarks/README.md) — performance measurements and methodology for Statekit

## Acknowledgements

Statekit is built with help from the Rust ecosystem and uses the following crates:

- [`thiserror`](https://crates.io/crates/thiserror) — ergonomic error definitions
- [`proptest`](https://crates.io/crates/proptest) — property-based testing
- [`criterion`](https://crates.io/crates/criterion) — benchmarking

Thank you to the maintainers and contributors of these projects.

## License
MIT

