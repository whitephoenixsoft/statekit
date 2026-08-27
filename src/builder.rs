use crate::{Machine, StateError, StateName, Transition, Transitions};

/// A builder for constructing a [`Machine`].
///
/// Transitions are added with [`MachineBuilder::try_allow`] and validated
/// immediately before being stored.
///
/// [`MachineBuilder::build`] only succeeds when at least one valid transition
/// has been configured.
///
/// State names are case-sensitive. Empty state names and self-transitions are
/// rejected. Cycles are allowed.
#[derive(Debug, PartialEq, Default)]
pub struct MachineBuilder {
    transitions: Transitions,
}

impl MachineBuilder {
    /// Creates an empty builder.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds an allowed transition from one state to another.
    ///
    /// Adding the same transition more than once has no additional effect.
    ///
    /// # Panics
    ///
    /// Panics if either state name is invalid or the transition is a self-transition.
    #[deprecated(
        since = "0.2.0",
        note = "use `try_allow` to handle invalid transitions without panicking"
    )]
    pub fn allow(self, from: impl AsRef<str>, to: impl AsRef<str>) -> Self {
        self.try_allow(from, to)
            .expect("invalid transition passed to MachineBuilder::allow")
    }

    /// Adds an allowed transition from one state to another.
    ///
    /// Adding the same transition more than once has no additional effect.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - [`StateError::AmbiguousStateName`] if an endpoint contains whitespace before or after
    ///   the name.
    /// - [`StateError::EmptyState`] if an endpoint is empty.
    /// - [`StateError::SelfTransition`] if a transition has identical endpoints.
    pub fn try_allow(
        mut self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> Result<Self, StateError> {
        let from = StateName::try_from(from.as_ref())?;
        let to = StateName::try_from(to.as_ref())?;

        let transition = Transition::try_new(from, to)?;

        self.transitions.add(transition);

        Ok(self)
    }

    /// Builds an immutable [`Machine`].
    ///
    /// # Errors
    ///
    /// Returns [`StateError::NoTransitions`] if no transitions were configured.
    pub fn build(self) -> Result<Machine, StateError> {
        if self.transitions.is_empty() {
            return Err(StateError::NoTransitions);
        }

        Ok(Machine::new(self.transitions))
    }

    /// Returns the number of transitions added to the state machine.
    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// Returns the number of unique states used by the configured transitions.
    pub fn state_count(&self) -> usize {
        self.transitions.state_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod empty {
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
    }

    mod allow {
        use super::*;

        #[test]
        #[allow(deprecated)]
        fn allow_once_has_two_states() {
            let builder = MachineBuilder::new();
            let builder = builder.allow("start", "finish");

            assert_eq!(builder.state_count(), 2);
        }

        #[test]
        #[allow(deprecated)]
        fn allow_once_has_one_transition() {
            let builder = MachineBuilder::new();
            let builder = builder.allow("start", "finish");

            assert_eq!(builder.transition_count(), 1);
        }

        #[test]
        #[allow(deprecated)]
        #[should_panic(expected = "invalid transition passed to MachineBuilder::allow")]
        fn allow_panics_for_empty_source_state() {
            MachineBuilder::new().allow("", "running");
        }
    }

    mod build {
        use super::*;

        #[test]
        fn build_empty_build_invalid() {
            let builder = MachineBuilder::new();

            assert_eq!(builder.build(), Err(StateError::NoTransitions));
        }

        #[test]
        fn build_allows_cycles() -> Result<(), StateError> {
            let machine = MachineBuilder::new()
                .try_allow("start", "finish")?
                .try_allow("finish", "start")?
                .build();

            assert!(machine.is_ok());

            Ok(())
        }
    }

    mod try_allow {
        use super::*;

        #[test]
        fn try_allow_once_has_two_states() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;

            assert_eq!(builder.state_count(), 2);

            Ok(())
        }

        #[test]
        fn try_allow_once_has_one_transition() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;

            assert_eq!(builder.transition_count(), 1);

            Ok(())
        }

        #[test]
        fn try_allow_twice_same_from_state_has_three_states() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("start", "finish2")?;

            assert_eq!(builder.state_count(), 3);

            Ok(())
        }

        #[test]
        fn try_allow_twice_same_from_state_has_two_transition() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("start", "finish2")?;

            assert_eq!(builder.transition_count(), 2);

            Ok(())
        }

        #[test]
        fn try_allow_twice_same_from_state_same_transition_has_two_states() -> Result<(), StateError>
        {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("start", "finish")?;

            assert_eq!(builder.state_count(), 2);

            Ok(())
        }

        #[test]
        fn try_allow_duplicate_transition_is_ignored() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("start", "finish")?;

            assert_eq!(builder.transition_count(), 1);

            Ok(())
        }

        #[test]
        fn try_allow_twice_different_states_has_four_states() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("start2", "finish2")?;

            assert_eq!(builder.state_count(), 4);

            Ok(())
        }

        #[test]
        fn try_allow_twice_different_states_has_two_transitions() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("start2", "finish2")?;

            assert_eq!(builder.transition_count(), 2);

            Ok(())
        }

        #[test]
        fn try_allow_twice_connected_transitions_have_three_states() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "mid")?;
            let builder = builder.try_allow("mid", "finish")?;

            assert_eq!(builder.state_count(), 3);

            Ok(())
        }

        #[test]
        fn try_allow_twice_connected_transitions_have_two_transitions() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "mid")?;
            let builder = builder.try_allow("mid", "finish")?;

            assert_eq!(builder.transition_count(), 2);

            Ok(())
        }

        #[test]
        fn try_allow_case_sensitive_from_has_three_states() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("Start", "finish")?;

            assert_eq!(builder.state_count(), 3);

            Ok(())
        }

        #[test]
        fn try_allow_case_sensitive_from_has_two_transitions() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("Start", "finish")?;

            assert_eq!(builder.transition_count(), 2);

            Ok(())
        }

        #[test]
        fn try_allow_case_sensitive_to_has_three_states() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("start", "Finish")?;

            assert_eq!(builder.state_count(), 3);

            Ok(())
        }

        #[test]
        fn try_allow_case_sensitive_to_has_two_transitions() -> Result<(), StateError> {
            let builder = MachineBuilder::new();
            let builder = builder.try_allow("start", "finish")?;
            let builder = builder.try_allow("start", "Finish")?;

            assert_eq!(builder.transition_count(), 2);

            Ok(())
        }

        #[test]
        fn try_allow_error_transition_to_self() {
            let builder = MachineBuilder::new();
            let result = builder.try_allow("start", "start");

            assert_eq!(
                result,
                Err(StateError::SelfTransition {
                    state: "start".to_string(),
                })
            );
        }

        #[test]
        fn try_allow_error_for_empty_source_state() {
            let result = MachineBuilder::new().try_allow("", "running");

            assert_eq!(result, Err(StateError::EmptyState));
        }

        #[test]
        fn try_allow_error_for_empty_target_state() {
            let result = MachineBuilder::new().try_allow("running", "");

            assert_eq!(result, Err(StateError::EmptyState));
        }

        #[test]
        fn try_allow_errors_for_ambiguous_target_state() {
            let result = MachineBuilder::new().try_allow("start", "running ");

            assert_eq!(result, Err(StateError::AmbiguousStateName));
        }

        #[test]
        fn try_allow_errors_for_ambiguous_source_state() {
            let result = MachineBuilder::new().try_allow("start ", "running");

            assert_eq!(result, Err(StateError::AmbiguousStateName));
        }
    }
}
