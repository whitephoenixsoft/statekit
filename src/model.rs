
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct StateName(String);

impl From<&str> for StateName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
} 
