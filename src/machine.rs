use crate::{MachineBuilder, StateError, Transition, Transitions};

/// An immutable state-machine definition.
///
/// A `Machine` contains at least one transition. Every stored state name and
/// transition has already been validated by the builder.
#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    transitions: Transitions,
}

impl Machine {
    /// Constructs a machine from a validated transition table.
    pub(crate) fn new(transitions: Transitions) -> Self {
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
        let transition = Transition::new(from, to);

        self.transitions.contains(transition)
    }

    /// Validates that the transition from `from` to `to` is allowed.
    ///
    /// # Errors
    ///
    /// Returns [`StateError::InvalidTransition`] when the machine does not
    /// contain the requested transition.
    /// State names are matched exactly. This method does not trim, normalize,
    /// or otherwise modify the supplied names.
    pub fn validate_transition(&self, from: &str, to: &str) -> Result<(), StateError> {
        let transition = Transition::new(from, to);

        if self.transitions.contains(transition) {
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
        self.transitions.len()
    }

    /// Returns whether `state` appears as either endpoint of a transition.
    pub fn contains_state(&self, state: &str) -> bool {
        self.transitions.contains_state(state)
    }

    /// Returns an iterator over states directly reachable from `from`.
    ///
    /// Returns `None` when `from` has no outgoing transitions. This includes
    /// states that appear only as transition targets.
    ///
    /// The iteration order is unspecified.
    #[deprecated(since = "0.2.0", note = "use `targets_from` instead")]
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
        self.transitions.targets_from(from)
    }

    /// Returns an iterator over all source states.
    ///
    /// The iteration order is unspecified.
    pub fn sources(&self) -> impl Iterator<Item = &str> {
        self.transitions.sources()
    }

    /// Returns an iterator over all unique source and target states.
    ///
    /// The iteration order is unspecified.
    pub fn states(&self) -> impl Iterator<Item = &str> {
        self.transitions.states()
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
        fn contains_state_finds_target_only_state() -> Result<(), StateError> {
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
        fn contains_state_finds_source_state() -> Result<(), StateError> {
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

    mod sources {
        use super::*;

        #[test]
        fn sources_one_source_one_value() -> Result<(), StateError> {
            let machine = Machine::builder().try_allow("start", "finish")?.build()?;

            let sources: Vec<_> = machine.sources().collect();

            assert_eq!(sources, vec!["start"]);

            Ok(())
        }

        #[test]
        fn returns_all_source_states() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "0")?
                .try_allow("2", "0")?
                .try_allow("3", "0")?
                .build()?;

            let mut sources: Vec<_> = machine.sources().collect();
            sources.sort();

            assert_eq!(sources, vec!["1", "2", "3"]);

            Ok(())
        }
    }

    mod states {
        use super::*;

        #[test]
        fn states_one_transition_returns_2_values() -> Result<(), StateError> {
            let machine = Machine::builder().try_allow("1", "2")?.build()?;

            let mut states: Vec<_> = machine.states().collect();
            states.sort();

            assert_eq!(states, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn returns_unique_source_and_target_states() -> Result<(), StateError> {
            let machine = Machine::builder()
                .try_allow("1", "2")?
                .try_allow("1", "3")?
                .try_allow("2", "3")?
                .try_allow("3", "4")?
                .build()?;

            let mut states: Vec<_> = machine.states().collect();
            states.sort();

            assert_eq!(states, vec!["1", "2", "3", "4"]);

            Ok(())
        }

        #[test]
        fn includes_target_only_states() -> Result<(), StateError> {
            let machine = Machine::builder().try_allow("queued", "running")?.build()?;

            let mut states: Vec<_> = machine.states().collect();
            states.sort();

            assert_eq!(states, vec!["queued", "running"]);

            Ok(())
        }
    }
}
