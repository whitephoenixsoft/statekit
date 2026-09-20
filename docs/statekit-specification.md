# Statekit Specification

Version: 0.4

Status: Foundational

Scope: Statekit domain definitions and invariants

## Purpose

This document defines Statekit and its components.

## Core Principle

Statekit is an immutable state transition validator for applications that model workflow as data.

Its purpose is to provide a foundation for validating dynamic state transitions at runtime. 

Statekit fulfills this purpose by providing an immutable state-machine definition that determines whether a transition is permitted.

## Core Components

The components in this specification describe Statekit's domain model. A component does not necessarily correspond one-to-one with a Rust type in every implementation version.

### Machine

Machine is an immutable value-like machine definition implemented as a shared handle. It represents a definition of the allowed state transitions.

- It is only valid if it contains at least one transition.
- Determines whether a transition from a source state to a target state is allowed.
- Machine must support querying allowed transitions.
- Querying the transitions reachable from a state with no outgoing transitions produces an empty result. This includes states that appear only as transition targets and names that do not occur in the machine.
- Provides access to queries over the machine's transitions.
- Machine has value equality based on its state-machine definition. Shared allocation identity is an internal ownership concern.

#### Invariants and Constraints

- Machine must be immutable.
- Machine will validate a transition based on source and target states.
- There must be at least one transition.
- State names supplied to machine queries are matched exactly and are not trimmed or normalized.
- Two machines with the same edges are equal.

### Machine Instance

Represents a mutable instance of a transitioning state machine.

- Is mutable representation of a state.
- Walks through the states using allowed transitions.
- Ends on a terminal state.
- Supports queries related to walking the state machine.
#### Invariants and Constraints

- The initial state of the instance must be state in the host state machine.
- A failed transition attempt has no observable effect on the instance.
- An instance can only transition through an adjacent edge.
- A state on a target with no outgoing transitions can no longer transition.
- Two instances of the same machine and on the same state are equal.

### Transitions

The collection of transitions.

- Manages storage for each transition.
- `Transitions` is an internal component and is not part of Statekit's public API.
- Supports queries related to an individual transition or for the collection.
- Contains unique transitions.
- Two transitions with the same source and target are treated as the same logical transition.
- Two transition collections are equal if they contain a similar list of transition items.

### Transition

Represents a directed transition from a source state to a target state.

- A transition is defined by its source and target states.
- The source and target states must be different.
- Two transitions with the same source and target represent the same logical transition.
- Cycles between distinct states are permitted.
- A transition is immutable.

### Machine Builder

Validates and builds the state-machine.

- Defines transitions between states.
- Enforces transition and state validation before a `Machine` is built.
- Provides an API for constructing a machine definition incrementally.

### State Name

Validates and holds the state name.

- Creates and maintains a valid state.
	- Two states with the same state name represent the same logical state.
	- A state name must not begin or end with Unicode whitespace.
	- A state name must contain at least one non-whitespace character.
- Must support UTF-8 strings.
- `State Name` is an internal component and is not part of Statekit's public API.
- State names are case-sensitive.


## Architecture

```
[Machine] -- Contains --> [Transitions] -- Contains --> [Transition] -- Has Source --> [State Name]
                             -- Has Target --> [State Name]
```
```
[Machine Builder]
      |
      | defines
      v
[Transition]
   |       |
 source   target
   |       |
   v       v
[State Name]

[Machine Builder]
      |
      | builds
      v
   [Machine]
      |
      | contains
      v
[Transitions]
      |
      | contains
      v
[Transition]
```


## Governing Principles 

### Compatibility Principles

Public API changes follow semantic versioning.
Internal storage is not part of the Statekit domain contract.

Strengthening an invariant that causes previously valid input to be rejected is considered a behavioral compatibility change and must be intentional.

### Runtime Principles 

Normal runtime state should be observable without provoking errors. Errors represent invalid operations, not ordinary machine conditions.

### Concurrency Principles 

Statekit makes independent machine instances safe and inexpensive to execute concurrently, while leaving scheduling and synchronization policy to the host application.

### Benchmark Principles 

Statekit may spend additional work during immutable machine construction when doing so materially reduces repeated runtime validation cost, provided memory growth remains bounded and measurable.

## What it is Not

Statekit is not:
- a process engine
- a policy engine
- pathfinding code
- a workflow engine

It can be used as a building block for these kinds of systems, but it intentionally does not implement them.





