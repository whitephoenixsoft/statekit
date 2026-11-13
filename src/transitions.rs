use std::collections::HashSet;

use crate::Transition;

/// A collection of states and transitions.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Transitions {
    states: Vec<StateName>,
    edges: HashSet<(usize, usize)>,
}

impl Transitions {
    /// Constructs a collection to hold transitions.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Resolve the state id.
    pub(crate) fn state_id(&self, name: &StateName) -> Option<usize> {
        self.states.iter().position(|s| s == name)
    }

    /// Insert a state atomically.
    pub(crate) get_or_insert_state(&mut self, name: StateName) -> usize {
        if let Some(id) = self.state_id(name) {
            return id;
        }

        let id = self.states.len();
        self.states.push(state);
        id
    }

    /// Adds a source and target ids as a transition.
    pub(crate) fn add_transaction(&mut self, source: usize, target: usize) {
        self.edges.insert((source, target));
    }

    /// Returns an iterator over the transitions.
    pub(crate) fn transactions(&self) -> impl Iterator<Item = &Transition> {
        self.edges.iter()
            .map(|(s, t)| Transition::new(self.states[s].clone(), self.states[t].clone()))
    }

    /// Returns the count of transactions
    pub(crate) fn transaction_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns the count of the unique source and target states in
    /// the collection.
    pub(crate) fn state_count(&self) -> usize {
        self.states.len()
    }

    /// Returns whether a transition exists based on `source` and `target`.
    pub(crate) fn contains(&self, source: &StateName, target: &StateName) -> bool {
        let source_id = self.state_id(source);
        let target_id = self.state_id(target);
        
        if source_id != None && target_id != None {
            self.edges.contains((source_id.unwrap(), target_id.unwrap())
        }
        
        false
    }

    /// Returns whether `state` is a reserved state or 
    pub(crate) fn contains_state(&self, state: &StateName) -> bool {
        self.states.contains(state)
    }

    /// Returns an iterator over the states directly reachable from `source`.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn targets_from(&self, source: &StateName) -> impl Iterator<Item = &StateName> {
        self.edges
            .iter()
            .filter(move |(s,_)| self.states[s] == source)
            .map(|(_,t)| self.states[t])
    }

    /// Returns an iterator over all source states.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn sources(&self) -> impl Iterator<Item = &StateName> {
        let mut unique: HashSet<&StateName> = HashSet::new();

        for (source_id, _) in &self.edges {
            unique.insert(self.states[source_id]);
        }

