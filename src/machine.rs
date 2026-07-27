use std::collections::{HashMap, HashSet};

use crate::MachineBuilder;
use crate::StateError;
use crate::StateName;

/// An immutable state-machine definition.
///
/// A `Machine` always contains at least one transition, and every stored
/// transition has valid, non-empty endpoints.
#[derive(Debug, PartialEq)]
pub struct Machine {
    transitions: HashMap<StateName, HashSet<String>>,
}

impl Machine {
    /// Private contructor for a new machine
    pub(crate) fn new(transitions: HashMap<StateName, HashSet<String>>) -> Self {
        Self { transitions }
    }
    /// Builder for creating a new Machine. This is the only way to instantiate it.
    pub fn builder() -> MachineBuilder {
        MachineBuilder::new()
    }

    /// Checks if a transition is possible in the state machine.
    ///
    /// Returns true if possible, false if not possible.
    pub fn can_transition(&self, from: &str, to: &str) -> bool {
        self.transitions
            .get(from)
            .is_some_and(|targets| targets.contains(to))
    }

    /// Validates a transition.
    ///
    /// Returns Ok() if valid, or a StateError if not.
    pub fn validate_transition(&self, from: &str, to: &str) -> Result<(), StateError> {
        if self.can_transition(from, to) {
            Ok(())
        } else {
            Err(StateError::InvalidTransition {
                from: from.to_string(),
                to: to.to_string(),
            })
        }
    }

    /// Returns the number of transitions in the state machine.
    pub fn transition_count(&self) -> usize {
        self.transitions.values().map(|s| s.len()).sum()
    }

    /// Return true if the a state exist in any transition.
    pub fn contains_state(&self, state: &str) -> bool {
        self.transitions.contains_key(state)
            || self.transitions.values().any(|targets| targets.contains(state))
    }

    /// Returns an iterator of the target states from the state specified.
    pub fn targets(&self, from: &str) -> Option<impl Iterator<Item = &str>> {
        Some(self.transitions.get(from)?.iter().map(String::as_str))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_transition_exists_returns_next_state() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();

        assert!(m.validate_transition("start", "finish").is_ok());
    }

    #[test]
    fn transition_count_1_transition() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();

        assert_eq!(m.transition_count(), 1);
    }

    #[test]
    fn validate_transition_exists_multiple_states_returns_next_state() {
        let builder = Machine::builder().allow("start", "finish").allow("1", "2");

        let m = builder.build().unwrap();

        assert!(m.validate_transition("start", "finish").is_ok());
    }

    #[test]
    fn transition_count_2_transitions() {
        let builder = Machine::builder().allow("start", "finish").allow("1", "2");

        let m = builder.build().unwrap();

        assert_eq!(m.transition_count(), 2);
    }

    #[test]
    fn can_transition_exists_returns_true() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();

        assert!(m.can_transition("start", "finish"));
    }

    #[test]
    fn validate_transition_not_exists_returns_invalid_error() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();

        assert_eq!(
            m.validate_transition("start", "invalid"),
            Err(StateError::InvalidTransition {
                from: "start".to_string(),
                to: "invalid".to_string(),
            })
        );
    }

    #[test]
    fn can_transition_not_exists_returns_false() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();

        assert_eq!(m.can_transition("start", "invalid"), false);
    }

    #[test]
    fn validate_transition_cyclic_is_valid() {
        let builder = Machine::builder()
            .allow("start", "finish")
            .allow("finish", "start");

        let m = builder.build().unwrap();

        assert!(m.validate_transition("finish", "start").is_ok());
    }

    #[test]
    fn contains_state_one_transition_existing_source_state_returns_true() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("start"));
    }

    #[test]
    fn contains_state_one_transition_existing_target_state_returns_true() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("finish"));
    }

    #[test]
    fn contains_state_one_transition_nonexisting_state_returns_false() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("other") == false);
    }

    #[test]
    fn contains_state_two_transitions_on_same_source_state_returns_true() {
        let builder = Machine::builder()
            .allow("start", "rest")
            .allow("start", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("start"));
    }

    #[test]
    fn contains_state_two_transitions_on_same_source_state_finds_first_target_state() {
        let builder = Machine::builder()
            .allow("start", "rest")
            .allow("start", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("rest"));
    }

    #[test]
    fn contains_state_two_transitions_on_same_source_state_finds_second_target_state() {
        let builder = Machine::builder()
            .allow("start", "rest")
            .allow("start", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("finish"));
    }

    #[test]
    fn contains_state_two_transitions_on_different_source_states_finds_first_source_state() {
        let builder = Machine::builder()
            .allow("start", "end")
            .allow("rest", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("start"));
    }

    #[test]
    fn contains_state_two_transitions_on_different_source_state_finds_second_source_state() {
        let builder = Machine::builder()
            .allow("start", "end")
            .allow("rest", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("rest"));
    }

    #[test]
    fn contains_state_two_transitions_on_different_source_state_finds_first_target_state() {
        let builder = Machine::builder()
            .allow("start", "end")
            .allow("rest", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("end"));
    }

    #[test]
    fn contains_state_two_transitions_on_different_source_states_finds_second_target_state() {
        let builder = Machine::builder()
            .allow("start", "end")
            .allow("rest", "finish");

        let m = builder.build().unwrap();

        assert!(m.contains_state("finish"));
    }

    #[test]
    fn targets_one_transition_key_does_not_exist_returns_none() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();
        let iter = m.targets("other");

        assert!(iter.is_none());
    }

    #[test]
    fn targets_one_transition_one_value() {
        let builder = Machine::builder().allow("start", "finish");

        let m = builder.build().unwrap();
        let collected: Vec<_> = m.targets("start").into_iter().flatten().collect();

        assert_eq!(collected, vec!["finish"]);
    }

    #[test]
    fn targets_one_transition_two_values() {
        let builder = Machine::builder().allow("start", "1").allow("start", "2");

        let m = builder.build().unwrap();
        let mut collected: Vec<_> = m.targets("start").into_iter().flatten().collect();
        collected.sort();

        assert_eq!(collected, vec!["1", "2"]);
    }

    #[test]
    fn targets_one_transition_three_values() {
        let builder = Machine::builder()
            .allow("start", "1")
            .allow("start", "2")
            .allow("start", "3");

        let m = builder.build().unwrap();
        let mut collected: Vec<_> = m.targets("start").into_iter().flatten().collect();
        collected.sort();

        assert_eq!(collected, vec!["1", "2", "3"]);
    }
}
