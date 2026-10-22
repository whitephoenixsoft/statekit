use statekit::{Machine, StateError};
    
fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

    machine.validate_transition("queued", "running")?;

    assert!(machine.can_transition("queued", "running"));
    assert!(!machine.can_transition("queued", "completed"));

    Ok(())
}
