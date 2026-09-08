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
✔ all transition values == all model values

CROSS-API CONSISTENCY
✔ transitions ↔ can_transition
✔ sources ⊆ states
✔ transition targets ↔ targets_from
✔ targets_from results ↔ can_transition

SEMANTIC DISTRIBUTION
✔ known-existing transition probe
✔ known-missing transition probe
✔ known-existing source
✔ known-missing source 

## Testing Relationships

### Structural Layers as a Whole

```
   ┌───────────────────────┐
     targeted edge cases                               │
       unit/integration                                │
   └───────────┬───────────┘
               │
┌──────────────▼──────────────┐
│ property tests              │
│ construction + invariants   │  └──────────────┬──────────────┘
                │
┌─────────────────▼─────────────────┐  │  independent model equivalence    │
│  edges / states / sources /       │
|   targets / membership            │
└───────────────────────────────────┘
              
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

> These are the invariants that Statekit must adhere to.

### Arbitrary Input 

> If Statekit accepts arbitrary input and constructs a machine, the resulting transition must satisfy Statekit's invariants.

## Summary 

All the tests in Statekit work as a complimentary test harness. Unit and Integration tests validate known examples while Property tests validate invariants from the specification. Model property tests validate from an external data set and Arbitrary property tests validates with a wide distribution of data.