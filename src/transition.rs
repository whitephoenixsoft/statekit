use crate::StateError;
use crate::StateName;

/// A validated identifier for a transition within a state machine.
/// 
/// A `Transition` is guaranteed to have valid state names
/// and that the source and target states are
/// different.
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Transition {
    source: StateName,
    target: StateName,
}

impl Transition {
    /// Contruct a machine
    pub(crate) fn try_new(
        source: StateName,
        target: StateName,
    ) -> Result<Self, StateError> {
        if source == target {
            return Err(StateError::SelfTransition{
                state: source.into_string(),
            });
        }
        
        Ok(Self {
            source,
            target,
        })
    }
    
    /// Return the source state of the transition.
    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    /// Return the target state of the transition.
    pub fn target(&self) -> &str {
        self.target.as_str()
    }
}
