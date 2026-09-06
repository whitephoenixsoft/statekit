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

## Testing Layer Relationship

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