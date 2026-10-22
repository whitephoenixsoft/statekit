use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    // BEGIN DOCUMENTATION EXAMPLE

    let machine = Machine::builder()
        .try_allow("draft", "approved")?
        .try_allow("approved", "draft")?
        .build()?;
        
    let mut instance = machine.instance("draft")?;
    assert_eq!(instance.state(), "draft");

    instance.transition_to("approved")?;
    assert_eq!(instance.state(), "approved");
    assert!(instance.can_transition_to("draft"));

    instance.transition_to("draft")?;
    assert_eq!(instance.state(), "draft");
    assert!(instance.can_transition_to("approved"));

    // END DOCUMENTATION EXAMPLE

    Ok(())
}
