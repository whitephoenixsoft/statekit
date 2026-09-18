use std::sync::Arc;
use crate::{Machine, MachineInner, StateError};

/// The instance of a state machine.
///
/// A machine instance is always on a valid state.
/// All states have already been validated and part of a transaction.
pub struct MachineInstance {
    machine: Arc<MachineInner>,
    current: String,
}

impl MachineInstance {
    /// Returns an MachineInstance. Returns a StateError::UnknownInitialState
    /// if `initial` is not an existing state.
    pub(crate) fn try_new(machine: Arc<MachineInner>, initial: String) -> Result<Self, StateError> {
        if !machine.contains_state(initial.as_str()) {
            return Err(StateError::UnknownInitialState { state: initial });
        }

        Ok(Self {
            machine,
            current: initial,
        })
    }
    
    /// Returns the current state.
    pub fn state(&self) -> &str {
        &self.current
    }

    /// Returns true if the instance can transition to `target`.
    pub fn can_transition_to(&self, target: &str) -> bool {
        self.machine.can_transition(&self.current, target)
    }

    /// Changes the state to `target` if the transition is allowed.
    ///
    /// # Errors:
    ///
    /// Returns [`StateError::InvalidTransition`] when the transition is not allowed in the state machine.
    pub fn transition_to(&mut self, target: &str) -> Result<(), StateError> {
        self.machine .validate_transition(&self.current, target)?;

        self.current.clear();
        self.current.push_str(target);

        Ok(())
    }
    
    /// Returns true if the state has no outgoing transitions.
    pub fn is_terminal(&self) -> bool {
        self.machine.is_terminal(self.state())
    }
    
    /// Returns a shared handle to this instance's machine definition.
    pub fn machine(&self) -> Machine {
        Machine::from_inner(Arc::clone(&self.machine))
    }
}

/*
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

    mod construction {
        use super::*;

        #[test]
        fn initial_known_source_state_succeeds() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .build()?;

            let instance = machine.instance("1");

            assert!(instance.is_ok());

            Ok(())
        }

        #[test]
        fn initial_known_target_only_state_succeeds() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .build()?;

            let instance = machine.instance("2");

            assert!(instance.is_ok());

            Ok(())
        }

        #[test]
        fn initial_unknown_state_fails() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .build()?;

            let instance = machine.instance("3");

            assert!(instance.is_err());

            Ok(())
        }

        #[test]
        fn initial_exact_match_behavior_is_preserved() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .build()?;

            let instance = machine.instance(" 1 ");

            assert!(instance.is_err());

            Ok(())
        }
    }

    mod state {
        use super::*;

        #[test]
        fn state_initial_state_matches() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .build()?;

            let instance = machine.instance("1")?;

            assert_eq!(instance.state(), "1");

            Ok(())
        }

        #[test]
        fn state_changes_are_reflected() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .try_allow("2", "3")?
                .build()?;

            let mut instance = machine.instance("1")?;

            instance.transition_to("2")?; 

            assert_eq!(instance.state(), "2");

            Ok(())
        }
    }

    mod can_transition_to {
        use super::*;

        #[test]
        fn can_transition_to_returns_true_on_valid_edge() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .try_allow("2", "3")?
                .build()?;

            let instance = machine.instance("1")?;

            assert!(instance.can_transition_to("2"));

            Ok(())
        }

        #[test]
        fn can_transition_to_returns_false_on_disallowed_edge() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .try_allow("2", "3")?
                .build()?;

            let instance = machine.instance("2")?;

            assert!(!instance.can_transition_to("4"));

            Ok(())
        }

        #[test]
        fn can_transition_to_returns_false_on_terminal_state() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .try_allow("2", "3")?
                .build()?;

            let instance = machine.instance("3")?;

            assert!(!instance.can_transition_to("2"));

            Ok(())
        }
    }
}
