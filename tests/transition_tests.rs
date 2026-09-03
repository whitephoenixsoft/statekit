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

    assert!(matches!(
        result,
        Err(StateError::InvalidTransition {
            ref from,
            ref to
        }) if from == "running" && to == "invalid"
    ));

    Ok(())
}

#[test]
fn rejects_an_empty_machine_definition() {
    let result = Machine::builder().build();

    assert!(matches!(result, Err(StateError::NoTransitions)));
}

#[test]
fn rejects_a_self_transition() {
    let result = Machine::builder().try_allow("running", "running");

    assert!(matches!(
        result,
        Err(StateError::SelfTransition {
            ref state
        }) if state == "running"
    ));
}

#[test]
fn rejects_ambiguous_state_names() {
    let result = Machine::builder().try_allow("queued ", "running");

    assert!(matches!(result, Err(StateError::AmbiguousStateName { ref state }) if state ==  "queued " ));
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
    
    let mut transitions: Vec<_> = machine.transitions().map(|item| (item.source(), item.target())).collect();
    transitions.sort();
    
    assert_eq!(transitions, vec![("queued", "running"), ("running", "completed"), ("running", "failed")]);

    Ok(())
}

#[test]
fn targets_from_unknown_state_returns_empty_iterator() -> Result<(), StateError> {
    let machine = workflow_machine()?;

    let targets: Vec<_> = machine.targets_from("unknown").collect();

    assert!(targets.is_empty());

    Ok(())
}

#[test]
fn targets_from_target_only_state_returns_empty_iterator() -> Result<(), StateError> {
    let machine = workflow_machine()?;

    assert!(machine.contains_state("completed"));

    let targets: Vec<_> = machine.targets_from("completed").collect();

    assert!(targets.is_empty());

    Ok(())
}

#[test]
fn rejects_whitespace_only_state_names() {
    let result = Machine::builder().try_allow(" \n\t", "running");

    assert!(matches!(result, Err(StateError::EmptyState)));
}
