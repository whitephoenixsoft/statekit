use statekit::{Machine, StateError};

#[test]
fn transition_queued_to_running() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.transition("queued", "running")?;

    assert_eq!(state, "running");
    
    Ok(())
}


#[test]
fn transition_running_to_failed() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.transition("running", "failed")?;

    assert_eq!(state, "failed");
    
    Ok(())
}

#[test]
fn transition_running_to_completed() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.transition("running", "completed")?;

    assert_eq!(state, "completed");
    
    Ok(())
}

#[test]
fn transition_running_to_invalid_state() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.transition("running", "invalid");

    assert_eq!(state, 
        Err(StateError::InvalidTransition {
            from: "running".to_string(),
            to: "invalid".to_string(),
        }));
    
    Ok(())
}