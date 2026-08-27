use std::collections::HashSet;

use crate::Transition;

/// A collection of Transition items.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Transitions {
    items: HashSet<Transition>,
}

impl Transitions {
    /// Constructs a collection to hold transitions.
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
         Self::default()
     }

    /// Returns the length of the collection.
    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the collection is empty or not.
    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Add a Transition to the collection.
    ///
    /// Returns `&Self` so for usability.
    pub(crate) fn add(&mut self, transition: Transition) -> &mut Self {
        self.items.insert(transition);

        self
    }
    
    /// Returns an iterator over the collection.
    pub(crate) fn iter(&self) ->  impl Iterator<Item = &Transition> {
        self.items.iter()
    }

    /// Returns the count of the unique source and target states in 
    /// the collection.
    pub(crate) fn state_count(&self) -> usize {
        let mut unique: HashSet<&str> = HashSet::new();

        for item in &self.items {
            unique.insert(item.source());
            unique.insert(item.target());
        }

        unique.len()
    }

    /// Return whether a transition exists based on `source` and `target`. 
    pub(crate) fn contains(
        &self, 
        source: &str,
        target: &str,
    ) -> bool {
        self.items.iter().any(|item| 
            item.source() == source &&
            item.target() == target
        )
    }
    
    /// Returns where the `state` appears as either endpoint of a transition.
    pub(crate) fn contains_state(
        &self,
        state: &str,
    ) -> bool {
        self.items.iter().any(|item| 
            item.source() == state || 
            item.target() == state
        )
    }

    /// Returns an iterator over the states directly reachable from `source`.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn targets_from(
        &self,
        source: &str
    ) -> impl Iterator<Item = &str> {
        self.items.iter().filter(move |&item| item.source() == source).map(Transition::target)
    }

    /// Returns an iterator over all source states.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn sources(&self) -> impl Iterator<Item = &str> {
        let mut unique: HashSet<&str> = HashSet::new();

        for item in &self.items {
            unique.insert(item.source());
        }

        unique.into_iter()
    }

    /// Returns an iterator over all unique source and target states.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn states(&self) -> impl Iterator<Item = &str> {
        let mut unique: HashSet<&str> = HashSet::new();

        for item in &self.items {
            unique.insert(item.source());
            unique.insert(item.target());
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
        fn len_returns_0_on_no_items() {
            let transitions = Transitions::new();

            assert_eq!(transitions.len(), 0);
        }

        #[test]
        fn len_returns_number_of_transitions() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?) 
                .add(Transition::try_new("3", "4")?)
                .add(Transition::try_new("5", "6")?);

            assert_eq!(transitions.len(), 3);

            Ok(())
        }
    }

    mod is_empty {
        use super::*;

        #[test]
        fn is_empty_returns_true_on_no_items() {
            let transitions = Transitions::new();

            assert!(transitions.is_empty());
        }

        #[test]
        fn is_empty_returns_false_with_an_item() -> Result<(), StateError> {
            let mut transitions = Transitions::new();
            transitions.add(Transition::try_new("1", "2")?);

            assert_eq!(transitions.is_empty(), false);

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
            
            items
                .add(Transition::try_new("1", "2")?)
                .add(Transition::try_new("2", "3")?)
                .add(Transition::try_new("2", "1")?)
                .add(Transition::try_new("5", "2")?);

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

            items.add(Transition::try_new("1","2")?);

            assert_eq!(items.state_count(), 2);

            Ok(())
        }

        #[test]
        fn state_count_two_shared_source_transitions_returns_3_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1","2")?);
            items.add(Transition::try_new("1","3")?);

            assert_eq!(items.state_count(), 3);

            Ok(())
        }

        #[test]
        fn state_count_same_transition_twice_returns_2_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1","2")?);
            items.add(Transition::try_new("1","2")?);

            assert_eq!(items.state_count(), 2);

            Ok(())
        }

        #[test]
        fn state_count_two_connected_transitions_returns_3_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1","2")?);
            items.add(Transition::try_new("2","3")?);

            assert_eq!(items.state_count(), 3);

            Ok(())
        }

        #[test]
        fn state_count_two_shared_terminal_transitions_returns_3_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1","2")?);
            items.add(Transition::try_new("4","2")?);

            assert_eq!(items.state_count(), 3);

            Ok(())
        }

        #[test]
        fn state_count_two_different_transitions_returns_4_states() -> Result<(), StateError> {
            let mut items = Transitions::new();

            items.add(Transition::try_new("1","2")?);
            items.add(Transition::try_new("4","3")?);

            assert_eq!(items.state_count(), 4);

            Ok(())
        }
    }


    mod contains {
        //use super::*;

    }
}