        unique.into_iter()
    }

    /// Returns an iterator over all unique states.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn states(&self) -> impl Iterator<Item = &StateName> {
        self.states.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StateError;

    mod len {
        use super::*;

        #[test]
        fn len_returns_0_on_no_items() {
            let transitions = Transitions::new();

            assert_eq!(transitions.len(), 0);
        }

        #[test]
        fn len_returns_number_of_transitions() -> Result<(), StateError> {
            let mut transitions = Transitions::new();

            transitions.add(Transition::try_new("1", "2")?);
            transitions.add(Transition::try_new("3", "4")?);
            transitions.add(Transition::try_new("5", "6")?);

            assert_eq!(transitions.len(), 3);

            Ok(())
        }

        #[test]
        fn duplicate_transition_is_stored_once() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("1", "2")?);

            assert_eq!(items.len(), 1);

            Ok(())
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn iter_returns_empty_on_no_items() {
            let items = Transitions::new();

            let transitions: Vec<_> = items.iter().collect();

            assert!(transitions.is_empty());
        }

        #[test]
        fn iter_one_transition_returns_matching_fields() -> Result<(), StateError> {
            let mut items = Transitions::new();
            items.add(Transition::try_new("1", "2")?);

            let transitions: Vec<_> = items.iter().collect();

            assert_eq!(transitions[0].source(), "1");
            assert_eq!(transitions[0].target(), "2");

            Ok(())
        }

        #[test]
        fn iter_one_transition_returns_one_item() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            let transitions: Vec<_> = items.iter().collect();

            assert_eq!(transitions.len(), 1);

            Ok(())
        }

        #[test]
        fn iter_multiple_transitions_returns_correct_count() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("2", "3")?);
            items.add(Transition::try_new("2", "1")?);
            items.add(Transition::try_new("5", "2")?);

            let transitions: Vec<_> = items.iter().collect();

            assert_eq!(transitions.len(), 4);

            Ok(())
        }
    }

    mod state_count {
        use super::*;

        #[test]
        fn state_count_no_items_returns_0() {
            let items = Transitions::new();

            assert_eq!(items.state_count(), 0);
        }

        #[test]
        fn state_count_one_transition_returns_2_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            assert_eq!(items.state_count(), 2);

            Ok(())
        }

        #[test]
        fn state_count_two_shared_source_transitions_returns_3_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("1", "3")?);

            assert_eq!(items.state_count(), 3);

            Ok(())
        }

        #[test]
        fn state_count_same_transition_twice_returns_2_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("1", "2")?);

            assert_eq!(items.state_count(), 2);

            Ok(())
        }

        #[test]
        fn state_count_two_connected_transitions_returns_3_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("2", "3")?);

            assert_eq!(items.state_count(), 3);

            Ok(())
        }

        #[test]
        fn state_count_two_shared_terminal_transitions_returns_3_states() -> Result<(), StateError>
        {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("4", "2")?);

            assert_eq!(items.state_count(), 3);

            Ok(())
        }

        #[test]
        fn state_count_two_different_transitions_returns_4_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("4", "3")?);

            assert_eq!(items.state_count(), 4);

            Ok(())
        }
    }

    mod contains {
        use super::*;

        #[test]
        fn contains_exists_returns_true() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            assert!(items.contains("1", "2"));

            Ok(())
        }

        #[test]
        fn contains_not_exists_returns_false() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            assert!(!items.contains("1", "3"));

            Ok(())
        }

        #[test]
        fn contains_no_items_returns_false() {
            let items = Transitions::new();

            assert!(!items.contains("1", "2"));
        }

        #[test]
        fn contains_is_directional() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            assert!(items.contains("1", "2"));
            assert!(!items.contains("2", "1"));

            Ok(())
        }
    }

    mod contains_state {
        use super::*;

        #[test]
        fn contains_state_no_items_returns_false() {
            let items = Transitions::new();

            assert!(!items.contains_state("1"));
        }

        #[test]
        fn contains_state_finds_target_only_state() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            assert!(items.contains_state("2"));

            Ok(())
        }

        #[test]
        fn contains_state_finds_source_state() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            assert!(items.contains_state("1"));

            Ok(())
        }

        #[test]
        fn contains_state_rejects_unknown_state() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            assert!(!items.contains_state("3"));

            Ok(())
        }
    }

    mod targets_from {
        use super::*;

        #[test]
        fn targets_from_no_items_returns_empty() {
            let items = Transitions::new();

            let collected: Vec<_> = items.targets_from("1").collect();

            assert!(collected.is_empty());
        }

        #[test]
        fn targets_from_source_does_not_exist_returns_empty() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            let collected: Vec<_> = items.targets_from("3").collect();

            assert!(collected.is_empty());

            Ok(())
        }

        #[test]
        fn targets_from_one_source_with_one_target() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            let collected: Vec<_> = items.targets_from("1").collect();

            assert_eq!(collected, vec!["2"]);

            Ok(())
        }

        #[test]
        fn targets_from_target_only_returns_empty() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            let collected: Vec<_> = items.targets_from("2").collect();

            assert!(items.contains_state("2"));
            assert!(collected.is_empty());

            Ok(())
        }

        #[test]
        fn targets_from_one_source_multiple_targets() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("1", "3")?);
            items.add(Transition::try_new("1", "4")?);

            let mut collected: Vec<_> = items.targets_from("1").collect();
            collected.sort();

            assert_eq!(collected, vec!["2", "3", "4"]);

            Ok(())
        }

        #[test]
        fn targets_from_duplicate_transition_is_stored_once() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("1", "2")?);

            let collected: Vec<_> = items.targets_from("1").collect();

            assert_eq!(collected, vec!["2"]);

            Ok(())
        }
    }

    mod sources {
        use super::*;

        #[test]
        fn sources_no_items_returns_empty() {
            let items = Transitions::new();

            let collected: Vec<_> = items.sources().collect();

            assert!(collected.is_empty());
        }

        #[test]
        fn sources_one_source_one_value() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            let collected: Vec<_> = items.sources().collect();

            assert_eq!(collected, vec!["1"]);

            Ok(())
        }

        #[test]
        fn sources_returns_all_source_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "0")?);
            items.add(Transition::try_new("4", "0")?);
            items.add(Transition::try_new("2", "0")?);
            items.add(Transition::try_new("3", "0")?);
            items.add(Transition::try_new("4", "1")?);

            let mut collected: Vec<_> = items.sources().collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2", "3", "4"]);

            Ok(())
        }
    }

    mod states {
        use super::*;

        #[test]
        fn states_no_items_returns_empty() {
            let items = Transitions::new();

            let collected: Vec<_> = items.states().collect();

            assert!(collected.is_empty());
        }

        #[test]
        fn states_one_transition_returns_2_values() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            let mut collected: Vec<_> = items.states().collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn returns_unique_source_and_target_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);
            items.add(Transition::try_new("1", "3")?);
            items.add(Transition::try_new("2", "3")?);
            items.add(Transition::try_new("3", "4")?);

            let mut collected: Vec<_> = items.states().collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2", "3", "4"]);

            Ok(())
        }

        /// Repeats the target-only-state invariant explicitly for documentation.
        #[test]
        fn includes_target_only_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1", "2")?);

            let mut collected: Vec<_> = items.states().collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2"]);

            Ok(())
        }
    }

    mod equality {
        use super::*;

        #[test]
        fn similar_transitions_are_equal() -> Result<(), StateError> {
            let mut items1 = Transitions::new();
            items1.add(Transition::try_new("1", "2")?);

            let mut items2 = Transitions::new();
            items2.add(Transition::try_new("1", "2")?);

            assert_eq!(items1, items2);

            Ok(())
        }

        #[test]
        fn different_transitions_are_not_equal() -> Result<(), StateError> {
            let mut items1 = Transitions::new();
            items1.add(Transition::try_new("1", "2")?);

            let mut items2 = Transitions::new();
            items2.add(Transition::try_new("2", "4")?);

            assert_ne!(items1, items2);

            Ok(())
        }
    }
}
