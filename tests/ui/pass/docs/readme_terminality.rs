use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    // BEGIN DOCUMENTATION EXAMPLE
    
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .build()?;

    let instance = machine.instance("running")?;

    assert!(machine.is_terminal("running"));
    assert!(instance.is_terminal());

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
