use crate::StateError;
use crate::StateName;

/// A validated directed transition between two states.
///
/// A `Transition` guarantees that both state names are valid
/// and that the source and target states are different.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transition {
    source: StateName,
    target: StateName,
}

impl Transition {
    /// Creates a validated transition from `source` to `target`.
    pub(crate) fn try_new(
        source: impl AsRef<str>,
        target: impl AsRef<str>,
    ) -> Result<Self, StateError> {
        let source = StateName::try_from(source.as_ref())?;
        let target = StateName::try_from(target.as_ref())?;

        if source == target {
            return Err(StateError::SelfTransition {
                state: source.into_string(),
            });
        }

        Ok(Self { source, target })
    }

    /// Returns the source state of the transition.
    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    /// Returns the target state of the transition.
    pub fn target(&self) -> &str {
        self.target.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_returns_correct_value() -> Result<(), StateError> {
        let transition = Transition::try_new("1", "2")?;

        assert_eq!(transition.source(), "1");

        Ok(())
    }

    #[test]
    fn target_returns_correct_value() -> Result<(), StateError> {
        let transition = Transition::try_new("1", "2")?;

        assert_eq!(transition.target(), "2");

        Ok(())
    }

    #[test]
    fn self_transition_returns_error()  {
        let result = Transition::try_new("1", "1");

        assert!(matches!(
            result,
            Err(StateError::SelfTransition { ref state }) if state == "1"
        ));
    }

    #[test]
    fn similar_transitions_are_equal() -> Result<(), StateError> {
        let transition1 = Transition::try_new("1", "2")?;
        let transition2 = Transition::try_new("1", "2")?;

        assert_eq!(transition1, transition2);

        Ok(())
    }

    #[test]
    fn different_transitions_are_not_equal() -> Result<(), StateError> { 
        let transition1 = Transition::try_new("1", "2")?;
        let transition2 = Transition::try_new("2", "3")?;

        assert_ne!(transition1, transition2);

        Ok(())
    }

}
