use statekit::{Machine, StateError};

fn workflow_machine() -> Result<Machine, StateError> {
    Machine::builder()
        .allow("queued", "running")
        .allow("running", "completed")
        .allow("running", "failed")
        .build()
}

#[test]
fn validates_configured_workflow_transitions() -> Result<(), StateError> {
    let machine = workflow_machine()?;

    assert!(machine.validate_transition("queued", "running").is_ok());
    assert!(machine.validate_transition("running", "completed").is_ok());
    assert!(machine.validate_transition("running", "failed").is_ok());

    Ok(())
}

#[test]
fn rejects_transition_not_defined_by_the_workflow() -> Result<(), StateError> {
    let machine = workflow_machine()?;

    let result = machine.validate_transition("running", "invalid");

    assert_eq!(
        result,
        Err(StateError::InvalidTransition {
            from: "running".to_owned(),
            to: "invalid".to_owned(),
        })
    );

    Ok(())
}

#[test]
fn rejects_an_empty_machine_definition() {
    let result = Machine::builder().build();

    assert_eq!(result, Err(StateError::NoTransitions));
}

#[test]
fn rejects_a_self_transition() {
    let result = Machine::builder().allow("running", "running").build();

    assert_eq!(
        result,
        Err(StateError::SelfTransition {
            state: "running".to_owned(),
        })
    );
}
