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

## Machine Builder

Validates and builds the state-machine.

## State Name

Validates and holds the state. 

## What it is Not

Statekit is not:
- a process engine
- a policy engine
- pathfinding code
- workflow engine

It can be used as a building block for these kinds of systems, but it intentionally does not implement them.





