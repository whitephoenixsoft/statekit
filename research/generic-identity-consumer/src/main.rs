use statekit::generic_identity::{IdentityError, Machine as IdentityMachine};
use statekit::{Machine, StateError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Phase {
    Idle,
    Running,
    Done,
}

fn configuration_consumer() -> Result<(), StateError> {
    // This is the existing crate-root API. Configuration data remains strings.
    let machine = Machine::builder()
        .try_allow("idle", "running")?
        .try_allow("running", "done")?
        .build()?;
    let mut instance = machine.instance("idle")?;
    instance.transition_to("running")?;
    assert_eq!(instance.state(), "running");
    Ok(())
}

fn custom_id_consumer() -> Result<(), IdentityError<Phase>> {
    // The additive spike API consumes resolved IDs directly and keeps a
    // separate spelling to avoid colliding with the string builder.
    let machine = IdentityMachine::<Phase>::builder_with_identity()
        .try_allow_identity(Phase::Idle, Phase::Running)?
        .try_allow_identity(Phase::Running, Phase::Done)?
        .build()?;
    let mut instance = machine.instance_identity(Phase::Idle)?;
    instance.transition_to_identity(Phase::Running)?;
    assert!(machine.can_transition_identity(&Phase::Running, &Phase::Done));
    assert_eq!(instance.state_identity(), &Phase::Running);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configuration_consumer()?;
    custom_id_consumer()?;
    Ok(())
}
