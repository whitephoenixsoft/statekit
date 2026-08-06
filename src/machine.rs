use std::collections::{HashMap, HashSet};

use crate::{MachineBuilder, StateError, StateName};

/// An immutable state-machine definition.
///
/// A `Machine` always contains at least one transition, and every stored
/// transition has valid, non-empty endpoints.
#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    transitions: HashMap<StateName, HashSet<StateName>>,
}

impl Machine {
    /// Constructs a machine from a validated transition table.
    pub(crate) fn new(transitions: HashMap<StateName, HashSet<StateName>>) -> Self {
        Self { transitions }
    }

    /// Returns a builder for constructing a [`Machine`].
    ///
    /// This is the public entry point for creating machine definitions.
    pub fn builder() -> MachineBuilder {
        MachineBuilder::new()
    }

    /// Returns whether the transition from `from` to `to` is allowed.
    pub fn can_transition(&self, from: &str, to: &str) -> bool {
        self.transitions
            .get(from)
            .is_some_and(|targets| targets.contains(to))
    }

    /// Validates that the transition from `from` to `to` is allowed.
    ///
    /// # Errors
    ///
    /// Returns [`StateError::InvalidTransition`] when the machine does not
    /// contain the requested transition.
    pub fn validate_transition(&self, from: &str, to: &str) -> Result<(), StateError> {
        if self.can_transition(from, to) {
            Ok(())
        } else {
            Err(StateError::InvalidTransition {
                from: from.to_owned(),
                to: to.to_owned(),
            })
        }
    }

    /// Returns the number of transitions in the state machine.
    pub fn transition_count(&self) -> usize {
        self.transitions.values().map(HashSet::len).sum()
    }

    /// Returns whether `state` appears as either endpoint of a transition.
    pub fn contains_state(&self, state: &str) -> bool {
        self.transitions.contains_key(state)
            || self
                .transitions
                .values()
                .any(|targets| targets.contains(state))
    }

    /// Returns an iterator over states directly reachable from `from`.
    ///
    /// Returns `None` when `from` has no outgoing transitions. This includes
    /// states that appear only as transition targets.
    ///
    /// The iteration order is unspecified.
    pub fn targets(&self, from: &str) -> Option<impl Iterator<Item = &str>> {
        Some(self.transitions.get(from)?.iter().map(StateName::as_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_transition_exists_returns_ok() -> Result<(), StateError> {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;

        assert!(m.validate_transition("start", "finish").is_ok());

        Ok(())
    }

    #[test]
    fn transition_count_1_transition() -> Result<(), StateError> {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;

        assert_eq!(m.transition_count(), 1);

        Ok(())
    }

    #[test]
    fn validate_transition_exists_multiple_states_returns_ok() -> Result<(), StateError> {
        let builder = Machine::builder()
            .try_allow("start", "finish")?
            .try_allow("1", "2")?;

        let m = builder.build()?;

        assert!(m.validate_transition("start", "finish").is_ok());

        Ok(())
    }

    #[test]
    fn transition_count_2_transitions() -> Result<(), StateError> {
        let builder = Machine::builder()
            .try_allow("start", "finish")?
            .try_allow("1", "2")?;

        let m = builder.build()?;

        assert_eq!(m.transition_count(), 2);

        Ok(())
    }

    #[test]
    fn can_transition_exists_returns_true() -> Result<(), StateError> {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;

        assert!(m.can_transition("start", "finish"));

        Ok(())
    }

    #[test]
    fn validate_transition_not_exists_returns_invalid_error() -> Result<(), StateError> {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;

        assert_eq!(
            m.validate_transition("start", "invalid"),
            Err(StateError::InvalidTransition {
                from: "start".to_string(),
                to: "invalid".to_string(),
            })
        );

        Ok(())
    }

    #[test]
    fn can_transition_not_exists_returns_false() -> Result<(), StateError> {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;

        assert!(!m.can_transition("start", "invalid"));

        Ok(())
    }

    #[test]
    fn validate_transition_cyclic_is_valid() -> Result<(), StateError> {
        let builder = Machine::builder()
            .try_allow("start", "finish")?
            .try_allow("finish", "start")?;

        let m = builder.build()?;

        assert!(m.validate_transition("finish", "start").is_ok());

        Ok(())
    }

    #[test]
    fn contains_state_one_transition_existing_source_state_returns_true() -> Result<(), StateError>
    {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;

        assert!(m.contains_state("start"));

        Ok(())
    }

    #[test]
    fn contains_state_one_transition_existing_target_state_returns_true() -> Result<(), StateError>
    {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;

        assert!(m.contains_state("finish"));

        Ok(())
    }

    #[test]
    fn contains_state_one_transition_nonexisting_state_returns_false() -> Result<(), StateError> {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;

        assert!(!m.contains_state("other"));

        Ok(())
    }

    #[test]
    fn contains_state_two_transitions_on_different_source_state_finds_second_source_state()
    -> Result<(), StateError> {
        let builder = Machine::builder()
            .try_allow("start", "end")?
            .try_allow("rest", "finish")?;

        let m = builder.build()?;

        assert!(m.contains_state("rest"));

        Ok(())
    }

    #[test]
    fn targets_one_transition_key_does_not_exist_returns_none() -> Result<(), StateError> {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;
        let iter = m.targets("other");

        assert!(iter.is_none());

        Ok(())
    }

    #[test]
    fn targets_one_transition_one_value() -> Result<(), StateError> {
        let builder = Machine::builder().try_allow("start", "finish")?;

        let m = builder.build()?;
        let collected: Vec<_> = m.targets("start").into_iter().flatten().collect();

        assert_eq!(collected, vec!["finish"]);

        Ok(())
    }

    #[test]
    fn targets_one_transition_two_values() -> Result<(), StateError> {
        let builder = Machine::builder()
            .try_allow("start", "1")?
            .try_allow("start", "2")?;

        let m = builder.build()?;
        let mut collected: Vec<_> = m.targets("start").into_iter().flatten().collect();
        collected.sort();

        assert_eq!(collected, vec!["1", "2"]);

        Ok(())
    }

    #[test]
    fn targets_one_transition_three_values() -> Result<(), StateError> {
        let builder = Machine::builder()
            .try_allow("start", "1")?
            .try_allow("start", "2")?
            .try_allow("start", "3")?;

        let m = builder.build()?;
        let mut collected: Vec<_> = m.targets("start").into_iter().flatten().collect();
        collected.sort();

        assert_eq!(collected, vec!["1", "2", "3"]);

        Ok(())
    }

    #[test]
    fn targets_target_only_state_returns_none() -> Result<(), StateError> {
        let machine = Machine::builder().try_allow("start", "finish")?.build()?;

        assert!(machine.contains_state("finish"));
        assert!(machine.targets("finish").is_none());

        Ok(())
    }

    #[test]
    fn duplicate_transition_is_stored_once() -> Result<(), StateError> {
        let machine = Machine::builder()
            .try_allow("start", "finish")?
            .try_allow("start", "finish")?
            .build()?;

        assert_eq!(machine.transition_count(), 1);

        let targets: Vec<_> = machine.targets("start").unwrap().collect();
        assert_eq!(targets, vec!["finish"]);

        Ok(())
    }
}
