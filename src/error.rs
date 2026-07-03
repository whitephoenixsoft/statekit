use thiserror::Error;

/// The possible errors the state machine.    
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StateError {
    #[error("state machine must define at least one transition")]
    NoTransitions,

    #[error("state names cannot be empty")]
    EmptyState,

    #[error("state cannot transition to itself: {state}")]
    SelfTransition { state: String },

    #[error("invalid transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },
}