# Statekit Implementation Design

MachineBuilder
- creates MachineInner

Machine
- Arc of MachineInner

MachineInner
- Vec of StateName
- Transitions 

Transitions
- HashSet of Transition 

Transition
- source of StateName 
- target of StateName 