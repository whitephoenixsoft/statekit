# Statekit Migration Guide

This document describes breaking and behaviorally significant changes between Statekit releases and provides guidance for migrating existing code.

The newest migration information appears first.

## 0.3 -> 0.4

Version 0.4 adds stateful traversal through `MachineInstance`. The release is additive and requires no changes to existing 0.3 code.

### New `MachineInstance` for stateful traversal of `Machine`

Version 0.4 introduces `MachineInstance` to allow for traversal of `Machine` states.

```rust
let machine = Machine::builder()
    .try_allow("draft", "approved")?
    .try_allow("approved", "draft")?
    .build()?;
    
let mut instance = machine.instance("draft")?;
assert_eq!(instance.state(), "draft");

instance.transition_to("approved")?;
assert_eq!(instance.state(), "approved");
assert!(instance.can_transition_to("draft"));

instance.transition_to("draft")?;
assert_eq!(instance.state(), "draft");
assert!(instance.can_transition_to("approved"));
```

Two `MachineInstance` can also be compared for equality. As long as the machine definitions are equal, the `MachineInstance` will be equal when they share the same traversal position in its `Machine`.

```rust
let machine1 = Machine::builder()
    .try_allow("draft", "approved")?
    .build()?;
    
let machine2 = Machine::builder()
    .try_allow("draft", "approved")?
    .build()?;

// Different machine definitions
let instance1 = machine1.instance("draft")?;
let instance2 = machine2.instance("draft")?;

assert_eq!(instance1, instance2);

// Same machine definition
let instance3 = machine1.instance("draft")?;
assert_eq!(instance1, instance3);
```
#### `StateError::UnknownInitialState` for `MachineInstance` errors 

The initial state of a `MachineInstance` must already exist in `Machine`. If initialized with a non-existent state then it will return `StateError::UnknownInitialState`.

```rust
let machine = Machine::builder()
    .try_allow("draft", "approved")?
    .build()?;
    
let result = machine.instance("unknown");

if let Err(StateError::UnknownInitialState { state }) = result {
    println!("Failed to initialize with {}!", state);
}
```
### `Machine::is_terminal` for stateless terminality queries

With the introduction of traversing `Machine`, `Machine::is_terminal(state)`  has been added to determine if the state has any outgoing transitions .

```rust
let machine = Machine::builder()
    .try_allow("draft", "approved")?
    .build()?;
    
assert!(!machine.is_terminal("draft"));
assert!(machine.is_terminal("approved"));
    
let instance = machine.instance("draft")?;
assert!(!instance.is_terminal());

instance.transition_to("approved")?;
assert!(instance.is_terminal());
```

### Migration summary 

There is no need to change existing code for this migration. All changes are additive and allows for:

- creating an instance over the machine definition 
- determining whether a state is terminal

## 0.2 -> 0.3

Version 0.3 introduces first-class transition inspection and simplifies transition-target queries.

### Transitions can be enumerated directly

Version 0.3 introduces the public `Transition` type and `Machine::transitions`, allowing callers to inspect the directed transitions in a machine directly.

```rust
for transition in machine.transitions() {
    println!("{} -> {}", transition.source(), transition.target());
}
```

A `Transition` exposes its source and target state names through `source()` and `target()`.

Transitions are immutable and are exposed through `Machine::transitions()`.
Machine definitions continue to be constructed through `MachineBuilder`.

### `Machine::targets_from` now returns an iterator directly

Version 0.3 changes `Machine::targets_from` to return an iterator directly rather than `Option<impl Iterator>`.

When the source has no outgoing transitions, the returned iterator is empty. This includes unknown states and states that appear only as transition targets.

Version 0.2:
```rust
if let Some(targets) = machine.targets_from("source") {
    for target in targets { 
        println!("{target}");
    }
}
```

```rust
if machine.targets_from("completed").is_none() {
    // ...
}
```

Version 0.3:
```rust
let targets = machine.targets_from("source");

for target in targets { 
    println!("{target}");
}
```

```rust
let mut targets = machine.targets_from("source");

if targets.next().is_none() {
    println!("no outgoing transitions");
}
```

```rust
let targets: Vec<_> = machine.targets_from("source").collect();

if targets.is_empty() {
    println!("no outgoing transitions");
}
```

### `StateError::AmbiguousStateName` contains a `state` field
 
`StateError::AmbiguousStateName` now preserves the invalid state name in a `state` field, allowing callers to report the exact value that contained leading or trailing Unicode whitespace.

Version 0.2:
```rust
match err {
    StateError::AmbiguousStateName => { println!("{err}"); }
    _ => {}
}
```

Version 0.3:
```rust
match err {
    StateError::AmbiguousStateName { state } => { println!("{state}"); }
    _ => {}
}
```

### `MachineBuilder` no longer implements `Default` or `PartialEq`

Version 0.3 removes the `Default` and `PartialEq` implementations from `MachineBuilder`.

Machine builders should be created through `Machine::builder()` rather than `MachineBuilder::default()`.

Version 0.2:

```rust
let builder = MachineBuilder::default();
```

Version 0.3:

```rust
let builder = Machine::builder();
```

`PartialEq` was also removed because equality between builders is not part of Statekit's public domain contract. Code that directly compares `MachineBuilder` values must be updated to verify the resulting machine or other observable behavior instead.

Both trait removals are breaking changes.

### Migration summary

Most 0.2 consumers need to update code that handles `Machine::targets_from` or exhaustively matches `StateError::AmbiguousStateName`.

- `Machine::targets_from` now returns an iterator directly. No outgoing
  transitions are represented by an empty iterator.
