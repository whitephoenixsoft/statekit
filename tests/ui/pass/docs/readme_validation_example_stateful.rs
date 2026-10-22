use statekit::{Machine, StateError};
    
fn main() -> Result<(), StateError> {
    let machine = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "completed")?
        .try_allow("running", "failed")?
        .build()?;

   let mut instance = machine.instance("queued")?;

   instance.transition_to("running")?;

   assert!(instance.can_transition_to("completed"));
   assert!(!instance.can_transition_to("queued"));

   Ok(())
}
