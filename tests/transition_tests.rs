use statekit::{Machine, StateError};

// allow version of the tests

#[test]
fn validate_transition_allow_queued_to_running() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.validate_transition("queued", "running");

    assert!(state.is_ok());

    Ok(())
}

#[test]
fn validate_transition_allow_running_to_failed() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.validate_transition("running", "failed");

    assert!(state.is_ok());

    Ok(())
}

#[test]
fn validate_transition_allow_running_to_completed() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.validate_transition("running", "completed");

    assert!(state.is_ok());

    Ok(())
}

#[test]
fn validate_transition_allow_running_to_invalid_state() -> Result<(), StateError> {
    let machine = Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()?;

    let state = machine.validate_transition("running", "invalid");

    assert_eq!(
        state,
        Err(StateError::InvalidTransition {
            from: "running".to_string(),
            to: "invalid".to_string(),
        })
    );

    Ok(())
}

// try_allow version of the tests

#[test]
fn validate_transition_try_allow_queued_to_running() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

    let state = machine.validate_transition("queued", "running");

    assert!(state.is_ok());

    Ok(())
}

#[test]
fn validate_transition_try_allow_running_to_failed() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

    let state = machine.validate_transition("running", "failed");

    assert!(state.is_ok());

    Ok(())
}

#[test]
fn validate_transition_try_allow_running_to_completed() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

    let state = machine.validate_transition("running", "completed");

    assert!(state.is_ok());

    Ok(())
}

#[test]
fn validate_transition_try_allow_running_to_invalid_state() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

    let state = machine.validate_transition("running", "invalid");

    assert_eq!(
        state,
        Err(StateError::InvalidTransition {
            from: "running".to_string(),
            to: "invalid".to_string(),
        })
    );

    Ok(())
}
