use std::borrow::Borrow;

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

impl From<&str> for StateName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for StateName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Borrow<str> for StateName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
