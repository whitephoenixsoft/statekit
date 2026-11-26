use std::collections::HashSet;

use crate::Transition;

/// A collection of transition ids.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Transitions {
    edges: HashSet<(usize, usize)>,
}

impl Transitions {
    /// Constructs a collection to hold transitions.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds a source and target ids as a transition.
    pub(crate) fn insert(&mut self, source: usize, target: usize) {
        self.edges.insert((source, target));
    }

    /// Returns an iterator over the transitions.
    pub(crate) fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.edges.iter()
    }

    /// Returns the count of transactions
    pub(crate) fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether a transition exists based on `source` and `target`.
    pub(crate) fn contains(&self, source: &usize, target: &usize) -> bool {
        self.edges.contains((source, target))
    }

    /// Returns an iterator over the states directly reachable from `source`.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn targets_from(&self, source: &usize) -> impl Iterator<Item = &usize> {
        self.edges
            .iter()
            .filter(move |(s,_)| *s == source)
            .map(|(_,t)| *t)
    }

    /// Returns an iterator over all source states.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn sources(&self) -> impl Iterator<Item = &usize> {
        let mut unique: HashSet<&usize> = HashSet::new();

        for (source_id, _) in &self.edges {
            unique.insert(source_id);
        }

        unique.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StateError;
    
    mod len {
        use super::*;

        #[test]
        fn returns_0_on_no_items() {
            let transitions = Transitions::new();

            assert_eq!(transitions.len(), 0);
        }

        #[test]
        fn returns_number_of_transitions() {
            let mut transitions = Transitions::new();

            transitions.insert(0, 0);

            assert_eq!(transitions.len(), 1);
        }

        #[test]
        fn duplicate_transition_is_stored_once() {
            let mut items = Transitions::new();

            items.insert(0, 0);
            items.insert(0, 0);
            
            assert_eq!(items.len(), 1);
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn returns_empty_on_no_items() {
            let items = Transitions::new();

            let transitions: Vec<_> = items.iter().collect();

            assert!(transitions.is_empty());
        }

        #[test]
        fn one_transition_returns_matching_fields() {
            let mut items = Transitions::new();
            items.insert(1, 2);

            let transitions: Vec<_> = items.iter().collect();

            assert_eq!(transitions[0], (1, 2));
        }

        #[test]
        fn one_transition_returns_one_item() {
            let mut items = Transitions::new();

            items.insert(1, 2);

            let transitions: Vec<_> = items.iter().collect();

            assert_eq!(transitions.len(), 1);
        }

        #[test]
        fn multiple_transitions_returns_correct_count() {
            let mut items = Transitions::new();

            items.insert(1, 2);
            items.insert(2, 3);
            items.insert(2, 1);
            items.insert(5, 2);

            let transitions: Vec<_> = items.iter().collect();

            assert_eq!(transitions.len(), 4);
        }
    }
    
    mod contains {
        use super::*;

        #[test]
        fn contains_exists_returns_true() {
            let mut items = Transitions::new();

            items.insert(1, 2);

            assert!(items.contains(1, 2));
        }

        #[test]
        fn contains_not_exists_returns_false() {
            let mut items = Transitions::new();

            items.insert(1, 2);

            assert!(!items.contains(1, 3));
        }

        #[test]
        fn contains_no_items_returns_false() {
            let items = Transitions::new();

            assert!(!items.contains(1, 2));
        }

        #[test]
        fn contains_is_directional() {
            let mut items = Transitions::new();

            items.insert(1, 2);

            assert!(items.contains(1, 2));
            assert!(!items.contains(2, 1));
        }
    }
    
    mod targets_from {
        use super::*;

        #[test]
        fn targets_from_no_items_returns_empty() {
            let items = Transitions::new();

            let collected: Vec<_> = items.targets_from(1).collect();

            assert!(collected.is_empty());
        }

        #[test]
        fn targets_from_source_does_not_exist_returns_empty() {
            let mut items = Transitions::new();

            items.insert(1, 2);

            let collected: Vec<_> = items.targets_from(3).collect();

            assert!(collected.is_empty());
        }

        #[test]
        fn targets_from_one_source_with_one_target() {
            let mut items = Transitions::new();

            items.insert(1, 2);

            let collected: Vec<_> = items.targets_from(1).collect();

            assert_eq!(collected, vec![2]);
        }

        #[test]
        fn targets_from_target_only_returns_empty() {
            let mut items = Transitions::new();

            items.insert(1, 2);

            let collected: Vec<_> = items.targets_from(2).collect();
            
            assert!(collected.is_empty());
        }

        #[test]
        fn targets_from_one_source_multiple_targets() {
            let mut items = Transitions::new();

            items.insert(1, 2);
            items.insert(1, 3);
            items.insert(1, 4);

            let mut collected: Vec<_> = items.targets_from(1).collect();
            collected.sort();

            assert_eq!(collected, vec![2, 3, 4]);
        }

        #[test]
        fn targets_from_duplicate_transition_is_stored_once() {
            let mut items = Transitions::new();

            items.insert(1, 2);
            items.insert(1, 2);

            let collected: Vec<_> = items.targets_from(1).collect();

            assert_eq!(collected, vec![2]);
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
        fn sources_one_source_one_value() {
            let mut items = Transitions::new();

            items.insert(1, 2);

            let collected: Vec<_> = items.sources().collect();

            assert_eq!(collected, vec![1]);
        }

        #[test]
        fn sources_returns_all_source_states() {
            let mut items = Transitions::new();

            items.insert(1, 0);
            items.insert(4, 0);
            items.insert(2, 0);
            items.insert(3, 0);
            items.insert(4, 1);

            let mut collected: Vec<_> = items.sources().collect();
            collected.sort();

            assert_eq!(collected, vec![1, 2, 3, 4]);
        }
    }
 
    mod equality {
        use super::*;

        #[test]
        fn empty_transitions_are_equal() {
            let items1 = Transitions::new();

            let items2 = Transitions::new();

            assert_eq!(items1, items2);
        }
        
        #[test]
        fn similar_transitions_are_equal() {
            let mut items1 = Transitions::new();
            
            items1.insert(1, 2);

            let mut items2 = Transitions::new();
            
            items2.insert(1, 2);

            assert_eq!(items1, items2);
        }

        #[test]
        fn different_transitions_are_not_equal() {
            let mut items1 = Transitions::new();
            
            items1.insert(1, 2);

            let mut items2 = Transitions::new();
            
            items2.insert(2, 1);

            assert_ne!(items1, items2);
        }
        
        #[test]
        fn different_length_transitions_are_not_equal() {
            let mut items1 = Transitions::new();
            
            items1.insert(1, 2);

            let mut items2 = Transitions::new();
            
            items1.insert(1, 2);
            items2.insert(2, 1);

            assert_ne!(items1, items2);
        }
    }
}
