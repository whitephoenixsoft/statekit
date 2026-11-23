use crate::StateName;

/// A registry of states.
#[derive(Debug, Default, Eq)]
pub(crate) struct StateRegistry {
    states: Vec<StateName>
}

impl StateRegistry {
    pub(crate) fn new() {
        Self {
            states: Vec::new(),
        }
    }
    
    /// Returns the state id from a StateName.
    pub(crate) fn id_of(&self, name: &StateName) -> Option<usize> {
        self.states.iter().position(|s| s == name)
    }

    /// Returns the state id from a &str.
    pub(crate) fn id_of_name(&self, name: &str) -> Option<usize> {
        self.states.iter().position(|s| s.as_str() == name)
    }
    
    /// Insert a state atomically.
    pub(crate) get_or_insert(&mut self, name: StateName) -> usize {
        if let Some(id) = self.id_of(name) {
            return id;
        }

        let id = self.states.len();
        self.states.push(name);
        id
    }
    
    /// Returns a state name from an id.
    pub(crate) fn state_of(&self, id: usize) -> Option<&StateName> {
        if id < self.states.len() {
            return Some(self.states[id]);
        }
        
        None
    }
    
    /// Returns the count of the unique source and target states in
    /// the collection.
    pub(crate) fn len(&self) -> usize {
        self.states.len()
    }
    
    /// Returns an iterator over all unique states.
    ///
    /// The iteration order is unspecified.
    pub(crate) fn states(&self) -> impl Iterator<Item = &StateName> {
        self.states.iter()
    }
}

impl PartialEq for StateRegistry {
    fn eq(&self, other: &Self) -> bool {
        //states must be in the same order to compare
        self.states.sort_unstable() == other.states.sort_unstable()    
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StateError;
    
    mod get_or_insert {
        use super::*;

        #[test]
        fn no_states_returns_id() -> Result<(), StateError> {
            let registry = StateRegistry::new();

            let state = StateName::try_from("a")?;
            let id = registry.get_or_insert(state);
            
            assert_eq!(id, 0);

            Ok(())
        }
    }
}

}
