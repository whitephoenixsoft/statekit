# Unit Test Outline

## Purpose

Documents how Statekit's targeted tests provide explicit evidence for Project Core invariants and specification contracts.

## INV-001 — Machine Immutability

Covered by:
- ...
- ...

Evidence:
A successfully built Machine exposes no mutation path for changing its transition definition.

## INV-002 — Self Transitions Are Rejected

Covered by:
- rejects_self_transition
- ...

Evidence:
Known self-transition inputs produce the expected
StateError.

Property coverage:
See property-test-outline.md.