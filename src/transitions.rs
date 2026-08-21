use std::collections::HashSet;

use crate::Transition;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Transitions {
    items: HashSet<Transition>,
}

impl Transitions {
    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn add(&mut self, transition: Transition) -> &Self {
        self.items.insert(transition);

        self
    }
    
    pub(crate) fn iter(&self) ->  impl Iterator<Item = &Transition> {
        self.items.iter()
    }

    pub(crate) fn state_count(&self) -> usize {
        let mut unique: HashSet<&str> = HashSet::new();

        for item in &self.items {
            unique.insert(item.source());
            unique.insert(item.target());
        }

        unique.len()
    }

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
    
    pub(crate) fn contains_state(
        &self,
        state: &str,
    ) -> bool {
        self.items.iter().any(|item| 
            item.source() == state || 
            item.target() == state
        )
    }

    pub(crate) fn targets_from(
        &self,
        source: &str
    ) -> Option<impl Iterator<Item = &str>> {
        Some(self.items.iter().filter(move |&item| item.source() == source).map(Transition::target))
    }


    pub(crate) fn sources(&self) -> impl Iterator<Item = &str> {
        self.items.iter().map(Transition::source)
    }

    pub(crate) fn states(&self) -> impl Iterator<Item = &str> {
        let mut unique: HashSet<&str> = HashSet::new();

        for item in &self.items {
            unique.insert(item.source());
            unique.insert(item.target());
        }

        unique.into_iter()
    }
}
