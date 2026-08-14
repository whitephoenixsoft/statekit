use std::collections::{HashMap, HashSet};

use crate::{MachineBuilder, StateError, StateName};

/// An immutable state-machine definition.
///
/// A `Machine` contains at least one transition. Every stored state name and
/// transition has already been validated by the builder.
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
    /// contain the requested transition. Submitted `from` and `to`names must 
    /// match exactly the transition state names exactly.
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
    #[deprecated(
        since = "0.2.0",
        note = "use `targets_from` instead"
    )]
    pub fn targets(&self, from: &str) -> Option<impl Iterator<Item = &str>> {
        self.targets_from(from)
    }

    /// Returns an iterator over states directly reachable from `from`.
    ///
    /// Returns `None` when `from` has no outgoing transitions. This includes
    /// states that appear only as transition targets.
    ///
    /// The iteration order is unspecified.
    pub fn targets_from(&self, from: &str) -> Option<impl Iterator<Item = &str>> {
        Some(self.transitions.get(from)?.iter().map(StateName::as_str))
    }

    /// Returns an iterator over all source states.
    ///
    /// The iteration order is unspecified.
    pub fn sources(&self) -> impl Iterator<Item = &str> {
        self.transitions.keys().map(StateName::as_str)
    }


    /// Returns an iterator over all unique source and target states.
    ///
    /// The iteration order is unspecified.
    pub fn state(&self) -> impl Iterator<Item = &str> {
        let mut unique: HashSet<&StateName> = HashSet::new();

        for (from, target) in &self.transitions {
            unique.insert(from);
            unique.extend(target);
        }

        unique.into_iter().map(StateName::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod validate_transition {
        use super::*;

        #[test]
        fn validate_transition_accepts_configured_transition() -> Result<(), StateError> {
            let builder = Machine::builder()
                .try_allow("start", "finish")?
                .try_allow("1", "2")?;

            let m = builder.build()?;

            assert!(m.validate_transition("start", "finish").is_ok());

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
        fn validate_transition_cyclic_is_valid() -> Result<(), StateError> {
            let builder = Machine::builder()
                .try_allow("start", "finish")?
                .try_allow("finish", "start")?;

            let m = builder.build()?;

            assert!(m.validate_transition("finish", "start").is_ok());

            Ok(())
        }
    }

    mod transition_count {
        use super::*;

        #[test]
        fn transition_count_counts_transitions() -> Result<(), StateError> {
            let builder = Machine::builder()
                .try_allow("start", "finish")?
                .try_allow("1", "2")?;

            let m = builder.build()?;

            assert_eq!(m.transition_count(), 2);

            Ok(())
        }
    }

    mod can_transition {
        use super::*;

        #[test]
        fn can_transition_exists_returns_true() -> Result<(), StateError> {
            let builder = Machine::builder().try_allow("start", "finish")?;

            let m = builder.build()?;

            assert!(m.can_transition("start", "finish"));

            Ok(())
        }

        #[test]
        fn can_transition_not_exists_returns_false() -> Result<(), StateError> {
            let builder = Machine::builder().try_allow("start", "finish")?;

            let m = builder.build()?;

            assert!(!m.can_transition("start", "invalid"));

            Ok(())
        }
    }

    mod contains_state {
        use super::*;

        #[test]
        fn contains_state_finds_target_only_state() -> Result<(), StateError>
        {
            let builder = Machine::builder().try_allow("start", "finish")?;

            let m = builder.build()?;

            assert!(m.contains_state("finish"));

            Ok(())
        }

        #[test]
        fn contains_state_rejects_unknown_state() -> Result<(), StateError> {
            let builder = Machine::builder().try_allow("start", "finish")?;

            let m = builder.build()?;

            assert!(!m.contains_state("other"));

            Ok(())
        }

        #[test]
        fn contains_state_finds_source_state()
        -> Result<(), StateError> {
            let builder = Machine::builder()
                .try_allow("start", "end")?
                .try_allow("rest", "finish")?;

            let m = builder.build()?;

            assert!(m.contains_state("rest"));

            Ok(())
        }
    }

    mod targets_from {
        use super::*;

        #[test]
        fn targets_from_one_transition_key_does_not_exist_returns_none() -> Result<(), StateError> {
            let builder = Machine::builder().try_allow("start", "finish")?;

            let m = builder.build()?;
            let iter = m.targets_from("other");

            assert!(iter.is_none());

            Ok(())
        }

        #[test]
        fn targets_from_one_transition_one_value() -> Result<(), StateError> {
            let builder = Machine::builder().try_allow("start", "finish")?;

            let m = builder.build()?;
            let collected: Vec<_> = m.targets_from("start").into_iter().flatten().collect();

            assert_eq!(collected, vec!["finish"]);

            Ok(())
        }

        #[test]
        fn targets_from_one_transition_two_values() -> Result<(), StateError> {
            let builder = Machine::builder()
                .try_allow("start", "1")?
                .try_allow("start", "2")?;

            let m = builder.build()?;
            let mut collected: Vec<_> = m.targets_from("start").into_iter().flatten().collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn targets_from_one_transition_three_values() -> Result<(), StateError> {
            let builder = Machine::builder()
                .try_allow("start", "1")?
                .try_allow("start", "2")?
                .try_allow("start", "3")?;

            let m = builder.build()?;
            let mut collected: Vec<_> = m.targets_from("start").into_iter().flatten().collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2", "3"]);

            Ok(())
        }

        #[test]
        fn targets_from_target_only_state_returns_none() -> Result<(), StateError> {
            let machine = Machine::builder().try_allow("start", "finish")?.build()?;

            assert!(machine.contains_state("finish"));
            assert!(machine.targets_from("finish").is_none());

            Ok(())
        }

        #[test]
        fn targets_from_duplicate_transition_is_stored_once() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("start", "finish")?
                .try_allow("start", "finish")?
                .build()?;

            assert_eq!(machine.transition_count(), 1);

            let targets: Vec<_> = machine.targets_from("start").unwrap().collect();
            assert_eq!(targets, vec!["finish"]);

            Ok(())
        }
    }
}
