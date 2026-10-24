// Source: README.md
// Section: Validation Example
// Compatibility: current
// Introduced: 0.4
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .build()?;

    // BEGIN DOCUMENTATION EXAMPLE
    
    for source in machine.sources() {
        println!("{source}");

        for target in machine.targets_from(source) {
            println!(" -> {target}");
        }
    }

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
