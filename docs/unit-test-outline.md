---
project: statekit
category: traceability
version: 0.4
status: active
review-status: needs-review
---

# Unit Test Outline

## Purpose

This document maps the Project Core invariants to focused unit tests, integration tests, and compile-time API-contract tests. It records direct, example-based evidence and identifies where the current suite does not directly prove an invariant.

Property tests are intentionally tracked separately in [`property-test-outline.md`](property-test-outline.md). A property test may strengthen an invariant across generated inputs, but it does not replace a missing focused test in this outline.

## Evidence Labels

- **Direct** — a focused test explicitly asserts the invariant's observable behavior.
- **Partial** — focused tests exercise part of the invariant, but an important claim remains implicit or untested.
- **Missing direct evidence** — no focused test directly asserts the invariant.

Test paths and names below describe the current suite. Internal unit tests live under `src/`; external-consumer integration tests and compile-time API tests live under `tests/`.

## Coverage Summary

| Invariant                              | Focused-test evidence                                                           | Status |
| -------------------------------------- | ------------------------------------------------------------------------------- | ------ |
| INV-001 Machine Immutability           | Private field access and mutable transition access are rejected                 | Direct |
| INV-002 No Self Transitions            | Known self-transition inputs are rejected                                       | Direct |
| INV-003 Non-Empty Transition Set       | Empty builds are rejected                                                       | Direct |
| INV-004 Logical Transition Uniqueness  | Duplicate transitions collapse                                                  | Direct |
| INV-005 Case-Sensitive State Identity  | Case-distinct names remain distinct                                             | Direct |
| INV-006 Runtime Configuration          | Runtime-owned transitions retained after source data is dropped                 | Direct |
| INV-007 Earliest Valid Error Detection | Construction errors occur at `try_allow` or `build`                             | Direct |
| INV-008 Standard Error Integration     | Error variants, Display messages, and std::error::Error implementation verified | Direct |
| INV-009 Shared Immutable Machine       | Instances retain and share one machine definition                               | Direct |
| INV-010 Valid Traversal                | Allowed traversal succeeds and disallowed traversal fails                       | Direct |
| INV-011 Safe Traversal Failure         | Failed traversal preserves current state                                        | Direct |
| INV-012 Independent Instances          | One instance's traversal does not alter another                                 | Direct |
| INV-013 Machine Equality               | Equality follows definition rather than allocation                              | Direct |
| INV-014 Instance Equality              | Equality follows machine definition and current state                           | Direct |

## INV-001 — Machine Immutability

**Status:** Direct

Focused evidence:

- `src/instance.rs::machine_returns_handle_to_same_machine_inner`
- `tests/transition_tests.rs::independent_multi_threaded_instances`
- `tests/ui/pass/machine_construction.rs::main`
- `tests/ui/fail/machine_internal_access.rs::main`
- `tests/ui/fail/machine_transition_mutation.rs::main`

Evidence: instances can retain and share a built machine definition, and the public `Machine` API exposes inspection and instance construction without an obvious mutation operation.

Compile-time tests verify that external consumers cannot access `Machine::inner` and that `Machine::transitions()` returns shared references that cannot be assigned to mutable transition references.

`machine_construction.rs` verifies supported public construction and inspection rather than immutability itself.

Property-test coverage: none documented for INV-001 in `property-test-outline.md`.

## INV-002 — No Self Transitions

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::rejects_a_self_transition`
- `src/builder.rs::try_allow_error_transition_to_self`
- `src/transition.rs::self_transition_returns_error`

Evidence: a known transition whose source and target are equal is rejected as `StateError::SelfTransition`, including the offending state.

Property-test coverage: none documented for INV-002.

## INV-003 — Non-Empty Transition Set

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::rejects_an_empty_machine_definition`
- `src/builder.rs::build_empty_returns_no_transitions`

Evidence: building without a transition returns `StateError::NoTransitions`.

Property-test coverage: none documented for INV-003.

## INV-004 — Logical Transition Uniqueness

**Status:** Direct

Focused evidence:

- `src/builder.rs::try_allow_duplicate_transition_is_ignored`
- `src/transitions.rs::duplicate_transition_is_stored_once`
- `src/machine_inner.rs::targets_from_duplicate_transition_is_stored_once`

Evidence: adding the same source-target pair more than once leaves one logical transition and one outgoing target.

Property-test coverage: `duplicate_transitions_collapse` and reference-model transition cardinality are documented for INV-004.

## INV-005 — Case-Sensitive State Identity

**Status:** Direct

Focused evidence:

- `src/builder.rs::try_allow_case_sensitive_from_has_three_states`
- `src/builder.rs::try_allow_case_sensitive_from_has_two_transitions`
- `src/builder.rs::try_allow_case_sensitive_to_has_three_states`
- `src/builder.rs::try_allow_case_sensitive_to_has_two_transitions`
- `src/state_name.rs::different_state_names_are_not_equal`

