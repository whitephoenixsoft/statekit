---
project: statekit
category: foundation
version: 0.4
status: active
review-status: needs-review
---
# Project Core 
## Purpose

This document records the durable intent, invariants, decisions, rationale, and relationships that form the semantic core of Statekit.

It is not intended to replace specifications, roadmaps, tests, benchmarks, source code, or Git history. Those artifacts describe and verify the project in greater detail. This document preserves the reasoning and constraints that give those artifacts context.

## Project Identity

Statekit is an immutable state transition validator
for applications that model workflow as data.

## Intents

**I-001: Runtime Use**
Statekit supports dynamic state machines whose states and transitions may be supplied at application runtime.

**I-002: Small and Fast**
Statekit is intended to remain small and computationally inexpensive.

**I-003: Ease of Use**
Statekit is intended to provide a simple and understandable API for its users.

**I-004: Engineering Reference**
Statekit is intended to serve as a reference project for disciplined software-engineering practices.

**I-005: Data-Driven Definition**
State machines are intended to be defined from data rather than requiring machine structure to be encoded directly into application logic.

## Boundaries / Durable Intent

**BI-001: Building Block**
Statekit remains a validation building block rather than becoming a process, workflow, policy, or pathfinding engine.

**BI-002: Hidden Internal Storage**
Internal storage/representation is not part of the public domain contract.

**BI-003: Concurrency**
Scheduling and synchronization policy belong to the host application.

## Core Invariants

**INV-001: Machine Immutability**
A built state machine is immutable.

**INV-002: No Self Transitions**
Self transitions are rejected.

**INV-003: Non-Empty Transition Set**
A valid machine contains at least one transition.

**INV-004: Logical Transition Uniqueness**
Duplicate transitions with the same source and target represent the same logical transition.

**INV-005: Case-Sensitive State Identity**
State names are case sensitive.

**INV-006: Runtime Configuration**
A state machine can be configured from data during application runtime.

**INV-007: Earliest Valid Error Detection**
Validation errors are reported at the earliest point at which they can be determined to be invalid.

**INV-008: Standard Error Integration**
State errors implement the standard Rust error interface and remain straightforward for callers to match and handle.

**INV-009: Shared Immutable Machine**
Machine instances can safely share an immutable machine definition.

**INV-010: Valid Traversal**
A machine instance can transition only along transitions allowed by its machine.

**INV-011: Safe Traversal Failure**
Failed instance transitions have no observable effect.

**INV-012: Independent Instances**
Instances sharing a machine maintain independent traversal state.

**INV-013: Machine Equality**
Machine equality reflects machine definition, not shared allocation identity.

**INV-014: Instance Equality**
Instances on equal machine definitions and in the same state are equal. This is because two instances can be in the same logical traversal position.

## Decisions

**D-001: Declarative Machine Definition**
> The primary state machine is defined declaratively rather than through mutation after construction.

**Supports:** I-003, I-005
**Preserves:** INV-001, INV-009
**Reason:** Declarative construction makes the resulting machine easier to validate, reason about, and safely share between instances.

**D-002: Builder and Machine as Primary APIs**
> The primary construction APIs are a builder and a machine.

**Supports:** I-002, I-003, I-005
**Preserves:** INV-001, INV-006
**Reason:** These provide the minimum separation needed to configure and validate a machine before exposing the resulting immutable definition.

**D-003: Unit Testing**
> Statekit uses focused unit tests to verify individual behaviors.

**Supports:** I-004
**Verifies:** Core behavior and local implementation contracts.
**Reason:** Unit tests provide direct verification that individual Statekit behaviors continue to meet their expected contracts.

**D-004: `thiserror` for State Errors**
> Statekit uses `thiserror` to support its state error types.

**Supports:** I-002, I-003
**Preserves:** INV-008
**Reason:** `thiserror` keeps error implementation simple while integrating with Rust's standard error conventions.

**D-005: External Transition API Testing**
> Statekit tests its transition API from the perspective of an external crate consumer.

**Supports**: I-003, I-004
**Verifies:** Public API usability and externally observable behavior.
**Reason:** Testing through the public API verifies that Statekit works as a consumer experiences it rather than only through internal implementation access.

**D-006: Fail During Machine Construction When Possible**
> Errors that can be determined while constructing a machine are reported during construction rather than deferred to traversal.

**Supports:** I-003
**Preserves:** INV-007
**Reason:** Reporting known-invalid configurations immediately makes failures easier to understand and avoids carrying unnecessary invalid-state handling into later runtime operations.

**D-007: Property Testing with `proptest`**
> Statekit uses `proptest` for property-based testing.

**Supports:** I-004
**Verifies:** Core invariants across generated machine structures and operation sequences.
**Reason:** Property testing provides broader invariant validation as Statekit's internal representation and infrastructure evolve.

**D-008: Performance Benchmarking with `criterion`**
> Statekit uses `criterion` for performance benchmarking.

**Supports:** I-002, I-004
**Evaluates:** Processing-performance tradeoffs.
**Reason:** Repeatable benchmarks provide evidence for evaluating performance consequences when internal infrastructure changes.

**D-009: Memory Benchmarking**
> Statekit explicitly measures memory behavior.

**Supports:** I-002, I-004
**Evaluates:** Memory and allocation tradeoffs.
**Reason:** Memory measurements provide evidence for evaluating representation and infrastructure changes rather than relying on assumptions about their cost.

**D-010: Mutable Instances over Shared Machines**
> Statekit represents traversal through mutable machine instances that reference a shared immutable machine.

**Supports:** I-001, I-003
**Preserves:** INV-001, INV-009, INV-010
**Reason:** Separating mutable traversal state from the immutable machine definition provides a natural runtime model while allowing the underlying machine to be safely shared.

**D-011: Performance Tradeoff**
> Construction may perform additional work to reduce repeated runtime cost when the tradeoff is supported by measured performance and bounded memory cost.

**Supports:** I-001, I-002, BI-002
**Reason:** Construction cost is not as important  as the overall increase of the runtime functions.

**D-012: Runtime Failures**
> Normal runtime state should be observable without provoking errors. Errors represent invalid operations, not ordinary machine conditions.

**Supports:** I-003
**Preserves:** INV-007, INV-008
**Reason:** Keeping all possible conditions from reporting errors keeps the complexity of the code supporting it down. It also allows errors to be treated as exceptions.
