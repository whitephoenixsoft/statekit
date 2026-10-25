// Source: MIGRATION.md
// Section: `StateError::AmbiguousStateName` contains a `state` field
// Compatibility: current
// Introduced: 0.3
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let err  = Machine::builder()
        .try_allow(" draft", "approved");

    // BEGIN DOCUMENTATION EXAMPLE
        
    match err {
        Err(StateError::AmbiguousStateName { state }) => { println!("{state}"); }
        _ => {}
    }

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
