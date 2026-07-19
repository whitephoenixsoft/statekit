use thiserror::Error;

/// Errors produced whn contructing or using a state machine.
///
/// Each variant represents a violation of one of the crate's domain invariants.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StateError {
    ///The machine must contain at lease one transition.
    #[error("state machine must define at least one transition")]
    NoTransitions,

    ///A state name must not be empty.
    #[error("state names must not be empty")]
    EmptyState,

    ///A transition must connect two different states.
    #[error("state must not transition to itself: {state}")]
    SelfTransition {
        ///The state that was used as both the source and destination.
        state: String,
    },

    ///The requested transition must be present in the machine definition.
    #[error("transition from {from} to {to} is not allowed")]
    InvalidTransition {
        ///The source state.
        from: String,
        ///The requested destination state.
        to: String,
    },
}
