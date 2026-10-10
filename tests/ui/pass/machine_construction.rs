use statekit::{Machine, StateError};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "finished")?
        .build()?;

    let mut instance = machine.instance("queued")?;

    let _ = machine.transition_count();
    let _ = machine.transitions().count();
    let _ = machine.targets_from("queued").count();

    instance.transition_to("running")?;

    let _ = instance.machine();

    Ok(())
}