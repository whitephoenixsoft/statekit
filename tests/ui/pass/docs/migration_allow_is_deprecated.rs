// Source: MIGRATION.md
// Section: `MachineBuilder::allow` is deprecated
// Compatibility: current
// Introduced: 0.2
use statekit::{Machine, StateError};

#[allow(deprecated)]
#[allow(unused_variables)]
fn main() -> Result<(), StateError> {
    // BEGIN DOCUMENTATION EXAMPLE
    
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .build()?;

    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .build()?;

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
