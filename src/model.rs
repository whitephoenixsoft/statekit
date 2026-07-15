
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct StateName(pub String);

impl From<&str> for StateName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
} 

impl From<&String> for StateName {
    fn from(s: &String) -> Self {
        Self(s.clone())
    }
}
