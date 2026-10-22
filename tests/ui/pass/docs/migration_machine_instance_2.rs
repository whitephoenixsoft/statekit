use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    // BEGIN DOCUMENTATION EXAMPLE

    let machine1 = Machine::builder()
        .try_allow("draft", "approved")?
        .build()?;
        
    let machine2 = Machine::builder()
        .try_allow("draft", "approved")?
        .build()?;

    // Equal definitions constructed independently
    let instance1 = machine1.instance("draft")?;
    let instance2 = machine2.instance("draft")?;

    assert_eq!(instance1, instance2);

    // Instances created from the same Machine are also equal
    // when their current states are equal.
    let instance3 = machine1.instance("draft")?;
    assert_eq!(instance1, instance3);

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
