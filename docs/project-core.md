# Project Core

## Purpose

This document is to record high-level project intent, invariants, decisions, and relationships.

## Intents

I-001: Statekit is a state machine validation library
I-002: Statekit is meant to be used in the runtime layer 
I-003: Statekit is meant to be small and quick
I-004: Statekit is meant to be easy to use
I-005: Statekit is meant to be a reference project for proper software engineering principles

## Invariants

INV-001: The state machine must be immutable
INV-002: Self transitions are rejected
INV-003: A machine must contain at least one transition
INV-004: Duplicate transitions between the same source and target are considered the same logical transition
INV-005: State names are case sensitive
INV-007: Validation errors must be as early as possible
INV-008: State Errors must be of standard Error and simple to match
INV-009: The state machine instance must safely share an immutable machine
INV-010: The state machine insntance must always tranverse allowed transitions

## Decisions

D-001: The main state machine is to be called declaratively 
	Supports: INV-009
	Reason: It makes it easier to share between instances
D-002: The main apis are a builder and a machine
	Supports: I-003, I-004
	Reason: This is what is minimally needed to implement a validation tool
D-003: Unit tests to quality test functions
	Supports: I-005
	Reason: Needed to ensure statekit meets expected behavior
D-004: Use of the ThisError crate for supporting StateError
	Supports: INV-008
	Reason: Keeps the code simple when it comes to handling errors
D-005: Transition API testing for crate validation
	Supports: I-005, I-004
	Reason: Validates that the external API will work outside of statekit
D-006: If possible fail immediately when building the machine
	Supports: INV-007
	Reason: It's less confusing and makes the code less bloated.
D-007: Use of proptest for property testing.
	Supports: I-005
	Reason: Property testing is needed to ensure statekit invariants now that infrasture changes are planned
D-008: Use of Criterion for benchmarking
	Supports: I-003, I-005
	Reason: Need a way to prove processing tradeoffs when infrastructure changes are made
D-009: Use of memory benchmarking
	Supports: I-003, I-005
	Reason: Neead a way to prove the memory tradeoffs when infrastructure changes are made
D-010: Use of instances
	Support: I-001, I-002, I-004
	Reason: Seems more natural when traversing the state machine through a mutable instance

