# Unit Test Outline

## Purpose

This document maps the Project Core invariants to focused unit and integration
tests. It records direct, example-based evidence and identifies where the
current suite does not directly prove an invariant.

Property tests are intentionally tracked separately in
[`property-test-outline.md`](property-test-outline.md). A property test may
strengthen an invariant across generated inputs, but it does not replace a
missing focused test in this outline.

## Evidence Labels

- **Direct** — a focused test explicitly asserts the invariant's observable
  behavior.
- **Partial** — focused tests exercise part of the invariant, but an important
  claim remains implicit or untested.
- **Missing direct evidence** — no focused test directly asserts the invariant.

Test paths and names below describe the current suite. Internal unit tests live
under `src/`; external-consumer integration tests live under `tests/`.

## Coverage Summary

| Invariant | Focused-test evidence | Status |
| --- | --- | --- |
| INV-001 Machine Immutability | No mutation API is exposed; sharing is exercised | Partial |
| INV-002 No Self Transitions | Known self-transition inputs are rejected | Direct |
| INV-003 Non-Empty Transition Set | Empty builds are rejected | Direct |
| INV-004 Logical Transition Uniqueness | Duplicate transitions collapse | Direct |
| INV-005 Case-Sensitive State Identity | Case-distinct names remain distinct | Direct |
| INV-006 Runtime Configuration | Construction is public, but focused tests use literals | Partial |
| INV-007 Earliest Valid Error Detection | Construction errors occur at `try_allow` or `build` | Direct |
| INV-008 Standard Error Integration | Variants and display text are tested | Partial |
| INV-009 Shared Immutable Machine | Instances retain and share one machine definition | Direct |
| INV-010 Valid Traversal | Allowed traversal succeeds and disallowed traversal fails | Direct |
| INV-011 Safe Traversal Failure | Failed traversal preserves current state | Direct |
| INV-012 Independent Instances | One instance's traversal does not alter another | Direct |
| INV-013 Machine Equality | Equality follows definition rather than allocation | Direct |
| INV-014 Instance Equality | Equality follows machine definition and current state | Direct |

## INV-001 — Machine Immutability

**Status:** Partial

Focused evidence:

- `src/instance.rs::machine_returns_handle_to_same_machine_inner`
- `tests/transition_tests.rs::independent_multi_threaded_instances`

Evidence: instances can retain and share a built machine definition, and the
public `Machine` API exposes inspection and instance construction without an
obvious mutation operation.

Missing direct evidence: ordinary runtime unit tests cannot prove that a public
mutation path is absent. Add a compile-fail/API-surface test (for example with
`trybuild`) if this invariant should have executable direct evidence.

Property-test coverage: none documented for INV-001 in
`property-test-outline.md`.

## INV-002 — No Self Transitions

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::rejects_a_self_transition`
- `src/builder.rs::try_allow_error_transition_to_self`
- `src/transition.rs::self_transition_returns_error`

Evidence: a known transition whose source and target are equal is rejected as
`StateError::SelfTransition`, including the offending state.

Property-test coverage:
`self_transition_is_always_rejected` is attributed to INV-002 in
`property-test-outline.md`, but no test with that name currently exists in
`tests/property_tests.rs`. This is a property-outline traceability gap.

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

Evidence: adding the same source-target pair more than once leaves one logical
transition and one outgoing target.

Property-test coverage: `duplicate_transitions_collapse` and reference-model
transition cardinality are documented for INV-004.

## INV-005 — Case-Sensitive State Identity

**Status:** Direct

Focused evidence:

- `src/builder.rs::try_allow_case_sensitive_from_has_three_states`
- `src/builder.rs::try_allow_case_sensitive_from_has_two_transitions`
- `src/builder.rs::try_allow_case_sensitive_to_has_three_states`
- `src/builder.rs::try_allow_case_sensitive_to_has_two_transitions`
- `src/state_name.rs::different_state_names_are_not_equal`

Evidence: names that differ only by case remain distinct states and produce
distinct transitions.

Property-test coverage: `case_distinct_states_remain_distinct` is attributed to
INV-005 in `property-test-outline.md`, but no test with that name currently
exists in `tests/property_tests.rs`. This is a property-outline traceability
gap.

## INV-006 — Runtime Configuration

**Status:** Partial

Focused evidence:

- `tests/transition_tests.rs::validates_configured_workflow_transitions`
- `tests/transition_tests.rs::exposes_machine_structure_for_inspection`

Evidence: an external consumer can construct and inspect a machine through the
public builder API.

Missing direct evidence: the focused tests construct definitions from string
literals. Add an external-consumer test that builds a machine by iterating over
a runtime-owned collection of transition data and then validates the resulting
definition.

Property-test coverage: generated transition vectors demonstrate data-driven
construction throughout `tests/property_tests.rs`, but INV-006 is not mapped in
`property-test-outline.md`.

## INV-007 — Earliest Valid Error Detection

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::rejects_a_self_transition`
- `tests/transition_tests.rs::rejects_ambiguous_state_names`
- `tests/transition_tests.rs::rejects_whitespace_only_state_names`
- `tests/transition_tests.rs::rejects_an_empty_machine_definition`
- `src/instance.rs::initial_unknown_state_fails_with_unknown_initial_state_error`

