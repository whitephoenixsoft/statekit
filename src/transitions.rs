use std::collections::HashSet;

use crate::Transition;

/// A collection of Transition items.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Transitions {
    items: HashSet<Transition>,
}

impl Transitions {
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
    /// Returns `&Self` so for convenience.
    pub(crate) fn add(&mut self, transition: Transition) -> &Self {
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