Evidence: names that differ only by case remain distinct states and produce distinct transitions.

Property-test coverage:  none documented for INV-005.

## INV-006 — Runtime Configuration

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::validates_configured_workflow_transitions`
- `tests/transition_tests.rs::exposes_machine_structure_for_inspection`
- `tests/transition_tests.rs::machine_can_be_built_from_runtime_owned_transitions`

Evidence: an external consumer can construct and inspect a machine through the public builder API. An external consumer builds a machine from runtime-owned String pairs, discards the original configuration before calling build(), and verifies that the resulting machine preserves the configured transitions.

Property-test coverage: generated transition vectors demonstrate data-driven construction throughout `tests/property_tests.rs`, but INV-006 is not mapped in `property-test-outline.md`.

## INV-007 — Earliest Valid Error Detection

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::rejects_a_self_transition`
- `tests/transition_tests.rs::rejects_ambiguous_state_names`
- `tests/transition_tests.rs::rejects_whitespace_only_state_names`
- `tests/transition_tests.rs::rejects_an_empty_machine_definition`
- `src/instance.rs::initial_unknown_state_fails_with_unknown_initial_state_error`

Evidence: invalid transition endpoints and self-transitions fail when added by `try_allow`; an empty transition set fails at `build`; an unknown initial state fails when instance construction first makes it invalid.

Property-test coverage: validation properties cover whitespace rejection and preservation of offending values, but INV-007 is not explicitly mapped in `property-test-outline.md`.

## INV-008 — Standard Error Integration

**Status:** Direct

Focused evidence:

- `src/error.rs::empty_state_display_message`
- `src/error.rs::no_transitions_display_message`
- `src/error.rs::self_transition_display_message_includes_state`
- `src/error.rs::invalid_transition_display_message_includes_endpoints`
- focused `matches!` assertions throughout `src/builder.rs`,
  `src/instance.rs`, and `tests/transition_tests.rs`
- `tests/error_tests.rs::verify_std_error_compliance`

Evidence: callers can match concrete `StateError` variants and obtain stable,
useful display messages. A compile-time trait-bound assertion verifies that `StateError` implements `std::error::Error`.

Property-test coverage: generated tests match several error variants and payloads, but INV-008 is not mapped in `property-test-outline.md`.

## INV-009 — Shared Immutable Machine

**Status:** Direct

Focused evidence:

- `src/instance.rs::machine_returns_handle_to_same_machine_inner`
- `src/instance.rs::instance_functions_with_no_handle`
- `tests/transition_tests.rs::independent_multi_threaded_instances`

Evidence: an instance retains the shared machine allocation after the original handle is gone, and separate instances can use that definition on different threads without definition mutation.

Property-test coverage: `instances_are_independent` is documented for INV-009; the outline also cites multithreaded integration coverage, which is the focused
integration test above rather than a property test.

## INV-010 — Valid Traversal

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::instance_walks_the_machine`
- `src/instance.rs::transition_to_one_transition`
- `src/instance.rs::transition_to_failure_when_not_reachable`
- `src/instance.rs::transition_to_failure_when_unknown`
- `src/instance.rs::transition_validation_uses_current_state_not_initial`

Evidence: configured edges permit traversal, while missing or unreachable edges return an error based on the instance's current state.

Property-test coverage: `can_transition_to_agrees_with_the_edge_model` and the model/instance attempt properties are documented for INV-010.

## INV-011 — Safe Traversal Failure

**Status:** Direct

Focused evidence:

- `src/instance.rs::transition_to_failure_does_not_change_the_state`

Evidence: after a rejected transition, the instance reports the same current state it held before the attempt.

Property-test coverage: `failed_instance_transitions_have_no_observable_effect` and the model/instance attempt properties are documented for INV-011. 

## INV-012 — Independent Instances

**Status:** Direct

Focused evidence:

- `src/instance.rs::two_instances_are_independent`
- `tests/transition_tests.rs::independent_multi_threaded_instances`

Evidence: moving one instance does not change another instance created from the same machine, including when the instances move on separate threads.

Property-test coverage: `instances_are_independent` is documented for INV-012.

## INV-013 — Machine Equality

**Status:** Direct

Focused evidence:

- `src/machine.rs::similar_machines_are_equal`
- `src/machine.rs::different_machines_are_not_equal`

Evidence: independently allocated machines with equivalent definitions compare equal, while different transition definitions compare unequal.

Property-test coverage:  none documented for INV-013.

## INV-014 — Instance Equality

**Status:** Direct

Focused evidence:

- `src/instance.rs::instances_are_equal_when_machine_and_current_state_are_equal`
- `src/instance.rs::instances_are_not_equal_when_machine_and_current_state_are_different`
- `src/instance.rs::instances_are_not_equal_when_machine_definitions_differ`

Evidence: instances compare equal when both their machine definitions and current states are equal. A different current state or machine definition makes them unequal.

Property-test coverage: none documented for INV-014.

## Missing Direct Evidence Backlog

None.
