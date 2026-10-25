// Source: MIGRATION.md
// Section: `StateError::UnknownInitialState` for `MachineInstance` errors 
// Compatibility: current
// Introduced: 0.4
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    // BEGIN DOCUMENTATION EXAMPLE
    
    let machine = Machine::builder()
        .try_allow("draft", "approved")?
        .build()?;
        
    let result = machine.instance("unknown");

    if let Err(StateError::UnknownInitialState { state }) = result {
        println!("Failed to initialize with {}!", state);
    }

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
