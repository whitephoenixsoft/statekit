use std::sync::Arc;
use crate::{Machine, MachineInner, StateError};

/// The instance of a state machine.
///
/// A machine instance is always on a valid state.
/// All states have already
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
    
    ///A state is terminal when it has no outgoing transitions.
    pub fn is_terminal(&self) -> bool {
        self.machine.is_terminal(self.state())
    }
    
    /// Returns a shared handle to this instance's machine definition.
    pub fn machine(&self) -> Machine {
        Machine::from_inner(Arc::clone(&self.machine))
    }
}

/*
 Construction
- known source state succeeds
- known target-only state succeeds       [if we choose existing-state semantics]
- unknown state fails
- exact-match behavior is preserved

State observation
- state() returns initial state
- state() changes after successful transition

Transition query
- can_transition_to() true for allowed edge
- false for disallowed edge
- false from terminal state

Mutation
- successful transition changes current state
- multiple sequential transitions work
- failed transition leaves state unchanged
- transition validation uses current state, not initial state

Independence
- two instances from same Machine mutate independently

Ownership
- instance remains usable after original Machine handle is dropped
 */

#[cfg(test)]
mod tests {
    use super::*;

}
