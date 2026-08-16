# Statekit Specification

Version: 0.2

Status: Foundational — In Development

Scope: Statekit domain definitions and invariants

## Purpose

This document defines statekit and its components. 

## Core Principle

Statekit is an immutable state transition validator for applications that model workflow as data.

Its purpose is to become a foundation from which dynamic state transitions can be validated during runtime. 

The method to fulfill this is to provide an immutable state-machine definition that validates whether a transition is permitted.

## Core Components

The components in this specification describe Statekit's domain model. A component does not necessarily correspond one-to-one with a Rust type in every implementation version.

### Machine

Holds the state transitions.

- It is only valid if there is a transition.
- Verifies that a transition exists.
- Machine must support querying allowed transitions.
#### Invariants and Constraints

- Machine must be immutable.
- Machine will validate a transition based on source and target states.
- There must be at least one transition.

### Transition

Holds a unique state transitions.

- A transition is defined by its source and target states.
- A transition is unique.
	- The source and target of a transition must be different.
- Cycles between distinct states are permitted.

### Machine Builder

Validates and builds the state-machine.

- Defines transitions between states.
- Enforces transition and state validation before a `Machine` is built.
- Provides an API for constructing a machine definition incrementally.

### State Name

Validates and holds the state. 

- Creates and maintains a valid state.
	- Two states with the same state name represent the same logical state.
	- A state name must not begin or end with Unicode whitespace.
	- A state name must contain at least one non-whitespace character.
- Must support actions related to the state domain. 
- Must support UTF-8 strings.


## Architecture

[Machine] -- Contains --> [Transitions] -- Contains --> [States]

[Machine Builder] -- Allows --> [Transition] -- Builds Upon --> [Source State Name, Target State Name]
                  -- Builds --> [Machine] 

## Compatibility Principles

Public API changes follow semantic versioning.
Internal storage is not part of the Statekit domain contract.

Strengthening an invariant that causes previously valid input to be rejected is considered a behavioral compatibility change and must be intentional.

## What it is Not

Statekit is not:
- a process engine
- a policy engine
- pathfinding code
- workflow engine

It can be used as a building block for these kinds of systems, but it intentionally does not implement them.





