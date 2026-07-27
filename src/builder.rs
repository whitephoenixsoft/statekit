use std::collections::{HashMap, HashSet};

use crate::Machine;
use crate::StateError;
use crate::StateName;

/*
Contraints
- empty string invalid
- self-transition invalid
- case-sensitive by default
- cycles allowed
- terminal states need no special behavior yet
*/

/// The builder for the Machine.
///
/// Adds transitions to the state-machine by allowing them, then builds and immutable `Machine`.
///
/// Validate transitions must connect to two different states, and states must be not empty.
/// When building the `Machine` there must be at least one transition.
#[derive(Debug, PartialEq)]
pub struct MachineBuilder {
    transitions: HashMap<String, HashSet<String>>,
}

impl MachineBuilder {
    /// Create a new builder.
    pub(crate) fn new() -> Self {
        Self {
            transitions: HashMap::new(),
        }
    }

    /// Add a new transition from one state to another.
    ///
    /// Identical transitions are ignored.
    pub fn allow(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.transitions
            .entry(from.into())
            .or_default()
            .insert(to.into());

        self
    }

    /// (experimental) Add new transition from on state to another while validating it.
    pub fn try_allow(
        self,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Result<Self, StateError> {
        let from = from.into();
        let to = to.into();

        if from.is_empty() || to.is_empty() {
            return Err(StateError::EmptyState);
        } else if from == to {
            return Err(StateError::SelfTransition {
                state: from.to_string(),
            });
        }

        Ok(self.allow(from, to))
    }

    /// Validate the transitions and build them into an immutable state Machine.
    pub fn build(self) -> Result<Machine, StateError> {
        if self.transitions.is_empty() {
            return Err(StateError::NoTransitions);
        }

        for transition in self.transitions.iter() {
            if transition.0.is_empty() || transition.1.contains("") {
                return Err(StateError::EmptyState);
            } else if transition.1.contains(transition.0) {
                return Err(StateError::SelfTransition {
                    state: transition.0.clone(),
                });
            }
        }

        Ok(Machine::new(
            self.transitions
                .iter()
                .map(|s| (StateName::from(s.0.as_str()), s.1.clone()))
                .collect(),
        ))
    }

    /// Returns the number of transitions added to the state machine.
    pub fn transition_count(&self) -> usize {
        self.transitions.values().map(|s| s.len()).sum()
    }

    /// Returns the number of unique states in the state machine.
    pub fn state_count(&self) -> usize {
        let mut unique = HashSet::new();

        for (key, set) in &self.transitions {
            unique.insert(key.clone());
            unique.extend(set.iter().cloned());
        }

        unique.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn state_count_new_builder_returns_0() {
        let builder = MachineBuilder::new();

        assert_eq!(builder.state_count(), 0);
    }

    #[test]
    fn transition_count_new_builder_returns_0() {
        let builder = MachineBuilder::new();

        assert_eq!(builder.transition_count(), 0);
    }

    #[test]
    fn allow_once_has_two_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");

        assert_eq!(builder.state_count(), 2);
    }

    #[test]
    fn allow_once_has_one_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");

        assert_eq!(builder.transition_count(), 1);
    }

