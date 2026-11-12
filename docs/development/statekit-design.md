# Statekit Implementation Design

Context: v0.4 to v0.5

## Before

MachineBuilder
- Transitions
- try allows the str into Transition
- creates MachineInner with Transitions 
- creates Machine with MachineInner 

Machine
- Arc of MachineInner
- creates MachineInstance with Machine and StateName 

MachineInstance
- Arc of MachineInner 
- StateName 

MachineInner
- Transitions 

Transitions
- HashSet of Transition 
- adds transition

Transition
- source of StateName 
- target of StateName 
- try new transition 

StateName 
- String

## After

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
- state as usize

MachineInner
- Transitions 

Transitions
- Vec of StateName for states
- HashSet of (usize, usize) for edges
- yields Transition on demand 
    - (usize, usize) to source, target
- adds StateName 
- adds transitions usize id
- resolved state id usize (maybe also get_or_insert_state )

Transition
- source of StateName 
- target of StateName 
- continues to own these values to simplify interface 

StateName 
- String
- clone for use in Transition