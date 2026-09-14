use std::sync::Arc;
use crate::{MachineInner, StateError};

/// The instance of a state machine.
pub struct MachineInstance {
    machine: Arc<MachineInner>,
    current: String,
}

impl MachineInstance {
    pub(crate) fn new(machine: Arc<MachineInner>,
    current: String) -> Self {
        Self {
            machine,
            current,
        }
    }
    
    /// Returns the current state.
    pub fn state(&self) -> &str {
        &self.current
    }

    /// Returns true if the instance can transition to `target`.
    pub fn can_transition_to(&self, target: &str) -> bool {
        self.machine.can_transition(&self.current, target)
    }

    /// Changes the state to `target`
    pub fn transition_to(&mut self, target: &str) -> Result<(), StateError> {
        self.machine
            .validate_transition(&self.current, target)?;

        self.current.clear();
        self.current.push_str(target);

        Ok(())
    }
}
