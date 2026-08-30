use statekit::{Machine, StateError};

fn workflow_machine() -> Result<Machine, StateError> {
    Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()
}

#[test]
fn validates_configured_workflow_transitions() -> Result<(), StateError> {
    let machine = workflow_machine()?;
    machine.validate_transition("queued", "running")?;

    machine.validate_transition("running", "completed")?;

    machine.validate_transition("running", "failed")?;

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
    let result = Machine::builder().try_allow("running", "running");

    assert_eq!(
        result,
        Err(StateError::SelfTransition {
            state: "running".to_owned(),
        })
    );
}

#[test]
fn rejects_ambiguous_state_names() {
    let result = Machine::builder().try_allow("queued ", "running");

    assert_eq!(result, Err(StateError::AmbiguousStateName { state: "queued ".to_owned() }));
}

#[test]
fn exposes_machine_structure_for_inspection() -> Result<(), StateError> {
    let machine = workflow_machine()?;

    let mut sources: Vec<_> = machine.sources().collect();
    sources.sort();

    assert_eq!(sources, vec!["queued", "running"]);

    let targets: Vec<_> = machine.targets_from("queued").collect();

    assert_eq!(targets, vec!["running"]);

    let mut states: Vec<_> = machine.states().collect();
    states.sort();

    assert_eq!(states, vec!["completed", "failed", "queued", "running"]);

    Ok(())
}
