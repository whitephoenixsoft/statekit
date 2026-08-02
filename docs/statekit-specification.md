# Statekit Specification
Version: 0.2
Document Status: Foundational (In Development)
Affect: Statekit Definitions and Invariants

## Purpose

This document defines statekit and its components. 

## Core Principle

Statekit is an immutable state transition validator for applications that model workflow as data.

It's purpose is to become a foundation from which dynamic state transitions can be validated during runtime. 

The method to fulfill this is to provide an immutable state-machine definition that validates whether a transition is permitted.

## Core Components

### Machine

Holds the state transitions.

- It is only valid if there is a transition.
- Validates that a transition exists.
- Must support actions related to state transitions.

### Machine Builder

Validates and builds the state-machine.

- Converts states into transitions.
- Must support actions for dynamically creating transitions.

### State Name

Validates and holds the state. 

- Crates and maintains a valid state.
- Must support actions related to the state domain. 


## Architecture

[Machine] -- Contains --> [Transitions] -- Contains --> [States]

[Machine Builder] -- Allows --> [Transition] -- Builds Upon --> [Source State Name, Target State Name]
                  -- Builds --> [Machine] 

## What it is Not

Statekit is not:
- a process engine
- a policy engine
- pathfinding code
- workflow engine

It can be used as a building block for these kinds of systems, but it intentionally does not implement them.





