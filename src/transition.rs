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
