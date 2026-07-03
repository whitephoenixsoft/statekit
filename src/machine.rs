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
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn transition_exists_returns_next_state() {
        let builder = Machine::builder()
            .allow("start", "finish");
            
        let m = builder.build().unwrap();
        
        assert_eq!(m.transition("start", "finish"), 
            Ok("finish".to_string()));
    }
    
    #[test]
    fn transition_exists_multiple_states_returns_next_state() {
        let builder = Machine::builder()
            .allow("start", "finish")
            .allow("1", "2");
            
        let m = builder.build().unwrap();
        
        assert_eq!(m.transition("start", "finish"), 
            Ok("finish".to_string()));
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
        
        assert_eq!(m.transition("finish", "start"), 
            Ok("start".to_string()));
    }
}