    #[test]
    fn allow_twice_same_from_state_has_three_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "finish2");

        assert_eq!(builder.state_count(), 3);
    }

    #[test]
    fn allow_twice_same_from_state_has_two_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "finish2");

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn allow_twice_same_from_state_same_transition_has_two_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "finish");

        assert_eq!(builder.state_count(), 2);
    }

    #[test]
    fn allow_twice_same_from_state_same_transition_has_one_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "finish");

        assert_eq!(builder.transition_count(), 1);
    }

    #[test]
    fn allow_twice_different_states_has_four_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start2", "finish2");

        assert_eq!(builder.state_count(), 4);
    }

    #[test]
    fn allow_twice_different_states_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start2", "finish2");

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn allow_twice_connnected_transitions_have_three_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "mid");
        let builder = builder.allow("mid", "finish");

        assert_eq!(builder.state_count(), 3);
    }

    #[test]
    fn allow_twice_connected_transitions_have_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "mid");
        let builder = builder.allow("mid", "finish");

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn allow_case_sensitive_from_has_three_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("Start", "finish");

        assert_eq!(builder.state_count(), 3);
    }

    #[test]
    fn allow_case_sensitive_from_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("Start", "finish");

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn allow_case_sensitive_to_has_three_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "Finish");

        assert_eq!(builder.state_count(), 3);
    }

    #[test]
    fn allow_case_sensitive_to_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "Finish");

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn build_empty_from_state_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("", "finish");

        assert_eq!(builder.build(), Err(StateError::EmptyState));
    }

    #[test]
    fn build_empty_to_state_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "");

        assert_eq!(builder.build(), Err(StateError::EmptyState));
    }

    #[test]
    fn build_transition_to_self_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "start");

        assert_eq!(
            builder.build(),
            Err(StateError::SelfTransition {
                state: "start".to_string(),
            })
        );
    }

    #[test]
    fn build_empty_build_invalid() {
        let builder = MachineBuilder::new();

        assert_eq!(builder.build(), Err(StateError::NoTransitions));
    }

    #[test]
    fn try_allow_once_has_two_states() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();

        assert_eq!(builder.state_count(), 2);
    }

    #[test]
    fn try_allow_once_has_one_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();

        assert_eq!(builder.transition_count(), 1);
    }

    #[test]
    fn try_allow_twice_same_from_state_has_three_state() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("start", "finish2").unwrap();

        assert_eq!(builder.state_count(), 3);
    }

    #[test]
    fn try_allow_twice_same_from_state_has_two_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("start", "finish2").unwrap();

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn try_allow_twice_same_transition_has_two_states() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("start", "finish").unwrap();

        assert_eq!(builder.state_count(), 2);
    }

    #[test]
    fn try_allow_twice_same_transition_has_one_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("start", "finish").unwrap();

        assert_eq!(builder.transition_count(), 1);
    }

    #[test]
    fn try_allow_twice_different_from_states_has_four_states() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("start2", "finish2").unwrap();

        assert_eq!(builder.state_count(), 4);
    }

    #[test]
    fn try_allow_twice_different_from_state_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("start2", "finish2").unwrap();

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn try_allow_twice_connected_transition_has_three_states() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "mid").unwrap();
        let builder = builder.try_allow("mid", "finish").unwrap();

        assert_eq!(builder.state_count(), 3);
    }

    #[test]
    fn try_allow_twice_connected_transition_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "mid").unwrap();
        let builder = builder.try_allow("mid", "finish").unwrap();

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn try_allow_case_sensitive_from_state_same_transition_has_three_states() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("Start", "finish").unwrap();

        assert_eq!(builder.state_count(), 3);
    }

    #[test]
    fn try_allow_case_sensitive_state_from_same_transition_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("Start", "finish").unwrap();

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn try_allow_case_sensitive_to_state_same_transition_has_three_states() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("start", "Finish").unwrap();

        assert_eq!(builder.state_count(), 3);
    }

    #[test]
    fn try_allow_case_sensitive_to_state_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "finish").unwrap();
        let builder = builder.try_allow("start", "Finish").unwrap();

        assert_eq!(builder.transition_count(), 2);
    }

    #[test]
    fn try_allow_empty_from_state_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("", "finish");

        assert_eq!(builder, Err(StateError::EmptyState));
    }

    #[test]
    fn try_allow_empty_to_state_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "");

        assert_eq!(builder, Err(StateError::EmptyState));
    }

    #[test]
    fn try_allow_transaction_to_self_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.try_allow("start", "start");

        assert_eq!(
            builder,
            Err(StateError::SelfTransition {
                state: "start".to_string(),
            })
        );
    }
}
