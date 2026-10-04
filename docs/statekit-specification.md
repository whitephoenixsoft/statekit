---
project: statekit
category: specification
category-level: foundational
version: 0.4
status: active
review-status: under-second-review
---
# Statekit Specification

## Purpose

This specification defines the Statekit domain model and the behavioral contracts between its components. It describes the semantics that implementations of Statekit must preserve while remaining subordinate to the durable intent and invariants defined by the Project Core.

## Machine

Machine is an immutable value-like machine definition implemented as a shared handle. It represents a definition of the allowed state transitions.
### Construction
- Contains at least one logical transition.
- Contains only validated state names and transitions.
- Cycles between distinct states are permitted.
- Once constructed, its definition cannot change.
### Queries
- Determines whether an edge exists between two states.
- Exposes states, sources, transitions, and outgoing targets.
- Querying the transitions reachable from a state with no outgoing transitions produces an empty result. This includes states that appear only as transition targets and names that do not occur in the machine.
### Identity and Equality
- State names are matched exactly.
- Query input is not normalized or trimmed.
- Machine equality is based on the logical machine definition, independent of shared allocation identity.

## Machine Instance

Represents mutable traversal state over an immutable Machine definition.
### Construction
- The initial state of the instance must already exist in the machine definition.
### Current State
- The current state is always a state in the machine definition.
### Traversal
- The instance can transition from its current state only to a target allowed by the machine definition.
- A failed transition attempt has no observable effect on the instance.
### Terminality
- An instance is terminal when its current state has no outgoing transitions.
- A terminal instance cannot successfully transition.
### Independence
- Two instances referencing the same machine definition do not change each other's traversal state.
### Equality 
- Two instances are equal when their machine definitions are equal and their current states are equal.

## Machine Builder

Validates and builds the state-machine.
### Transition Addition
- Adds transitions between source and target states to the machine definition.
- Adding a duplicate transition does not change the machine definition.
### Validation Timing
- Validates requirements knowable about a proposed transition when the transition is added.
- Validates requirements knowable about the completed definition on build.
### Machine Construction
- A successful build produces a `Machine`.

## Transition

Represents a directed transition from a source state to a target state.
### Construction 
- A transition is defined by its source and target states.
- The source and target states must be different.
- Once constructed, its source and target cannot change.
### Equality 
- Two transitions with the same source and target represent the same logical transition.

## State Name

Represents the symbolic identity of a state.
### Construction
- A state name must not begin or end with Unicode whitespace.
- A state name must contain at least one non-whitespace character.
- State names may contain UTF-8 text.
- State names are not normalized or trimmed during construction.
### Identity
- Two states with the same state name represent the same logical state.
- State names are case-sensitive.

## Component Relationships

```
[Machine] -- Contains --> [Transition] -- Has Source --> [State Name]
                                       -- Has Target --> [State Name]
          -- Creates --> [Machine Instance]
```

```
MachineBuilder
      │
      │ builds
      ▼
   Machine
      │
      │ defines allowed
      ▼
 Transition
   │       │
 source   target
   │       │
   ▼       ▼
 State   State

Machine
   │
   │ creates / defines traversal for
   ▼
MachineInstance
```

## Compatibility

Public API changes follow semantic versioning.
Internal storage is not part of the Statekit domain contract.

Strengthening an invariant that causes previously valid input to be rejected is considered a behavioral compatibility change and must be intentional.

## Evidence

See [Project Core](./project-core.md).

Related testing documentation:

- [Unit Test Outline](./unit-test-outline.md)
- [Property Test Outline](./property-test-outline.md)