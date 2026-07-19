use std::collections::{HashMap, HashSet};

use crate::MachineBuilder;
use crate::StateError;
use crate::StateName;

/*
    pub fn targets(&self, from: &str) -> Option<impl Iterator<Item = &str>>
pub fn contains_state(&self, state: &str) -> bool
pub fn transition_count(&self) -> usize
---
builder.try_allow("a", "b")?
allow(...) -> Self          // ergonomic, validates on build
try_allow(...) -> Result<Self, StateError>
---
let definition = Machine::builder().allow(...).build()?;
let mut instance = definition.start_at("queued")?;
instance.transition_to("running")?;
    
    */

#[derive(Debug, PartialEq)]
pub struct Machine {
    pub(crate) transitions: HashMap<StateName, HashSet<String>>,
}

impl Machine {
    pub fn builder() -> MachineBuilder {
        MachineBuilder::new() 
    }
    
    pub fn can_transition(&self, from: &str, to: &str) -> bool {
        let state: StateName = from.into();
        self.transitions
            .get(&state)
            .is_some_and(|targets| targets.contains(to))
    }

    pub fn transition(
        &self,
        from: &str,
        to: &str,
    ) -> Result<(), StateError> {
        if self.can_transition(from, to) {
            Ok(())
        }
        else {
            Err(StateError::InvalidTransition { 
                from: from.to_string(), 
                to: to.to_string(),
            })
        }   
    }

    pub fn transition_count(&self) -> usize {
        self.transitions.values()
            .map(|s| s.len())
            .sum()
    }

    pub fn contains_state(&self, state: &str) -> bool {
        self.transitions.keys()
            .any(|s| s.0 == state) ||
        self.transitions.values()
            .any(|s| s.contains(state))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn transition_exists_returns_next_state() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.transition("start", "finish").is_ok()); 
            
    }
    
    #[test]
    fn transaction_count_1_transaction() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert_eq!(m.transition_count(), 1); 
    }

    #[test]
    fn transition_exists_multiple_states_returns_next_state() {
        let builder = Machine::builder()
            .allow("start", "finish")
            .allow("1", "2");
            
        let m = builder.build().unwrap();
        
        assert!(m.transition("start", "finish").is_ok()); 
    }
    
    #[test]
    fn transaction_count_2_transactions() {
        let builder = Machine::builder()
            .allow("start", "finish")
            .allow("1", "2");
            
        let m = builder.build().unwrap();
        
        assert_eq!(m.transition_count(), 2); 
    }

    #[test]
    fn can_transition_exists_returns_true() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.can_transition("start", "finish"));
    }
    
    #[test]
    fn transition_not_exists_returns_invalid_error() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert_eq!(m.transition("start", "invalid"), 
            Err(StateError::InvalidTransition {
                from: "start".to_string(),
                to: "invalid".to_string(),
            }));
    }
    
    #[test]
    fn can_transition_not_exists_returns_false() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert_eq!(m.can_transition("start", "invalid"), 
            false);
    }
    
    #[test]
    fn transition_cyclic_is_valid() {
        let builder = Machine::builder()
            .allow("start", "finish")
            .allow("finish", "start");
            
        let m = builder.build().unwrap();
        
        assert!(m.transition("finish", "start").is_ok());
    }

    #[test]
    fn contains_state_one_transaction_existing_to_state_returns_true() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("start")); 
    }

    #[test]
    fn contains_state_one_transaction_existing_from_state_returns_true() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("finish")); 
    }

    #[test]
    fn contains_state_one_transaction_nonexisting_state_returns_false() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("other") == false); 
    }

    #[test]
    fn contains_state_two_transactions_on_same_from_state_finds_from_state() {
        let builder = Machine::builder()
            .allow("start", "rest")
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("start")); 
    }

    #[test]
    fn contains_state_two_transactions_on_same_from_state_finds_first_to_state() {
        let builder = Machine::builder()
            .allow("start", "rest")
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("rest")); 
    }

    #[test]
    fn contains_state_two_transactions_on_same_from_state_finds_second_to_state() {
        let builder = Machine::builder()
            .allow("start", "rest")
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("finish")); 
    }

    #[test]
    fn contains_state_two_transactions_on_different_from_state_finds_first_from_state() {
        let builder = Machine::builder()
            .allow("start", "end")
            .allow("rest", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("start")); 
    }

    #[test]
    fn contains_state_two_transactions_on_different_from_state_finds_second_from_state() {
        let builder = Machine::builder()
            .allow("start", "end")
            .allow("rest", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("rest")); 
    }

    #[test]
    fn contains_state_two_transactions_on_different_from_state_finds_first_to_state() {
        let builder = Machine::builder()
            .allow("start", "end")
            .allow("rest", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("end")); 
    }

    #[test]
    fn contains_state_two_transactions_on_different_from_state_finds_second_to_state() {
        let builder = Machine::builder()
            .allow("start", "end")
            .allow("rest", "finish");
            
        let m = builder.build().unwrap();
        
        assert!(m.contains_state("finish")); 
    }
}
