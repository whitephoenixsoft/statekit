use std::collections::{HashMap, HashSet};

use crate::StateError;
use crate::Machine;
use crate::StateName;

/*
Contraints 
- empty string invalid
- self-transition invalid
- case-sensitive by default
- cycles allowed
- terminal states need no special behavior yet
*/

#[derive(Debug, PartialEq)]
pub struct MachineBuilder {
    transitions: HashMap<String, HashSet<String>>,
}

impl MachineBuilder {
    pub(crate) fn new() -> Self {
        Self {
            transitions: HashMap::new()
        }
    }
    
    pub fn allow(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        self.transitions.entry(from.into())
            .or_default()
            .insert(to.into());
        
        self
    }

    pub fn build(self) -> Result<Machine, StateError> {
        if self.transitions.is_empty() {
            return Err(StateError::NoTransitions);
        }
        
        for transition in self.transitions.iter() {
            if transition.0.is_empty() || transition.1.contains("")  {
                return Err(StateError::EmptyState);
            }
            else if transition.1.contains(transition.0)  {
                return Err(StateError::SelfTransition { 
                    state: transition.0.clone() 
                });
            }
        } 
         
        Ok(Machine {
            transitions: self.transitions
                .iter()
                .map(|s| (StateName(s.0.clone()), s.1.clone()))
                .collect()
        })
    }
    
    pub fn state_count(&self) -> usize {
        self.transitions.len()
    } 
    
    pub fn transition_count(&self) -> usize {
        self.transitions.values()
            .map(|s| s.len())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn new_builder_has_empty_map() {
        let builder = MachineBuilder::new();
        
        assert_eq!(builder.state_count(), 0);
    }
    
    #[test]
    fn allow_once_has_one_state() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        
        assert_eq!(builder.state_count(), 1);
    }

    #[test]
    fn allow_once_has_one_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        
        assert_eq!(builder.transition_count(), 1);
    }
    
    #[test]
    fn allow_twice_same_state_has_one_state() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "finish2");
        
        assert_eq!(builder.state_count(), 1);
    }

    #[test]
    fn allow_twice_same_state_has_two_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "finish2");
        
        assert_eq!(builder.transition_count(), 2);
    }
    
    #[test]
    fn allow_twice_same_state_same_transition_has_one_transition() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "finish");
        
        assert_eq!(builder.transition_count(), 1);
    }
    
    #[test]
    fn allow_twice_different_state_has_two_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start2", "finish2");
        
        assert_eq!(builder.state_count(), 2);
    }
    
    #[test]
    fn allow_twice_different_state_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start2", "finish2");
        
        assert_eq!(builder.transition_count(), 2);
    }
    
    #[test]
    fn allow_twice_continual_transition_has_two_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "mid");
        let builder = builder.allow("mid", "finish");
        
        assert_eq!(builder.state_count(), 2);
    }
    
    #[test]
    fn allow_twice_continual_transition_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "mid");
        let builder = builder.allow("mid", "finish");
        
        assert_eq!(builder.transition_count(), 2);
    }
    
    #[test]
    fn allow_case_sensitive_state_same_transition_has_two_states() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("Start", "finish");
        
        assert_eq!(builder.state_count(), 2);
    }
    
    #[test]
    fn allow_case_sensitive_state_same_transition_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("Start", "finish");
        
        assert_eq!(builder.transition_count(), 2);
    }
    
    #[test]
    fn allow_case_sensitive_same_transition_has_two_transitions() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "finish");
        let builder = builder.allow("start", "Finish");
        
        assert_eq!(builder.transition_count(), 2);
    }
    
    #[test]
    fn build_empty_from_state_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("", "finish");
        
        assert_eq!(builder.build(), Err(StateError::EmptyState));
    }
    
    #[test]
    fn build_empty_to_state_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "");
        
        assert_eq!(builder.build(), Err(StateError::EmptyState));
    }
    
    #[test]
    fn build_transaction_to_self_invalid() {
        let builder = MachineBuilder::new();
        let builder = builder.allow("start", "start");
        
        assert_eq!(builder.build(), Err(StateError::SelfTransition { 
            state: "start".to_string(),
        }));
    }
    
   #[test]
    fn build_empty_build_invalid() {
        let builder = MachineBuilder::new();
        
        assert_eq!(builder.build(), Err(StateError::NoTransitions));
    }
}
