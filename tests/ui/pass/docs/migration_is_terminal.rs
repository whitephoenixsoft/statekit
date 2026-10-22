use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    // BEGIN DOCUMENTATION EXAMPLE
    
    let machine = Machine::builder()
        .try_allow("draft", "approved")?
        .build()?;
        
    assert!(!machine.is_terminal("draft"));
    assert!(machine.is_terminal("approved"));
        
    let mut instance = machine.instance("draft")?;
    assert!(!instance.is_terminal());

    instance.transition_to("approved")?;
    assert!(instance.is_terminal());

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
