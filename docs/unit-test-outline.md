---
project: statekit
category: traceability
version: 0.5
status: active
review-status: needs-review
---
# Unit Test Outline

## Purpose

This document maps the active Project Core invariants to focused unit tests, integration tests, and compile-time API-contract tests. It records direct, example-based evidence and identifies where the current suite does not directly prove an invariant.

Property tests are intentionally tracked separately in [`property-test-outline.md`](property-test-outline.md). A property test may strengthen an invariant across generated inputs, but it does not replace a missing focused test in this outline.

## Evidence Labels

- **Direct** — a focused test explicitly asserts the invariant's observable behavior.
- **Partial** — focused tests exercise part of the invariant, but an important claim remains implicit or untested.
- **Missing direct evidence** — no focused test directly asserts the invariant.

Test paths and names below describe the current suite. Internal unit tests live under `src/`; external-consumer integration tests and compile-time API tests live under `tests/`.

## Coverage Summary

| Invariant                              | Focused-test evidence                                                                        | Status |
| -------------------------------------- | -------------------------------------------------------------------------------------------- | ------ |
| INV-001 Machine Immutability           | External field access and mutable transition access fail to compile                          | Direct |
| INV-004 Logical Transition Uniqueness  | Duplicate transitions are stored once                                                        | Direct |
| INV-005 Case-Sensitive State Identity  | Case-distinct names remain distinct                                                          | Direct |
| INV-006 Runtime Configuration          | Runtime-owned transitions retained after source data is dropped                              | Direct |
| INV-007 Earliest Valid Error Detection | Invalid names fail during addition; unknown initial state fails during instance construction | Direct |
| INV-008 Standard Error Integration     | Error variants, Display messages, and std::error::Error implementation verified              | Direct |
| INV-009 Shared Immutable Machine       | Instances share the definition, including across threads                                     | Direct |
| INV-010 Valid Traversal                | Allowed traversal succeeds and disallowed traversal fails                                    | Direct |
| INV-011 Safe Traversal Failure         | Failed traversal preserves current state                                                     | Direct |
| INV-012 Independent Instances          | One instance's traversal does not alter another                                              | Direct |
| INV-013 Machine Equality               | Equality follows logical definition, not transition order or allocation                      | Direct |
| INV-014 Instance Equality              | Equality follows machine definition and current state                                        | Direct |

## INV-001 — Machine Immutability

**Status:** Direct

Focused evidence:

- `src/instance.rs::machine_returns_handle_to_same_machine_inner`
- `tests/transition_tests.rs::independent_multi_threaded_instances`
- `tests/ui/pass/machine_construction.rs::main`
- `tests/ui/fail/machine_internal_access.rs::main`
- `tests/ui/fail/machine_transition_mutation.rs::main`
- `tests/compile_tests.rs::public_api_contracts`

Evidence: instances can retain and share a built machine definition, and the public `Machine` API exposes inspection and instance construction without an obvious mutation operation.

Compile-time tests verify that external consumers cannot access `Machine::inner` and that `Machine::transitions()` returns shared references that cannot be assigned to mutable transition references.

`machine_construction.rs` verifies supported public construction and inspection rather than immutability itself.

## INV-004 — Logical Transition Uniqueness

**Status:** Direct

Focused evidence:

- `src/builder.rs::try_allow_duplicate_transition_is_ignored`
- `src/transitions.rs::duplicate_transition_is_stored_once`
- `src/inner.rs::targets_from_duplicate_transition_is_stored_once`

Evidence: adding the same source-target pair repeatedly leaves one logical transition and one outgoing target.

## INV-005 — Case-Sensitive State Identity

**Status:** Direct

Focused evidence:

- `src/builder.rs::try_allow_case_sensitive_from_has_three_states`
- `src/builder.rs::try_allow_case_sensitive_from_has_two_transitions`
- `src/builder.rs::try_allow_case_sensitive_to_has_three_states`
- `src/builder.rs::try_allow_case_sensitive_to_has_two_transitions`
- `src/state_name.rs::different_state_names_are_not_equal`

Evidence: names that differ only by case remain distinct states and produce distinct transitions.

