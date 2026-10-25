use statekit::{Machine, StateError, Transition};

fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .build()?;

    let transition = machine.transitions().next().unwrap();
    
    let mutable_transition: &mut Transition = transition;

    Ok(())
}