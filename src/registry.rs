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
        let states1 = self.states.clone().sort_unstable();
        let states2 = other.states.clone().sort_unstable();
        
        states1.eq(states2)   
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StateError;
    
    mod get_or_insert {
        use super::*;

        #[test]
        fn no_states_returns_new_id() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();

            let state = StateName::try_from("a")?;
            let id = registry.get_or_insert(state);
            
            assert_eq!(id, 0);

            Ok(())
        }
        
        #[test]
        fn one_state_returns_new_id() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();

            let state = StateName::try_from("a")?;
            let state2 = StateName::try_from("b")?;
            
            _ = registry.get_or_insert(state);
            let id = registry.get_or_insert(state2);
            
            assert_eq!(id, 1);

            Ok(())
        }
        
        #[test]
        fn duplicate_state_returns_old_id() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();

            let state = StateName::try_from("a")?;
            let state2 = StateName::try_from("a")?;
            
            _ = registry.get_or_insert(state);
            let id = registry.get_or_insert(state2);
            
            assert_eq!(id, 0);

            Ok(())
        }
    }
    
    mod len {
        use super::*;

        #[test]
        fn no_state_returns_0() {
             let registry = StateRegistry::new();

            assert_eq!(items.len(), 0);
        }

        #[test]
        fn one_state_returns_1() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("a")?;
            
            _ = registry.get_or_insert(state);

            assert_eq!(registry.len(), 1);

            Ok(())
        }

        #[test]
        fn same_state_twice_returns_1_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("a")?;
            let state2 = StateName::try_from("a")?;
            
            _ = registry.get_or_insert(state);
            _ = registry.get_or_insert(state2);

            assert_eq!(registry.len(), 1);

            Ok(())
        }
    }
    
    mod id_of_name {
        use super::*;

        #[test]
        fn no_items_returns_none() {
            let registry = StateRegistry::new();

            assert_eq!(registry.id_of_name("1"), None);
        }
        
        #[test]
        fn finds_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            
            _ = registry.get_or_insert(state);

            assert_eq!(registry.id_of_name("1"), Some(0));

            Ok(())
        }

        #[test]
        fn rejects_unknown_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            
            _ = registry.get_or_insert(state);

            assert_eq!(registry.id_of_name("2"), None);

            Ok(())
        }
    }
    
    mod id_of {
        use super::*;

        #[test]
        fn no_items_returns_none() -> Result<(), StateError> {
            let registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;

            assert_eq!(registry.id_of(state), None);
        }
        
        #[test]
        fn finds_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            let state2 = StateName::try_from("1")?;
            
            _ = registry.get_or_insert(state);

            assert_eq!(registry.id_of(state2), Some(0));

            Ok(())
        }

        #[test]
        fn rejects_unknown_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            let state = StateName::try_from("2")?;
            
            _ = registry.get_or_insert(state);

            assert_eq!(registry.id_of(state2), None);

            Ok(())
        }
    }
    
    mod state_of {
        use super::*;
        
        #[test]
        fn no_items_returns_none() {
            let registry = StateRegistry::new();

            assert_eq!(registry.state_of(0), None);
        }
        
        #[test]
        fn finds_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            
            _ = registry.get_or_insert(state);

            assert_eq!(registry.state_of(0), Some(state));

            Ok(())
        }

        #[test]
        fn rejects_unknown_id() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            
            _ = registry.get_or_insert(state);

            assert_eq!(registry.state_of(1), None);

            Ok(())
        }
    }
    
    mod states {
        use super::*;

        #[test]
        fn no_items_returns_empty() {
            let registry = StateRegistry::new();

            let collected: Vec<_> = registry.states().collect();

            assert!(collected.is_empty());
        }

        #[test]
        fn one_state_returns_1_value() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            
            _ = registry.get_or_insert(state);
            
            let mut collected: Vec<_> = registry.states().map(|s| s.as_str()).collect();

            assert_eq!(collected, vec!["1"]);

            Ok(())
        }
        
        #[test]
        fn two_states_returns_2_values() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;
            
            _ = registry.get_or_insert(state);
            _ = registry.get_or_insert(state2);
            
            let mut collected: Vec<_> = registry.states().map(|s| s.as_str()).collect();
            collected.sort_unstable();

            assert_eq!(collected, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn returns_unique_states() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            for i in 1..4 {
                let state = StateName::try_from(String::from(i))?;
                let state2 = StateName::try_from(String::from(i))?;
                
                _ = registry.get_or_insert(state);
                _ = registry.get_or_insert(state2);
            }
            
            let mut collected: Vec<_> = registry.states().map(|s| s.as_str()).collect();

            let mut collected: Vec<_> = items.states().collect();
            collected.sort();

            assert_eq!(collected, vec!["1", "2", "3", "4"]);

            Ok(())
        }
    }
    
    mod equality {
        use super::*;
        
        #[test]
        fn similar_registries_are_equal() {
            let state1 = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;
            
            let mut registry1 = StateRegistry::new();
            _ = registry1.get_or_insert(state1);
            _ = registry1.get_or_insert(state2);

            let mut registry2 = StateRegistry::new();
            _ = registry2.get_or_insert(state2);
            _ = registry2.get_or_insert(state1);


            assert_eq!(registry1, registry2);
        }

        #[test]
        fn different_registries_are_not_equal() {
            let state1 = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;
            
            let mut registry1 = StateRegistry::new();
            _ = registry1.get_or_insert(state1);
            _ = registry1.get_or_insert(state2);

            let mut registry2 = StateRegistry::new(););
            _ = registry2.get_or_insert(state1);

            assert_ne!(registry1, registry2);
        }
    }
}

