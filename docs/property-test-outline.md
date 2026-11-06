---
project: statekit
category: traceability
version: 0.5
status: active
review-status: needs-review
---
# Property Test Outline 

## Purpose

The purpose of this document is to outline what property tests have been defined and how they work together as a whole within the complete test harness.

## Property Test Goal

> Every important invariant and public graph operation has strong behavioral coverage, and the graph projections are checked against an independent model.

## CORE INVARIANT COVERAGE

**INV-004 Logical Transition Uniqueness**
  → duplicate_transitions_collapse
  → model_based_valid_transitions_yields_equivalent_transition_count
  → model_and_machine_agree_on_possibly_empty_definitions

**INV-005 Case-Sensitive State Identity**
  → case_distinct_names_are_different
  → case_distinct_names_can_coexist_together

**INV-006 Runtime Configuration**
  → added_transition_is_allowed
  → model_and_machine_agree_on_possibly_empty_definitions

**INV-007 Earliest Valid Error Detection**
  → whitespace_only_source_is_rejected
  → whitespace_only_target_is_rejected
  → leading_whitespace_source_is_rejected
  → trailing_whitespace_source_is_rejected

**INV-009 Shared Immutable Machine**
  → instances_are_independent

**INV-010 Valid Traversal**
  → can_transition_to_agrees_with_the_edge_model
  → successful_instant_transitions_updates_current_state_to_target
  → model_and_instance_agree_with_transitions
  → model_and_instance_agree_on_attempts

**INV-011 Safe Traversal Failure**
  → failed_instance_transitions_have_no_observable_effect
  → model_and_instance_agree_with_transitions
  → model_and_instance_agree_on_attempts

**INV-012 Independent Instances**
  → instances_are_independent

**INV-013 Machine Equality**
  → duplicate_machines_are_equal
  → different_machines_are_not_equal
  → current properties use the same transition input for the equal case and add a distinct transition for the unequal case; they do not test equality across different insertion orders

**INV-014 Instance Equality**
  → instances_are_equal_when_machine_definition_and_states_are_equal
  → tests the equal case; no property tests unequal instances

## Outline

VALIDATION
✔ known-valid transitions accepted
✔ arbitrary accepted values preserve invariants
✔ whitespace-only rejected
✔ leading whitespace rejected
✔ trailing whitespace rejected
✔ source/target validation symmetry
✔ offending ambiguous value preserved
✔ instance construction preserves its initial state
✔ successful instance transitions update current state exactly to the target
✔ failed instance transitions have no observable effect
✔ shared-machine instance independence
✔ logically equal machine definitions are equal
✔ instances are equal when state and machine definitions are equal
✔ case distinct names are treated as distinct identities 

EDGE SEMANTICS
✔ exposed transition is queryable
✔ duplicate edges collapse
✔ positive membership
✔ negative membership

REFERENCE MODEL
✔ transition count == edge-set cardinality
✔ states == model vertices
✔ sources == model sources
✔ targets_from == model outgoing targets
✔ can_transition == model membership
✔ contains_state(s) == model vertex membership
✔ transitions() == model edge set
✔ validate_transition(a, b).is_ok() == model.contains((a, b))
✔ a.can_transition_to(b) == model.contains(a, b)
✔ instance is_terminal() == model terminal vertex
✔ instance transition_to()   == model.contains + mutable current state
✔ instance can_transition_to() + transition_to() + targets + terminality == model.contains + mutable current state + targets

CROSS-API CONSISTENCY
✔ transitions ↔ can_transition
✔ sources ⊆ states
✔ transition targets ↔ targets_from
✔ targets_from results ↔ can_transition

SEMANTIC DISTRIBUTION
✔ known-existing transition
✔ known-missing transition
✔ known-existing source
✔ source with no outgoing transition
✔ known-existing state
✔ known-missing state

RETIRED-INVARIANT BEHAVIOR
✔ self transitions are accepted and traversable
✔ a self-loop is not terminal
✔ an empty machine can be built, but cannot create an instance at an unknown state

Random generation provides variation, but generators should deliberately produce important semantic categories such as existing and missing transitions rather than relying on chance.

## Testing Relationships

### Structural Layers as a Whole

```
┌─────────────────────────────┐
│ Unit / Integration Tests    │
│ targeted known scenarios    │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│ Property Tests              │
│ invariants + input classes  │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│ Reference-Model Properties  │
│ graph semantic equivalence  │
└─────────────────────────────┘
```

### Structural Significance 

Example tests
    → "this particular scenario works"

Targeted property tests
    → "this entire semantic category works"

Arbitrary-input properties
    → "accepted values can never violate invariants"

Model-based properties
    → "the whole public graph API agrees with
       an independent specification"
       
## Property Test Input Types

```
valid-input property
────────────────────
Known valid input
      ↓
MUST be accepted


arbitrary-input property
────────────────────────
Any input
      ↓
IF accepted
      ↓
MUST satisfy invariants
```

Together:
```
    INPUT SPACE
┌───────────────────┐
│                   │
│   valid inputs    │──── must accept
│                   │
├───────────────────┤
│                   │
│ arbitrary inputs  │──── if accepted,
│                   │    invariants hold
└───────────────────┘
```

### Valid Input

Known-valid generated inputs exercise the acceptance side of the contract:

Statekit must accept them, and the resulting machine must exhibit the expected behavior.

### Arbitrary Input 

If Statekit accepts arbitrary input and constructs a machine, the resulting transition must satisfy Statekit's invariants.

## Summary 

All the tests in Statekit work as a complementary test harness. Unit and Integration tests validate known examples while Property tests validate invariants from the specification. Model-based property tests compare Statekit against an independent reference model, while arbitrary-input properties exercise a broad input space and verify that accepted values preserve the documented invariants.
