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
- try allows the str into Transitions
- creates MachineInner with Transitions 
- creates Machine with MachineInner 

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
- try adds states
- try adds transitions 

Transition
- source of StateName 
- target of StateName 

StateName 
- String