use crate::{StateError, Transition, Transitions};

/// The internally shared immutable state-machine definition.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MachineInner {
    transitions: Transitions,
}

impl MachineInner {
    /// Constructs a machine from validated transitions.
    pub(crate) fn new(transitions: Transitions) -> Self {
        Self { transitions }
    }
    

    /// Returns whether the transition from `from` to `to` is allowed.
    pub(crate) fn can_transition(&self, from: &str, to: &str) -> bool {
        self.transitions.contains(from, to)
    }

    /// Validates that the transition from `from` to `to` is allowed.
    pub(crate) fn validate_transition(&self, from: &str, to: &str) -> Result<(), StateError> {
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
    pub(crate) fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// Returns whether `state` appears as either endpoint of a transition.
    pub(crate) fn contains_state(&self, state: &str) -> bool {
        self.transitions.contains_state(state)
    }

    /// Returns an iterator over states directly reachable from `from`.
    pub(crate) fn targets_from(&self, from: &str) -> impl Iterator<Item = &str> {
        self.transitions.targets_from(from)
    }

    /// Returns an iterator over all source states.
    pub(crate) fn sources(&self) -> impl Iterator<Item = &str> {
        self.transitions.sources()
    }

    /// Returns an iterator over all unique source and target states.
    pub(crate) fn states(&self) -> impl Iterator<Item = &str> {
        self.transitions.states()
    }

    /// Returns an iterator over all the transitions in the state machine.
    pub(crate) fn transitions(&self) -> impl Iterator<Item = &Transition> {
        self.transitions.iter()
    }
    
    ///A state is terminal when it has no outgoing transitions.
    pub fn is_terminal(&self, state: &str) -> bool {
        self.targets_from(state).next().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod validate_transition {
        use super::*;

        #[test]
        fn validate_transition_accepts_configured_transition() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            transitions.add(Transition::try_new("1", "2")?);

            let m = MachineInner::new(transitions);

            assert!(m.validate_transition("start", "finish").is_ok());

            Ok(())
        }

        #[test]
        fn validate_transition_not_exists_returns_invalid_error() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);

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
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            transitions.add(Transition::try_new("finish", "start")?);
            
            let m = MachineInner::new(transitions);
            
            assert!(m.validate_transition("finish", "start").is_ok());

            Ok(())
        }
    }

    mod transition_count {
        use super::*;

        #[test]
        fn transition_count_counts_transitions() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            transitions.add(Transition::try_new("1", "2")?);
            
            let m = MachineInner::new(transitions);

            assert_eq!(m.transition_count(), 2);

            Ok(())
        }
    }

    mod can_transition {
        use super::*;

        #[test]
        fn can_transition_exists_returns_true() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);

            assert!(m.can_transition("start", "finish"));

            Ok(())
        }

        #[test]
        fn can_transition_not_exists_returns_false() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);

            assert!(!m.can_transition("start", "invalid"));

            Ok(())
        }
    }

    mod contains_state {
        use super::*;

        #[test]
        fn contains_state_finds_target_only_state() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);

            assert!(m.contains_state("finish"));

            Ok(())
        }

        #[test]
        fn contains_state_rejects_unknown_state() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);

            assert!(!m.contains_state("other"));

            Ok(())
        }

        #[test]
        fn contains_state_rejects_ambiguous_state() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);

            assert!(!m.contains_state(" start"));

            Ok(())
        }

        #[test]
        fn contains_state_finds_source_state() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "end")?);
            transitions.add(Transition::try_new("rest", "finish")?);
            
            let m = MachineInner::new(transitions);

            assert!(m.contains_state("rest"));

            Ok(())
        }
    }

    mod targets_from {
        use super::*;

        #[test]
        fn targets_from_one_transition_key_does_not_exist_returns_empty() -> Result<(), StateError>
        {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);
            let collected: Vec<_> = m.targets_from("other").collect();

            assert!(collected.is_empty());

            Ok(())
        }

        #[test]
        fn targets_from_one_transition_one_value() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);
            let collected: Vec<_> = m.targets_from("start").collect();

            assert_eq!(collected, vec!["finish"]);

            Ok(())
        }

        #[test]
        fn targets_from_one_source_returns_two_targets() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "1")?);
            transitions.add(Transition::try_new("start", "2")?);
            
            let m = MachineInner::new(transitions);
            let mut collected: Vec<_> = m.targets_from("start").collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn targets_from_one_source_returns_three_targets() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "1")?);
            transitions.add(Transition::try_new("start", "2")?);
            transitions.add(Transition::try_new("start", "3")?);
            
            let m = MachineInner::new(transitions);
            let mut collected: Vec<_> = m.targets_from("start").collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2", "3"]);

            Ok(())
        }

        #[test]
        fn targets_from_target_only_state_returns_empty() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);
            assert!(m.contains_state("finish"));
            assert!(
                m
                    .targets_from("finish")
                    .collect::<Vec<_>>()
                    .is_empty()
            );

            Ok(())
        }

        #[test]
        fn targets_from_duplicate_transition_is_stored_once() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            transitions.add(Transition::try_new("start", "finish")?);
            
            let m = MachineInner::new(transitions);
            assert_eq!(m.transition_count(), 1);

            let targets: Vec<_> = m.targets_from("start").collect();
            assert_eq!(targets, vec!["finish"]);

            Ok(())
        }
    }

    mod sources {
        use super::*;

        #[test]
        fn sources_one_source_one_value() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("start", "finish")?);
            
            let machine = MachineInner::new(transitions);

            let sources: Vec<_> = machine.sources().collect();

            assert_eq!(sources, vec!["start"]);

            Ok(())
        }

        #[test]
        fn returns_all_source_states() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "0")?);
            transitions.add(Transition::try_new("2", "0")?);
            transitions.add(Transition::try_new("3", "0")?);
            
            let machine = MachineInner::new(transitions);

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
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            
            let machine = MachineInner::new(transitions);

            let mut states: Vec<_> = machine.states().collect();
            states.sort();

            assert_eq!(states, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn returns_unique_source_and_target_states() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            transitions.add(Transition::try_new("1", "3")?);
            transitions.add(Transition::try_new("2", "3")?);
            transitions.add(Transition::try_new("3", "4")?);
            
            let machine = MachineInner::new(transitions);

            let mut states: Vec<_> = machine.states().collect();
            states.sort();

            assert_eq!(states, vec!["1", "2", "3", "4"]);

            Ok(())
        }

        #[test]
        fn includes_target_only_states() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("queued", "running")?);
            
            let machine = MachineInner::new(transitions);

            let mut states: Vec<_> = machine.states().collect();
            states.sort();

            assert_eq!(states, vec!["queued", "running"]);

            Ok(())
        }
    }

    mod transitions {
        use super::*;

        #[test]
        fn one_transition_returns_matching_fields() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            
            let machine = MachineInner::new(transitions);

            let transitions2: Vec<_> = machine.transitions().collect();

            assert_eq!(transitions2[0].source(), "1");
            assert_eq!(transitions2[0].target(), "2");

            Ok(())
        }

        #[test]
        fn one_transition_returns_one_item() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            
            let machine = MachineInner::new(transitions);

            let transitions2: Vec<_> = machine.transitions().collect();

            assert_eq!(transitions2.len(), 1);

            Ok(())
        }

        #[test]
        fn multiple_transitions_returns_correct_count() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            transitions.add(Transition::try_new("2", "3")?);
            transitions.add(Transition::try_new("2", "1")?);
            transitions.add(Transition::try_new("5", "2")?);
            
            let machine = MachineInner::new(transitions);

            let transitions2: Vec<_> = machine.transitions().collect();

            assert_eq!(transitions2.len(), 4);

            Ok(())
        }
    }
    
    mod is_terminal {
        use super::*;
        
        #[test]
        fn one_transition_target_returns_true() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            
            let machine = MachineInner::new(transitions);

            assert!(machine.is_terminal("2"));

            Ok(())
        }
        
        #[test]
        fn one_transition_source_returns_false() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            
            let machine = MachineInner::new(transitions);

            assert!(!machine.is_terminal("1"));

            Ok(())
        }
        
        #[test]
        fn multiple_transitions_terminal_returns_true() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            transitions.add(Transition::try_new("2", "3")?);
            
            let machine = MachineInner::new(transitions);

            assert!(machine.is_terminal("3"));

            Ok(())
        }
        
        #[test]
        fn multiple_transitions_connected_returns_false() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);
            transitions.add(Transition::try_new("2", "3")?);
            
            let machine = MachineInner::new(transitions);

            assert!(!machine.is_terminal("2"));

            Ok(())
        }
    }

    mod partial_eq {
        use super::*;

        #[test]
        fn similar_machines_are_equal() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);

            let machine1 = MachineInner::new(transitions);

            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);

            let machine2 = MachineInner::new(transitions);

            assert_eq!(machine1, machine2);

            Ok(())
        }

        #[test]
        fn different_machines_are_not_equal() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);

            let machine1 = MachineInner::new(transitions);

            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("2", "1")?);

            let machine2 = MachineInner::new(transitions);

            assert_ne!(machine1, machine2);

            Ok(())
        }
    }    
}
