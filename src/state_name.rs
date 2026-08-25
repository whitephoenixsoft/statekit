    }
}

    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Validates a state name against the crate's naming invariants.
fn validate_state_name(value: &str) -> Result<(), StateError> {
    if value.trim().is_empty() {
