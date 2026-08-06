use std::borrow::Borrow;
use crate::StateError;

/// A validated identifier for a state within a machine.
///
/// `StateName` exists to represent the domain concept of a state.
/// Future versions will enforce naming invariants during construction.
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) struct StateName(String);

impl StateName {
    /// Convert the state to type &str.
    pub fn as_str(&self) -> &str {
        &self.0
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

/// Validates the state definition.
///
/// Returns a result of Ok() or a [`StateError`] if validation fails.
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
    fn validatate_state_name_one_word_returns_ok() {
        let result = validate_state_name("something");
        
        assert!(result.is_ok());
    }

    #[test]
    fn validatate_state_name_two_words_returns_ok() {
        let result = validate_state_name("something else");
        
        assert!(result.is_ok());
    }

    #[test]
    fn validatate_state_name_empty_returns_error() {
        let result = validate_state_name("");
        
        assert_eq!(result, Err(StateError::EmptyState));
    }

    #[test]
    fn validatate_state_name_whitepace_returns_error() {
        let result = validate_state_name(" \n\t");
        
        assert_eq!(result, Err(StateError::EmptyState));
    }

    #[test]
    fn validatate_state_name_whitespace_before_name_returns_error() {
        let result = validate_state_name("\n\t something");
        
        assert_eq!(result, Err(StateError::AmbiguousStateName));
    }

    #[test]
    fn validatate_state_name_whitespace_after_name_returns_error() {
        let result = validate_state_name("something\n\t ");
        
        assert_eq!(result, Err(StateError::AmbiguousStateName));
    }
}