## INV-006 — Runtime Configuration

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::validates_configured_workflow_transitions`
- `tests/transition_tests.rs::exposes_machine_structure_for_inspection`
- `tests/transition_tests.rs::machine_can_be_built_from_runtime_owned_transitions`

Evidence: a machine can be built from runtime-owned `String` pairs after the input configuration is dropped, and its configured transitions can be validated and inspected.

## INV-007 — Earliest Valid Error Detection

**Status:** Direct

Focused evidence:

- `src/builder.rs::try_allow::try_allow_error_for_empty_source_state`
- `src/builder.rs::try_allow::try_allow_error_for_empty_target_state`
- `src/builder.rs::try_allow::try_allow_errors_for_ambiguous_source_state`
- `src/builder.rs::try_allow::try_allow_errors_for_ambiguous_target_state`
- `src/instance.rs::construction::initial_unknown_state_fails_with_unknown_initial_state_error`
- `tests/transition_tests.rs::rejects_ambiguous_state_names`
- `tests/transition_tests.rs::rejects_whitespace_only_state_names`

Evidence: invalid transition endpoints fail during `try_allow`; an unknown initial state fails when instance construction first makes it invalid.

## INV-008 — Standard Error Integration

**Status:** Direct

Focused evidence:

- `src/error.rs::empty_state_display_message`
- `src/error.rs::invalid_transition_display_message_includes_endpoints`
- `src/error.rs::ambiguous_state_name_display_message`
- `src/error.rs::invalid_transition_display_escapes_whitespace`
- `src/error.rs::unknown_initial_state_display_message_includes_state`
- focused `matches!` assertions throughout `src/builder.rs`,
  `src/instance.rs`, and `tests/transition_tests.rs`
- `tests/error_tests.rs::verify_std_error_compliance`

Evidence: callers can match concrete error variants and obtain useful display messages. A trait-bound assertion verifies that `StateError` implements `std::error::Error`.

## INV-009 — Shared Immutable Machine

**Status:** Direct

Focused evidence:

- `src/instance.rs::machine_returns_handle_to_same_machine_inner`
- `src/instance.rs::instance_functions_with_no_handle`
- `tests/transition_tests.rs::independent_multi_threaded_instances`

Evidence: an instance retains the shared machine definition, and multiple instances can use it independently, including on separate threads.

## INV-010 — Valid Traversal

**Status:** Direct

Focused evidence:

- `src/instance.rs::transition_validation_uses_current_state_not_initial`
- `src/instance.rs::can_transition_to::can_transition_to_returns_true_on_valid_edge`
- `src/instance.rs::can_transition_to::can_transition_to_returns_false_on_unknown_edge`
- `src/instance.rs::can_transition_to::can_transition_to_returns_false_when_edge_exists_from_disallowed_state`
- `src/instance.rs::transition_to::transition_to_one_transition`
- `src/instance.rs::transition_to::transition_to_failure_when_unknown`
- `src/instance.rs::transition_to::transition_to_failure_when_not_reachable`
- `src/inner.rs::validate_transition::validate_transition_accepts_configured_transition`
- `src/inner.rs::validate_transition::validate_transition_not_exists_returns_invalid_error`
- `tests/transition_tests.rs::instance_walks_the_machine`
- `tests/transition_tests.rs::rejects_transition_not_defined_by_the_workflow`

Evidence: configured edges permit traversal, while missing or unreachable edges return an error based on the instance's current state.

## INV-011 — Safe Traversal Failure

**Status:** Direct

Focused evidence:

- `src/instance.rs::transition_to_failure_does_not_change_the_state`

Evidence: after a rejected transition, the instance reports the same current state it held before the attempt.

## INV-012 — Independent Instances

**Status:** Direct

Focused evidence:

- `src/instance.rs::two_instances_are_independent`
- `tests/transition_tests.rs::independent_multi_threaded_instances`

Evidence: traversing one instance does not change another instance created from the same machine, including when they move on separate threads.

## INV-013 — Machine Equality

**Status:** Direct

Focused evidence:

- `src/machine.rs::equality::similar_machines_are_equal`
- `src/machine.rs::equality::different_machines_are_not_equal`
- `src/inner.rs::partial_eq::similar_machines_are_equal`
- `src/inner.rs::partial_eq::similar_machines_in_different_order_are_equal`
- `src/inner.rs::partial_eq::different_machines_are_not_equal`

Evidence: equivalent definitions compare equal regardless of transition order; different definitions compare unequal.

## INV-014 — Instance Equality

**Status:** Direct

Focused evidence:

- `src/instance.rs::instances_are_equal_when_machine_and_current_state_are_equal`
- `src/instance.rs::instances_are_not_equal_when_machine_and_current_state_are_different`
- `src/instance.rs::instances_are_not_equal_when_machine_definitions_differ`

Evidence: instances compare equal when both their machine definitions and current states are equal. A different current state or machine definition makes them unequal.

## Missing Direct Evidence Backlog

None.
