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
        if self.states.len() != other.states.len() {
            return false;
        }

        let mut states1: Vec<_> = self.states
            .iter()
            .map(|state| state.as_str())
            .collect();

        let mut states2: Vec<_> = other.states
            .iter()
            .map(|state| state.as_str())
            .collect();

        states1.sort_unstable();
        states2.sort_unstable();

        states1 == states2
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
            
            registry.get_or_insert(state);
            let id = registry.get_or_insert(state2);
            
            assert_eq!(id, 1);

            Ok(())
        }

        #[test]
        fn state_registration_is_case_insensitive() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();

            let state = StateName::try_from("a")?;
            let state2 = StateName::try_from("A")?;
            
            let id = registry.get_or_insert(state);
            let id2 = registry.get_or_insert(state2);
            
            assert_ne!(id, id2);

            Ok(())
        }
        
        #[test]
        fn duplicate_state_returns_old_id() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();

            let state = StateName::try_from("a")?;
            let state2 = StateName::try_from("a")?;
            
            registry.get_or_insert(state);
            let id = registry.get_or_insert(state2);
            
            assert_eq!(id, 0);

            Ok(())
        }

        #[test]
        fn new_state_preserves_existing_ids() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            let mut registered_ids = Vec::new();

            for i in 0..5 {
                let s = i.to_string();
                
                let state = StateName::try_from(i.as_str())?;
                registered_ids.push(registry.get_or_insert(state));
            }
            
            for i in 0..5 {
                let s = i.to_string();
                assert_eq!(registered_ids[i], s);
            }

            Ok(())
        }

        #[test]
        fn duplicate_registration_preserves_existing_ids() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            let mut registered_ids = Vec::new();

            for i in 0..5 {
                let s = i.to_string();
                
                let state = StateName::try_from(i.as_str())?;
                registered_ids.push(registry.get_or_insert(state));
            }
            
            for i in 0..5 {
                let s = i.to_string();
                let state = StateName::try_from(i.as_str())?;
                assert_eq!(registered_ids[i], registry.get_or_insert(state));
            }

            Ok(())
        }
    }
    
    mod len {
        use super::*;

        #[test]
        fn no_state_returns_0() {
             let registry = StateRegistry::new();

            assert_eq!(registry.len(), 0);
        }

        #[test]
        fn one_state_returns_1() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("a")?;
            
            registry.get_or_insert(state);

            assert_eq!(registry.len(), 1);

            Ok(())
        }

        #[test]
        fn same_state_twice_returns_1_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("a")?;
            let state2 = StateName::try_from("a")?;
            
            registry.get_or_insert(state);
            registry.get_or_insert(state2);

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
            
            registry.get_or_insert(state);

            assert_eq!(registry.id_of_name("1"), Some(0));

            Ok(())
        }

        #[test]
        fn rejects_unknown_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            
            registry.get_or_insert(state);

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
            
            Ok(())
        }
        
        #[test]
        fn finds_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            let state2 = StateName::try_from("1")?;
            
            registry.get_or_insert(state);

            assert_eq!(registry.id_of(state2), Some(0));

            Ok(())
        }

        #[test]
        fn rejects_unknown_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;
            
            registry.get_or_insert(state);

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
            let state2 = state.clone();
            
            registry.get_or_insert(state);

            assert_eq!(registry.state_of(0), Some(state2));

            Ok(())
        }

        #[test]
        fn rejects_unknown_id() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            
            registry.get_or_insert(state);

            assert_eq!(registry.state_of(1), None);

            Ok(())
        }

        #[test]
        fn id_maps_back_state() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            let mut registered_ids = Vec::new();

            for i in 0..5 {
                let s = i.to_string();
                
                let state = StateName::try_from(i.as_str())?;
                registered_ids.push(registry.get_or_insert(state));
            }
            
            for i in 0..5 {
                let s = i.to_string();
                let state = StateName::try_from(i.as_str())?;
                assert_eq!(registry.state_of(registered_ids[i]), state);
            }
             
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
            
            registry.get_or_insert(state);
            
            let mut collected: Vec<_> = registry.states().map(|s| s.as_str()).collect();

            assert_eq!(collected, vec!["1"]);

            Ok(())
        }
        
        #[test]
        fn two_states_returns_2_values() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            let state = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;
            
            registry.get_or_insert(state);
            registry.get_or_insert(state2);
            
            let mut collected: Vec<_> = registry.states().map(|s| s.as_str()).collect();
            collected.sort_unstable();

            assert_eq!(collected, vec!["1", "2"]);

            Ok(())
        }

        #[test]
        fn returns_unique_states() -> Result<(), StateError> {
            let mut registry = StateRegistry::new();
            
            for i in 1..=4 {
                let name = i.to_string();

                let state1 = StateName::try_from(name.as_str())?;
                let state2 = StateName::try_from(name.as_str())?;
                registry.get_or_insert(state1);
                registry.get_or_insert(state2);
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
        fn empty_registries_are_equal() {
            let registry1 = StateRegistry::new();
            
            let registry2 = StateRegistry::new();
            
            assert_eq!(registry1, registry2);
        }
        
        #[test]
        fn similar_registries_are_equal() -> Result<(), StateError> {
            let state1 = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;
            
            let mut registry1 = StateRegistry::new();
            registry1.get_or_insert(state1.clone());
            registry1.get_or_insert(state2.clone());

            let mut registry2 = StateRegistry::new();
            registry2.get_or_insert(state2);
            registry2.get_or_insert(state1);

            assert_eq!(registry1, registry2);

            Ok(())
        }
        
        #[test]
        fn different_length_registries_are_not_equal() -> Result<(), StateError> {
            let state1 = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;
            
            let mut registry1 = StateRegistry::new();
            registry1.get_or_insert(state1.clone());
            registry1.get_or_insert(state2);

            let mut registry2 = StateRegistry::new();
            
            registry2.get_or_insert(state1);
            
            assert_ne!(registry1, registry2);

            Ok(())
        }

        #[test]
        fn different_registries_are_not_equal() -> Result<(), StateError> {
            let state1 = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;
            let state3 = StateName::try_from("3")?;
            
            let mut registry1 = StateRegistry::new();
            registry1.get_or_insert(state1.clone());
            registry1.get_or_insert(state2);

            let mut registry2 = StateRegistry::new(););
            registry2.get_or_insert(state1);
            registry2.get_or_insert(state3);

            assert_ne!(registry1, registry2);

            Ok(())
        }

        #[test]
        fn equality_does_not_modify_first_registry() -> Result<(), StateError> {
            let state1 = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;

            let mut registry1 = StateRegistry::new();
            let r1s1 = registry1.get_or_insert(state1.clone());
            let r1s2 = registry1.get_or_insert(state2.clone());

            let mut registry2 = StateRegistry::new();
            registry2.get_or_insert(state1);
            registry2.get_or_insert(state2);

            let are_equal = registry1 == registry2;

            assert!(are_equal);
            assert_eq!(registry1.state_of(r1s1).as_str(), "1");
            assert_eq!(registry1.state_of(r1s2).as_str(), "2");

            Ok(())
        }

        #[test]
        fn equality_does_not_modify_second_registry() -> Result<(), StateError> {
            let state1 = StateName::try_from("1")?;
            let state2 = StateName::try_from("2")?;

            let mut registry1 = StateRegistry::new();
            registry1.get_or_insert(state1.clone());
            registry1.get_or_insert(state2.clone());

            let mut registry2 = StateRegistry::new();
            let r2s1 = registry2.get_or_insert(state1);
            let r2s2 = registry2.get_or_insert(state2);

            let are_equal = registry1 == registry2;

            assert!(are_equal);
            assert_eq!(registry2.state_of(r2s1).as_str(), "1");
            assert_eq!(registry2.state_of(r2s2).as_str(), "2");

            Ok(())
        }
    }
}

