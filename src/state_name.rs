use crate::StateError;
use std::borrow::Borrow;

/// A validated identifier for a state within a machine.
///
/// A `StateName` is guaranteed to be non-empty and to contain no
/// leading or trailing whitespace.
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) struct StateName(String);

impl StateName {
    /// Convert the state to type &str.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Consume the state into type String
    pub(crate) fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for StateName {
    type Error = StateError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate_state_name(value)?;

        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for StateName {
    type Error = StateError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_state_name(&value)?;

        Ok(Self(value))
    }
}

impl Borrow<str> for StateName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

/// Validates a state name against the crate's naming invariants.
fn validate_state_name(value: &str) -> Result<(), StateError> {
    if value.trim().is_empty() {
        return Err(StateError::EmptyState);
    }

    if value != value.trim() {
        return Err(StateError::AmbiguousStateName);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str_valid_state_succeeds() {
        let state = StateName::try_from("in progress").unwrap();

        assert_eq!(state.as_str(), "in progress");
    }

    #[test]
    fn try_from_string_reuses_valid_input() {
        let state = StateName::try_from(String::from("queued")).unwrap();

        assert_eq!(state.as_str(), "queued");
    }

    #[test]
    fn try_from_rejects_leading_whitespace() {
        let result = StateName::try_from(" queued");

        assert_eq!(result, Err(StateError::AmbiguousStateName));
    }

    #[test]
    fn validate_state_name_one_word_returns_ok() {
        let result = validate_state_name("something");

        assert!(result.is_ok());
    }

    #[test]
    fn validate_state_name_two_words_returns_ok() {
        let result = validate_state_name("something else");

        assert!(result.is_ok());
    }

    #[test]
    fn validate_state_name_empty_returns_error() {
        let result = validate_state_name("");

        assert_eq!(result, Err(StateError::EmptyState));
    }

    #[test]
    fn validate_state_name_whitespace_returns_error() {
        let result = validate_state_name(" \n\t");

        assert_eq!(result, Err(StateError::EmptyState));
    }

    #[test]
    fn validate_state_name_whitespace_before_name_returns_error() {
        let result = validate_state_name("\n\t something");

        assert_eq!(result, Err(StateError::AmbiguousStateName));
    }

    #[test]
    fn validate_state_name_whitespace_after_name_returns_error() {
        let result = validate_state_name("something\n\t ");

        assert_eq!(result, Err(StateError::AmbiguousStateName));
    }
}
