// Source: README.md
// Section: Inspecting a Machine
// Compatibility: current
// Introduced: 0.4
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .build()?;
    
    // BEGIN DOCUMENTATION EXAMPLE
    
    for state in machine.states() {
        println!("{state}");
    }

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
