use std::collections::{HashMap, HashSet};

use crate::{Machine, StateError, StateName};

/// A builder for constructing a [`Machine`].
///
/// Transitions are added with [`MachineBuilder::allow`] and validated when
/// [`MachineBuilder::build`] is called.
///
/// State names are case-sensitive. Empty state names and self-transitions are
/// rejected. Cycles are allowed.
#[derive(Debug, PartialEq, Default)]
pub struct MachineBuilder {
    transitions: HashMap<String, HashSet<String>>,
}

impl MachineBuilder {
    /// Creates an empty builder.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds an allowed transition from one state to another.
    ///
    /// Adding the same transition more than once has no additional effect.
    pub fn allow(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.transitions
            .entry(from.into())
            .or_default()
            .insert(to.into());

        self
    }

    /// Validates the configured transitions and builds an immutable [`Machine`].
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - [`StateError::NoTransitions`] if no transitions were configured.
    /// - [`StateError::EmptyState`] if an endpoint is empty.
    /// - [`StateError::SelfTransition`] if a transition has identical endpoints.
    pub fn build(self) -> Result<Machine, StateError> {
        if self.transitions.is_empty() {
            return Err(StateError::NoTransitions);
        }

        for (from, targets) in &self.transitions {
            if from.is_empty() || targets.contains("") {
                return Err(StateError::EmptyState);
            }

            if targets.contains(from) {
                return Err(StateError::SelfTransition {
                    state: from.clone(),
                });
            }
        }

        let transitions = self
            .transitions
            .into_iter()
            .map(|(from, targets)| (StateName::from(from), targets))
            .collect();

        Ok(Machine::new(transitions))
    }

    /// Returns the number of transitions added to the state machine.
    pub fn transition_count(&self) -> usize {
        self.transitions.values().map(HashSet::len).sum()
    }

    /// Returns the number of unique states used by the configured transitions.
    pub fn state_count(&self) -> usize {
        let mut unique: HashSet<&str> = HashSet::new();

        for (from, targets) in &self.transitions {
            unique.insert(from.as_str());
            unique.extend(targets.iter().map(String::as_str));
        }

        unique.len()
    }
}

#[cfg(test)]
mod tests {
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
    fn build_allows_cycles() {
        let machine = MachineBuilder::new()
            .allow("start", "finish")
            .allow("finish", "start")
            .build();

        assert!(machine.is_ok());
    }
}
