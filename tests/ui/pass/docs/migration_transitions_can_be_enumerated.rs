// Source: MIGRATION.md
// Section: Transitions can be enumerated directly
// Compatibility: current
// Introduced: 0.3
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("draft", "approved")?
        .build()?;

    // BEGIN DOCUMENTATION EXAMPLE
    
    for transition in machine.transitions() {
        println!("{} -> {}", transition.source(), transition.target());
    }

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
