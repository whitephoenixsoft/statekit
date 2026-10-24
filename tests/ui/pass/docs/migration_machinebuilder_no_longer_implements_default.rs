// Source: MIGRATION.md
// Section: `MachineBuilder` no longer implements `Default` or `PartialEq`
// Compatibility: current
// Introduced: 0.3
use statekit::Machine;

#[allow(unused)]
fn main() {
    // BEGIN DOCUMENTATION EXAMPLE
    
    let builder = Machine::builder();

    // END DOCUMENTATION EXAMPLE
}
