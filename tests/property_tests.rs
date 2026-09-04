use proptest::prelude::*;
use statekit::{Machine, StateError};

use proptest::prelude::*;

fn valid_state_name() -> impl Strategy<Value = String> {
    "[A-Za-z]{1,16}"
}

fn whitespace_only_state_name() -> impl Strategy<Value = String> {
    "[ \t\n]{1,8}"
}

fn leading_whitespace_state_name() -> impl Strategy<Value = String> {
    (
        "[ \t]{1,4}",
        valid_state_name(),
    )
    .prop_map(|(whitespace, name)| {
        format!("{whitespace}{name}")
    })
}

fn trailing_whitespace_state_name() -> impl Strategy<Value = String> {
    (
        "[ \t]{1,4}",
        valid_state_name(),
    )
    .prop_map(|(whitespace, name)| {
        format!("{name}{whitespace}")
    })
}

proptest! {
    #[test]
    fn added_transition_is_allowed(
        source in valid_state_name(),
        target in valid_state_name(),
    ) {
        prop_assume!(source != target);

        let machine = Machine::builder()
            .try_allow(&source, &target)
            .and_then(|builder| builder.build());

        prop_assert!(machine.is_ok());

        let machine = machine.unwrap();

        prop_assert!(machine.can_transition(&source, &target));
    }

    #[test]
    fn accepted_transitions_preserve_state_invariants(
        source in any::<String>(),
        target in any::<String>(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target)
            .and_then(|builder| builder.build());

        if let Ok(machine) = result {
            let transition = machine
                .transitions()
                .next()
                .expect("a successfully built machine has a transition");

            prop_assert_eq!(transition.source().trim(), transition.source());
            prop_assert_eq!(transition.target().trim(), transition.target());

            prop_assert!(!transition.source().trim().is_empty());
            prop_assert!(!transition.target().trim().is_empty());

            prop_assert_ne!(transition.source(), transition.target());
        }
    }
    
    #[test]
    fn whitespace_only_source_is_rejected(
        source in whitespace_only_state_name(),
        target in valid_state_name(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target);

        prop_assert!(matches!(
            result,
            Err(StateError::EmptyState)
        ));
    }

    #[test]
    fn leading_whitespace_source_is_rejected(
        source in leading_whitespace_state_name(),
        target in valid_state_name(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target);

        let is_ambiguous = matches!(
            result,
            Err(StateError::AmbiguousStateName { .. })
        );

        prop_assert!(is_ambiguous);
    }

    #[test]
    fn leading_whitespace_error_preserves_offending_source_state(
        source in leading_whitespace_state_name(),
        target in valid_state_name(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target);

        match result {
            Err(StateError::AmbiguousStateName { state }) => {
                prop_assert_eq!(state, source);
            }
            other => {
                prop_assert!(
                    false,
                    "expected AmbiguousStateName, got {other:?}"
                );
            }
        }
    }

    #[test]
    fn leading_whitespace_error_preserves_offending_target_state(
        source in valid_state_name(),
        target in leading_whitespace_state_name(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target);

        match result {
            Err(StateError::AmbiguousStateName { state }) => {
                prop_assert_eq!(state, target);
            }
            other => {
                prop_assert!(
                    false,
                    "expected AmbiguousStateName, got {other:?}"
                );
            }
        }
    }

    #[test]
    fn trailing_whitespace_source_is_rejected(
        source in trailing_whitespace_state_name(),
        target in valid_state_name(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target);

        let is_ambiguous = matches!(
            result,
            Err(StateError::AmbiguousStateName { .. })
        );

        prop_assert!(is_ambiguous);
    }

    #[test]
    fn trailing_whitespace_error_preserves_offending_source_state(
        source in trailing_whitespace_state_name(),
        target in valid_state_name(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target);

        match result {
            Err(StateError::AmbiguousStateName { state }) => {
                prop_assert_eq!(state, source);
            }
            other => {
                prop_assert!(
                    false,
                    "expected AmbiguousStateName, got {other:?}"
                );
            }
        }
    }

    #[test]
    fn trailing_whitespace_error_preserves_offending_target_state(
        source in valid_state_name(),
        target in trailing_whitespace_state_name(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target);

        match result {
            Err(StateError::AmbiguousStateName { state }) => {
                prop_assert_eq!(state, target);
            }
            other => {
                prop_assert!(
                    false,
                    "expected AmbiguousStateName, got {other:?}"
                );
            }
        }
    }
}
