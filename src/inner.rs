use crate::{MachineBuilder, StateError, StateRegistry, Transition, Transitions};

/// The internally shared immutable state-machine definition.
#[derive(Debug,  Eq)]
pub(crate) struct MachineInner {
    states: StateRegistry,
    transitions: Transitions,
}

impl MachineInner {
    /// Constructs a machine from validated transitions.
    pub(crate) fn new(
        states: StateRegistry,
        transitions: Transitions,
    ) -> Self {
        Self { states, transitions }
    }
    
    /// Returns the state from the string literal representation
    pub(crate) fn state_of_name(&self, name: &str) -> Option<&StateName> {
        self.states.state_of(self.states.id_of_name(name))
    }

    /// Returns whether the transition from `from` to `to` is allowed.
    pub(crate) fn can_transition(&self, from: &str, to: &str) -> bool {
        let Some(source) = self.states.id_of_name(from) else {
            return false;
        }

        let Some(target) = self.states.id_of_name(to) else {
            return false;
        }
        
        self.transitions.contains(source, target);
    }

    /// Validates that the transition from `from` to `to` is allowed.
    pub(crate) fn validate_transition(&self, from: &StateName, to: &StateName) -> Result<(), StateError> {
        if self.can_transition(from, to) {
            Ok(())
        } else {
            Err(StateError::InvalidTransition {
                from: from.as_str(),
                to: to.as_str(),
            })
        }
    }

    /// Returns the number of transitions in the state machine.
    pub(crate) fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// Returns the number of states in the state machine.
    pub(crate) fn state_count(&self) -> usize {
        self.states.len()
    }

    /// Returns whether `state` appears as either endpoint of a transition.
    pub(crate) fn contains_state(&self, state: &str) -> bool {
        self.states.id_of_name(state) != None
    }

    /// Returns an iterator over states directly reachable from `from`.
    pub(crate) fn targets_from(&self, from: &str) -> impl Iterator<Item = &str> {
        if let Some(source_id) = self.states.id_of_name(from) {
            return self.transitions.targets_from(source_id)
                .map(|id| self.safe_resolve_id(id))
        }
        
        std::iter::empty::<&str>()
    }

    /// Returns an iterator over all source states.
    pub(crate) fn sources(&self) -> impl Iterator<Item = &str> {
        self.transitions.sources()
            .map(|id| self.safe_resolve_id(id))
    }

    /// Returns an iterator over all unique source and target states.
    pub(crate) fn states(&self) -> impl Iterator<Item = &str> {
        self.states.states()
            .map(|state_name| state_name.as_str())
    }

    /// Returns an iterator over all the transitions in the state machine.
    pub(crate) fn transitions(&self) -> impl Iterator<Item = &Transition> {
        self.transitions.iter()
            .map(|(sid, tid)| Transition::new( 
                self.states.stateof(sid).unwrap(),
                self.states.stateof(tid).unwrap(),
            ))
    }

    ///A state is terminal when it is part of the machine and it has no outgoing transitions.
    pub(crate) fn is_terminal(&self, state: &str) -> bool {
        if let Some(id) = self.states.id_of_name(state) {
            return self.transitions.targets_from(id).next().is_none();
        }
        
        false
    }

    /// Returns whether the machine empty or not.
    pub(crate) fn is_empty(&self) -> bool {
        self.states.len() == 0 && self.transitions.len() == 0
    }

    /// Returns whether a state is not part of a transition
    pub(crate) fn is_isolated(&self, state: &str) -> bool {
        if let Some(id) = self.states.id_of_name(state) {
            return !self.transitions.iter().any(|&(s,t)| s == id || t == id)
        }
        
        false
    }

    /// Convience wrapper function to get the state, unwrap, and convert it to &str
    fn safe_resolve_id(&self, id: usize) -> &str {
        self.states.state_of(id).unwrap().as_str()
    }
}

impl PartialEq for MachineInner {
    fn eq(&self, other: &Self) -> bool {
        self.states == other.states &&
            self.transitions.iter().map(|&(s1,t1)| 
                (self.states.state_of(s1).unwrap(), self.states.state_of(t1).unwrap()))
            .eq(
                other.transitions.iter().map(|&(s2,t2)| 
                    (other.states.state_of(s2).unwrap(), other.states.state_of(t2).unwrap()))
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod validate_transition {
        use super::*;

        #[test]
        fn validate_transition_accepts_configured_transition() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .try_allow("1", "2")?
                .build_inner()?;

            assert!(m.validate_transition("start", "finish").is_ok());

            Ok(())
        }

        #[test]
        fn validate_transition_accepts_self_transition() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "1")?
                .build_inner()?;

            assert!(m.validate_transition("1", "1").is_ok());

            Ok(())
        }

        #[test]
        fn validate_transition_not_exists_returns_invalid_error() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

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
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .try_allow("finish", "start")?
                .build_inner()?;

            assert!(m.validate_transition("finish", "start").is_ok());

