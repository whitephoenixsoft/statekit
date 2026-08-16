use thiserror::Error;

/// Errors produced when constructing or using a state machine.
///
/// Each variant represents either a violated construction invariant
/// or an invalid operation against a machine definition.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StateError {
    /// The machine must contain at least one transition.
    #[error("state machine must define at least one transition")]
    NoTransitions,

    /// A state name must not be empty.
    #[error("state names must not be empty")]
    EmptyState,

    /// A state name must not begin or end with Unicode whitespace.
    #[error("state names must not begin or end with Unicode whitespace")]
    AmbiguousStateName,

    /// A transition must connect two different states.
    #[error("self-transitions are not allowed for state `{state}`")]
    SelfTransition {
        /// The state that was used as both the source and destination.
        state: String,
    },

    /// The requested transition must be present in the machine definition.
    #[error("transition from `{from}` to `{to}` is not allowed")]
    InvalidTransition {
        /// The source state.
        from: String,
        /// The requested destination state.
        to: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_display_message() {
        let error = StateError::EmptyState;

        assert_eq!(error.to_string(), "state names must not be empty");
    }

    #[test]
    fn no_transitions_display_message() {
        let error = StateError::NoTransitions;

        assert_eq!(
            error.to_string(),
            "state machine must define at least one transition"
        );
    }

    #[test]
    fn self_transition_display_message_includes_state() {
        let error = StateError::SelfTransition {
            state: "start".to_owned(),
        };

        assert_eq!(
            error.to_string(),
            "self-transitions are not allowed for state `start`"
        );
    }

    #[test]
    fn invalid_transition_display_message_includes_endpoints() {
        let error = StateError::InvalidTransition {
            from: "start".to_owned(),
            to: "finish".to_owned(),
        };

        assert_eq!(
            error.to_string(),
            "transition from `start` to `finish` is not allowed"
        );
    }

    #[test]
    fn ambiguous_state_name_display_message() {
        let error = StateError::AmbiguousStateName;

        assert_eq!(
            error.to_string(),
            "state names must not begin or end with Unicode whitespace"
        );
    }
}
