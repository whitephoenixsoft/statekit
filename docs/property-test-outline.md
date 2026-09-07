# Property Test Outline 

## Outline 

Construction / validation
├── valid state names accepted
├── empty / whitespace-only rejected
├── leading whitespace rejected
├── trailing whitespace rejected
├── self-transition rejected
└── non-empty machine required

Graph membership
├── added transition is queryable
├── missing transition is not queryable
└── model membership == can_transition

Graph projection APIs
├── transition_count == model edge count
├── transitions() == model edges
├── sources() == model sources
├── states() == model vertices
└── targets_from(source) == model outgoing targets

Cross-API consistency
├── every exposed transition passes can_transition
├── every exposed source is a state
├── every targets_from result is a valid transition
└── each source has at least one outgoing target

Edge semantics
├── directionality
├── duplicate transitions collapse
├── target-only states exist in states()
├── target-only states are not necessarily sources()
├── unknown source gives empty targets_from()
└── iteration order is not assumed

Arbitrary Input

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
   targets / membership             │
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
        │                   │     invariants hold
        └───────────────────┘
```

## Definitions 

### Arbitrary Input 

> If Statekit accepts arbitrary input and constructs a machine, the resulting transition must satisfy Statekit's invariants.