Evidence: invalid transition endpoints and self-transitions fail when added by
`try_allow`; an empty transition set fails at `build`; an unknown initial state
fails when instance construction first makes it invalid.

Property-test coverage: validation properties cover whitespace rejection and
preservation of offending values, but INV-007 is not explicitly mapped in
`property-test-outline.md`.

## INV-008 — Standard Error Integration

**Status:** Partial

Focused evidence:

- `src/error.rs::empty_state_display_message`
- `src/error.rs::no_transitions_display_message`
- `src/error.rs::self_transition_display_message_includes_state`
- `src/error.rs::invalid_transition_display_message_includes_endpoints`
- focused `matches!` assertions throughout `src/builder.rs`,
  `src/instance.rs`, and `tests/transition_tests.rs`

Evidence: callers can match concrete `StateError` variants and obtain stable,
useful display messages.

Missing direct evidence: no focused compile-time assertion currently requires
`StateError: std::error::Error`. Add a small generic trait-bound test to prove
standard Rust error integration independently of the `thiserror` derive.

Property-test coverage: generated tests match several error variants and
payloads, but INV-008 is not mapped in `property-test-outline.md`.

## INV-009 — Shared Immutable Machine

**Status:** Direct

Focused evidence:

- `src/instance.rs::machine_returns_handle_to_same_machine_inner`
- `src/instance.rs::instance_functions_with_no_handle`
- `tests/transition_tests.rs::independent_multi_threaded_instances`

Evidence: an instance retains the shared machine allocation after the original
handle is gone, and separate instances can use that definition on different
threads without definition mutation.

Property-test coverage: `instances_are_independent` is documented for INV-009;
the outline also cites multithreaded integration coverage, which is the focused
integration test above rather than a property test.

## INV-010 — Valid Traversal

**Status:** Direct

Focused evidence:

- `tests/transition_tests.rs::instance_walks_the_machine`
- `src/instance.rs::transition_to_one_transition`
- `src/instance.rs::transition_to_failure_when_not_reachable`
- `src/instance.rs::transition_to_failure_when_unknown`
- `src/instance.rs::transition_validation_uses_current_state_not_initial`

Evidence: configured edges permit traversal, while missing or unreachable
edges return an error based on the instance's current state.

Property-test coverage: `can_transition_to_agrees_with_the_edge_model` and the
model/instance attempt properties are documented for INV-010.

## INV-011 — Safe Traversal Failure

**Status:** Direct

Focused evidence:

- `src/instance.rs::transition_to_failure_does_not_change_the_state`

Evidence: after a rejected transition, the instance reports the same current
state it held before the attempt.

Property-test coverage: `failed_instance_transitions_have_no_observable_effect`
and the model/instance attempt properties are documented for INV-011. The
current implementation name in `tests/property_tests.rs` is
`failed_instant_transitions_have_no_observable_effect`, so the two outlines
should be reconciled.

## INV-012 — Independent Instances

**Status:** Direct

Focused evidence:

- `src/instance.rs::two_instances_are_independent`
- `tests/transition_tests.rs::independent_multi_threaded_instances`

Evidence: moving one instance does not change another instance created from the
same machine, including when the instances move on separate threads.

Property-test coverage: `instances_are_independent` is documented for INV-012.

## INV-013 — Machine Equality

**Status:** Direct

Focused evidence:

- `src/machine.rs::similar_machines_are_equal`
- `src/machine.rs::different_machines_are_not_equal`

Evidence: independently allocated machines with equivalent definitions compare
equal, while different transition definitions compare unequal.

Property-test coverage: equivalent and different definition properties are
attributed to INV-013 in `property-test-outline.md`, but neither named test
currently exists in `tests/property_tests.rs`. This is a property-outline
traceability gap.

## INV-014 — Instance Equality

**Status:** Direct

Focused evidence:

- `src/instance.rs::instances_are_equal_when_machine_and_current_state_are_equal`
- `src/instance.rs::instances_are_not_equal_when_machine_and_current_state_are_different`
- `src/instance.rs::instances_are_not_equal_when_machine_definitions_differ`

Evidence: instances compare equal when both their machine definitions and
current states are equal. A different current state or machine definition makes
them unequal.

Property-test coverage: none documented for INV-014.

## Missing Direct Evidence Backlog

1. **INV-001:** add a compile-fail/API-surface test proving that a built
   `Machine` cannot be mutated through the public API.
2. **INV-006:** add an external-consumer test that constructs a machine from a
   runtime-owned transition collection rather than literals.
3. **INV-008:** add a compile-time trait-bound assertion for
   `StateError: std::error::Error`.

Separately, reconcile the stale or absent property-test names called out under INV-002, INV-005, INV-011, and INV-013. Those discrepancies do not reduce the focused unit-test evidence recorded here, but they currently weaken end-to-end traceability between the two outlines and the executable suite.