            Ok(())
        }
    }

    mod transition_count {
        use super::*;

        #[test]
        fn transition_count_counts_transitions() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .try_allow("1", "2")?
                .build_inner()?;

            assert_eq!(m.transition_count(), 2);

            Ok(())
        }

        #[test]
        fn transition_count_counts_self_transitions() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "1")?
                .build_inner()?;

            assert_eq!(m.transition_count(), 1);

            Ok(())
        }

        #[test]
        fn transition_count_empty_returns_zero() {
            let m = MachineBuilder::new().build_inner()?;
            assert_eq!(m.transition_count(), 0);
        }
    }

    mod can_transition {
        use super::*;

        #[test]
        fn can_transition_exists_returns_true() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            assert!(m.can_transition("start", "finish"));

            Ok(())
        }

        #[test]
        fn can_transition_self_transition_returns_true() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "start")?
                .build_inner()?;

            assert!(m.can_transition("start", "start"));

            Ok(())
        }

        #[test]
        fn can_transition_not_exists_returns_false() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            assert!(!m.can_transition("start", "invalid"));

            Ok(())
        }
    }

    mod contains_state {
        use super::*;

        #[test]
        fn contains_state_finds_target_only_state() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            assert!(m.contains_state("finish"));

            Ok(())
        }

        #[test]
        fn contains_state_rejects_unknown_state() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            assert!(!m.contains_state("other"));

            Ok(())
        }

        #[test]
        fn contains_state_rejects_ambiguous_state() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            let m = MachineInner::new(transitions);

            assert!(!m.contains_state(" start"));

            Ok(())
        }

        #[test]
        fn contains_state_finds_source_state() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "end")?
                .try_allow("rest", "finish")?
                .build_inner()?;

            assert!(m.contains_state("rest"));

            Ok(())
        }
    }

    mod targets_from {
        use super::*;

        #[test]
        fn targets_from_one_transition_key_does_not_exist_returns_empty() -> Result<(), StateError>
        {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            let collected: Vec<_> = m.targets_from("other").collect();

            assert!(collected.is_empty());

            Ok(())
        }

        #[test]
        fn targets_from_one_transition_one_value() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            let collected: Vec<_> = m.targets_from("start").collect();

            assert_eq!(collected, vec!["finish"]);

            Ok(())
        }

        #[test]
        fn targets_from_one_source_returns_two_targets() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "1")?
                .try_allow("start", "2")?
                .build_inner()?;

            let mut collected: Vec<_> = m.targets_from("start").collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn targets_from_one_source_returns_three_targets() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "1")?
                .try_allow("start", "2")?
                .try_allow("start", "3")?
                .build_inner()?;

            let mut collected: Vec<_> = m.targets_from("start").collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2", "3"]);

            Ok(())
        }

        #[test]
        fn targets_from_target_only_state_returns_empty() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            assert!(m.contains_state("finish"));
            assert!(m.targets_from("finish").collect::<Vec<_>>().is_empty());

            Ok(())
        }

        #[test]
        fn targets_from_duplicate_transition_is_stored_once() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .try_allow("start", "finish")?
                .build_inner()?;

            assert_eq!(m.transition_count(), 1);

            let targets: Vec<_> = m.targets_from("start").collect();
            assert_eq!(targets, vec!["finish"]);

            Ok(())
        }

        #[test]
        fn targets_from_self_transition_returns_itself() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "start")?
                .build_inner()?;

            let targets: Vec<_> = m.targets_from("start").collect();

            assert_eq!(targets, vec!["start"]);

            Ok(())
        }

        #[test]
        fn targets_from_empty_returns_zero() {
            let m = MachineBuilder::new().build_inner()?;
            let targets: Vec<_> = m.targets_from("start").collect();
            assert!(targets.is_empty());
        }
    }

    mod sources {
        use super::*;

        #[test]
        fn sources_one_source_one_value() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "finish")?
                .build_inner()?;

            let sources: Vec<_> = m.sources().collect();

            assert_eq!(sources, vec!["start"]);

            Ok(())
        }

        #[test]
        fn sources_one_self_transition_one_value() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("start", "start")?
                .build_inner()?;

            let sources: Vec<_> = m.sources().collect();

            assert_eq!(sources, vec!["start"]);

            Ok(())
        }

        #[test]
        fn returns_all_source_states() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_new("1", "0")?
                .try_new("2", "0")?
                .try_new("3", "0")?
                .build_inner()?;

            let mut sources: Vec<_> = m.sources().collect();
            sources.sort();

            assert_eq!(sources, vec!["1", "2", "3"]);

            Ok(())
        }

        #[test]
        fn sources_empty_returns_zero() {
            let m = MachineBuilder::new().build_inner()?;
            let sources: Vec<_> = m.sources().collect();
            assert!(sources.is_empty());
        }
    }

    mod states {
        use super::*;

        #[test]
        fn states_one_transition_returns_2_values() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            let mut states: Vec<_> = m.states().collect();
            states.sort();

            assert_eq!(states, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn states_self_transition_returns_1_values() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "1")?
                .build_inner()?;

            let states: Vec<_> = m.states().collect();

            assert_eq!(states, vec!["1"]);

            Ok(())
        }

        #[test]
        fn returns_unique_source_and_target_states() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .try_allow("1", "3")?
                .try_allow("2", "3")?
                .try_allow("3", "4")?
                .build_inner()?;

            let mut states: Vec<_> = m.states().collect();
            states.sort();

            assert_eq!(states, vec!["1", "2", "3", "4"]);

            Ok(())
        }

        #[test]
        fn includes_target_only_states() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("queued", "running")?
                .build_inner()?;

            let mut states: Vec<_> = m.states().collect();
            states.sort();

            assert_eq!(states, vec!["queued", "running"]);

            Ok(())
        }

        #[test]
        fn states_empty_returns_zero() {
            let m = MachineBuilder::new().build_inner()?;
            let states: Vec<_> = m.states().collect();
            assert!(states.is_empty());
        }
    }

    mod transitions {
        use super::*;

        #[test]
        fn one_transition_returns_matching_fields() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            let transitions2: Vec<_> = m.transitions().collect();

            assert_eq!(transitions2[0].source(), "1");
            assert_eq!(transitions2[0].target(), "2");

            Ok(())
        }

        #[test]
        fn one_transition_returns_one_item() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            let transitions2: Vec<_> = m.transitions().collect();

            assert_eq!(transitions2.len(), 1);

            Ok(())
        }

        #[test]
        fn multiple_transitions_returns_correct_count() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .try_allow("2", "3")?
                .try_allow("2", "1")?
                .try_allow("5", "2")?
                .build_inner()?;

            let transitions2: Vec<_> = m.transitions().collect();

            assert_eq!(transitions2.len(), 4);

            Ok(())
        }

        #[test]
        fn transitions_empty_returns_zero() {
            let m = MachineBuilder::new().build_inner()?;
            let t: Vec<_> = m.transitions().collect();
            assert!(t.is_empty());
        }
    }

    mod is_terminal {
        use super::*;

        #[test]
        fn one_transition_target_returns_true() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            assert!(m.is_terminal("2"));

            Ok(())
        }

        #[test]
        fn one_transition_source_returns_false() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            assert!(!m.is_terminal("1"));

            Ok(())
        }

        #[test]
        fn unknown_state_returns_false() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            assert!(!m.is_terminal("unknown"));

            Ok(())
        }

        #[test]
        fn self_transition_returns_false() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "1")?
                .build_inner()?;

            assert!(!m.is_terminal("1"));

            Ok(())
        }

        #[test]
        fn multiple_transitions_terminal_returns_true() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .try_allow("2", "3")?
                .build_inner()?;

            assert!(m.is_terminal("3"));

            Ok(())
        }

        #[test]
        fn multiple_transitions_connected_returns_false() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("1", "2")?
                .try_allow("2", "3")?
                .build_inner()?;

            assert!(!m.is_terminal("2"));

            Ok(())
        }
    }

    mod partial_eq {
        use super::*;

        #[test]
        fn similar_machines_are_equal() -> Result<(), StateError> {
            let machine1 = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            let machine2 = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            assert_eq!(machine1, machine2);

            Ok(())
        }

        #[test]
        fn machines_with_different_sizes_are_not_equal() -> Result<(), StateError> {
            let machine1 = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            let machine2 = MachineBuilder::new()
                .try_allow("1", "2")?
                .try_allow("1", "3")?
                .build_inner()?;

            assert_ne!(machine1, machine2);

            Ok(())
        }

        #[test]
        fn similar_machines_in_different_order_are_equal() -> Result<(), StateError> {
            let machine1 = MachineBuilder::new()
                .try_allow("1", "2")?
                .try_allow("3", "4")?
                .build_inner()?;

            let machine2 = MachineBuilder::new()
                .try_allow("3", "4")?
                .try_allow("1", "2")?
                .build_inner()?;

            assert_eq!(machine1, machine2);

            Ok(())
        }

        #[test]
        fn different_machines_are_not_equal() -> Result<(), StateError> {
            let machine1 = MachineBuilder::new()
                .try_allow("1", "2")?
                .build_inner()?;

            let machine2 = MachineBuilder::new()
                .try_allow("2", "1")?
                .build_inner()?;

            assert_ne!(machine1, machine2);

            Ok(())
        }

        #[test]
        fn empty_machines_are_equal() {
            let m1 = MachineBuilder::new().build_inner()?;
            let m2 = MachineBuilder::new().build_inner()?;
            assert_eq!(m1, m2);
        }
    }

    mod is_empty {
        use super::*;

        #[test]
        fn is_empty_no_items() {
            let m = MachineBuilder::new().build_inner()?;

            assert!(m.is_empty());
        }

        #[test]
        fn is_empty_items() -> Result<(), StateError> {
            let m = MachineBuilder::new()
                .try_allow("2", "1")?
                .build_inner()?;

            assert!(!m.is_empty());

            Ok(())
        }
    }
}
