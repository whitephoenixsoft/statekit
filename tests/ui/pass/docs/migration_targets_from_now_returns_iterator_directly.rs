// Source: MIGRATION.md
// Section: `Machine::targets_from` now returns an iterator directly
// Compatibility: current
// Introduced: 0.3
use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("source", "target")?
        .build()?;

    // BEGIN DOCUMENTATION EXAMPLE
    
    // Example 1
    let targets = machine.targets_from("source");

    for target in targets { 
        println!("{target}");
    }

    // Example 2
    let mut targets = machine.targets_from("no_source");

    if targets.next().is_none() {
        println!("no outgoing transitions");
    }

    // Example 3
    let targets: Vec<_> = machine.targets_from("no_source").collect();

    if targets.is_empty() {
        println!("no outgoing transitions");
    }

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
