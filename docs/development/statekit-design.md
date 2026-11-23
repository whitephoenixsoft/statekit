# Statekit Detailed Specification

## Purpose

To describe internal domain responsibilities, invariants, relationships, and representations.

Context: v0.4 to v0.5

## Changes
MachineBuilder
- Transitions
- try allows the str as API
- try from str to StateName 
- adds state directly to Transitions
- resolves id from Transitions 
- adds source and target IDs directly to Transitions 
- creates MachineInner with Transitions 
- creates Machine with MachineInner 
- manual PartialEq implementation 

Machine
- Arc of MachineInner
- creates MachineInstance with Machine and StateName 

MachineInstance
- Arc of MachineInner 
- state as StateName

MachineInner
- Transitions 
- translates StateName Iterators to &str
- translates &str queries into StateName

Transitions
- Vec of StateName for states
- HashSet of (usize, usize) for edges
- yields Transition on demand 
    - (usize, usize) to source, target
- adds StateName 
- adds transitions usize id
- resolved state id usize (maybe also get_or_insert_state )
- Iterators return &StateName

Transition
- source of StateName 
- target of StateName 
- continues to own these values to simplify interface 

StateName 
- String
- clone for use in Transition

---
## Machine
Shared Public immutable machine definition

### Representation
ARC of Machine Inner.

---
## Machine Builder
Validate and build the machine definition.

### Responsibilities
- Public interface for fallible state contruction
- Public interface for fallible transition contruction
- Guarantee that both state and transition construction is atomic 

### Representation
- State Registry for the states
- Transitions for the transitions

---
---
## Machine Inner
Immutable machine definition.

### Responsibilities
- return public projections of states as State Name
- return public projections of transitions State Name or Transition

### Representation
- State Registry
- Transitions

---
## State Registry

### Purpose
Maintain the identities of states belonging to a machine definition.

### Responsibilities
- Register valid states independently of transition creation.
- Resolve state names to internal identifiers
- Resolve internal identifiers to state names.
- Prevent duplicate logical state registration.

### Invariants
- Every registered state has exactly one internal identity.
- A state name resolves consistently throughout construction.
- Registering an existing state does not create another logical state.
- Internal identities remain valid for the lifetime of the constructed machine.

### Relationships
- `MachineBuilder` coordinates registration.
- `Transitions` references registered identities.
- `MachineInner` provides access to the validated state set.

### Representation
Vector of State Name.

### Traceability
Project Core: INV-005, INV-015, INV-017.
Foundation Specification: State Name, Machine Construction, Machine Queries

---
## Transitions
Maintain a collection of the relationships between state identities in the machine definition.

### Responsibilities
- inserts a transition to the collection
- prevents duplicate logical transitions
- returns projections of the transitions

### Representation
HashSet of a Tuple of (usize, usize).

---
## State Name
The semantic identity of the state.

### Reponsibilities
- maintain invariants for the the state name identity

### Representation
String

---
