# Property Test Outline 

## Purpose

The purpose of this document is to outline what property tests have been defined and how they work together as a whole within the complete test harness.

## Property Test Goal

> Every important invariant and public graph operation has strong behavioral coverage, and the graph projections are checked against an independent model.

## Outline

VALIDATION
✔ known-valid transitions accepted
✔ arbitrary accepted values preserve invariants
✔ whitespace-only rejected
✔ leading whitespace rejected
✔ trailing whitespace rejected
✔ source/target validation symmetry
✔ offending ambiguous value preserved

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
✔ contains_state(s) == model contains vertex s
✔ transitions() == model edge set
✔ validate_transition(a, b).is_ok() == model.contains((a, b))

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