- `StateError::AmbiguousStateName` now contains the invalid state name.
- `MachineBuilder::default()` is no longer available; use `Machine::builder()`.
- `MachineBuilder` no longer implements `PartialEq`.
- `Machine::transitions()` can be used to inspect transitions directly.

## 0.1 -> 0.2

Version 0.2 strengthens Statekit's domain invariants and moves validation closer to the point where state names and transitions are created.

### `MachineBuilder::allow` is deprecated

`MachineBuilder::allow` has been deprecated in favor of `MachineBuilder::try_allow`.

Version 0.1:
```rust
let machine = Machine::builder()
    .allow("queued", "running")
    .allow("running", "completed")
    .build()?;
```

Version 0.2:
```rust
let machine = Machine::builder()
    .try_allow("queued", "running")?
    .try_allow("running", "completed")?
    .build()?;
```

`try_allow` validates the transition immediately and returns `Result<MachineBuilder, StateError>`.

The deprecated `allow` method remains available for compatibility but panics when given an invalid transition. New code should use `try_allow`.

> **Important:** Although `allow` remains available for source compatibility, its error behavior has changed. In 0.1, invalid transitions were generally rejected by `build()`. In 0.2, invalid input passed to `allow` causes an immediate panic. Code that handles invalid or externally supplied state names should migrate to `try_allow`.

### Validation now occurs when transitions are added

In 0.1, invalid transition definitions could be stored temporarily in `MachineBuilder` and were rejected by `build()`.

In 0.2, `try_allow` validates state names and transition-level invariants before storing the transition.

As a result, errors such as invalid state names and self-transitions are returned by `try_allow` rather than being deferred until `build`.

`build()` is responsible for machine-level validation, including requiring at least one transition.

### State-name validation is stricter

Version 0.2 introduces stronger state-name invariants. This is a behavioral compatibility change: state names accepted by 0.1 may be rejected by 0.2.

A state name:

- must contain at least one non-whitespace character;
- must not begin with Unicode whitespace;
- must not end with Unicode whitespace;
- remains case-sensitive;
- may contain whitespace internally;
- supports UTF-8.

For example:

```
"running"          valid
"In Progress"      valid
" running"         invalid
"running "         invalid
"   "              invalid
```

Applications that previously used state names containing leading or trailing whitespace must correct those values before upgrading.

Statekit rejects invalid names rather than silently trimming or normalizing them.

### `StateError::AmbiguousStateName` is added

State names containing leading or trailing whitespace now produce:

```
StateError::AmbiguousStateName
```

Consumers that exhaustively match `StateError` variants may need to handle this additional variant.

### Self-transitions fail earlier

Self-transitions remain invalid:

```
running -> running
```

When using `try_allow`, the error is now returned when the transition is added rather than when the machine is built:

```rust
let result = Machine::builder()
    .try_allow("running", "running");

assert_eq!(
    result,
    Err(StateError::SelfTransition {
        state: "running".to_owned(),
    })
);
```

Cycles between distinct states remain valid:

```
queued  -> running
running -> queued
```

### `Machine::targets` is deprecated in favor of `Machine::targets_from`

`Machine::targets` remains available in 0.2 for compatibility, but is deprecated. Existing code continues to compile with a deprecation warning.

New code should use `targets_from`.

Version 0.1:

```rust
machine.targets("running");
```

Version 0.2:

```rust
machine.targets_from("running");
```

`targets_from` returns the states directly reachable from the specified source state.

### Source states can be enumerated

Version 0.2 adds a source-state iterator:

```rust
for source in machine.sources() {
    if let Some(targets) = machine.targets_from(source) {
        for target in targets {
            println!("{source} -> {target}");
        }
    }
}
```

`sources()` returns states that have at least one outgoing transition. States that appear only as transition targets are not included. The iteration order is unspecified.

This complements `targets_from` and allows callers to inspect the transition graph without exposing Statekit's internal storage representation.

### Unique states can be enumerated

Version 0.2 adds an iterator over all unique states in the machine:

```rust
for state in machine.states() {
    println!("{state}");
}
```

`states()` returns every unique state that appears as either the source or target of a transition. Each state is returned once, and iteration order is unspecified.

Because a valid machine contains at least one transition and self-transitions are prohibited, a machine contains at least two unique states.

### Internal architectural changes

The following changes are important to Statekit's architecture but do not themselves form part of the public storage contract.

#### State names are represented internally by `StateName`

In version 0.2, Machine definitions store validated domain values rather than raw strings.

Conceptually:

```rust
HashMap<StateName, HashSet<StateName>>
```

This ensures that invalid state names cannot exist inside a successfully constructed builder or machine.

The internal storage representation is not part of Statekit's public API and may change in future releases.

#### Validation responsibilities are separated

Version 0.2 establishes clearer invariant boundaries:

- `StateName` guarantees a valid state name.
- `MachineBuilder::try_allow` guarantees a valid transition relationship.
- `MachineBuilder::build` guarantees that a machine contains at least one transition.
- `Machine` represents an immutable, valid machine definition.

This separation is intended to keep invalid domain states from propagating through the library.

### Migration summary

Most 0.1 consumers should migrate builder calls from:

```rust
.allow(from, to)
```

to:

```rust
.try_allow(from, to)?
```

and transition-target queries from:

```rust
.targets(from)
```

to:

```rust
.targets_from(from)
```

Applications should also verify that persisted or externally supplied state names do not contain leading or trailing whitespace.

Consumers that exhaustively match `StateError` should add handling for `StateError::AmbiguousStateName`.
