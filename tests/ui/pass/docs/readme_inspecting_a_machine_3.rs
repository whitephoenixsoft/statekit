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
    
    for transition in machine.transitions() {
        println!(
            "{} -> {}",
            transition.source(),
            transition.target()
        );
    }

    // END DOCUMENTATION EXAMPLE
    
    Ok(())
}
