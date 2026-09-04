#[allow(unused)]
use proptest::prelude::*;

use statekit::{Machine, StateError};

use proptest::prelude::*;

fn valid_state_name() -> impl Strategy<Value = String> {
    "[A-Za-z]{1,16}"
}


fn valid_transition_pair() -> impl Strategy<Value = (String, String)> {
    (valid_state_name(), valid_state_name())
        .prop_filter(
            "source and target must be different",
            |(source, target)| source != target,
        )
}

fn valid_transition_pairs()
    -> impl Strategy<Value = Vec<(String, String)>>
{
    proptest::collection::vec(
        valid_transition_pair(),
        1..20,
    )
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
        (source, target) in valid_transition_pair(),
    ) {
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
    
    #[test]
    fn transition_count_matches_iteration(
        transitions in valid_transition_pairs(),
    ) {
        let mut builder = Machine::builder();

        for (source, target) in &transitions {
            builder = builder
                .try_allow(source, target)
                .expect("generated transitions are valid");
        }

        let machine = builder
            .build()
            .expect("at least one transition was generated");

        prop_assert_eq!(
            machine.transition_count(),
            machine.transitions().count(),
        );
    }

    #[test]
    fn every_exposed_transition_is_queryable(
        transitions in valid_transition_pairs(),
    ) {
        let mut builder = Machine::builder();

        for (source, target) in &transitions {
            builder = builder
                .try_allow(source, target)
                .expect("generated transitions are valid");
        }

        let machine = builder
            .build()
            .expect("at least one transition was generated");

        for transition in machine.transitions() {
            prop_assert!(
                machine.can_transition(
                    transition.source(),
                    transition.target(),
                )
            );
        }
    }
    
    #[test]
    fn every_exposed_source_is_an_existing_state(
        transitions in valid_transition_pairs(),
    ) {
        let mut builder = Machine::builder();

        for (source, target) in &transitions {
            builder = builder
                .try_allow(source, target)
                .expect("generated transitions are valid");
        }

        let machine = builder
            .build()
            .expect("at least one transition was generated");
        
        let states: Vec<_> = machine.states().collect();

        for source in machine.sources() {
            prop_assert!(states.contains(&source));
        }

    }

    #[test]
    fn every_transition_target_is_in_targets_from(
        transitions in valid_transition_pairs(),
    ) {
        let mut builder = Machine::builder();

        for (source, target) in &transitions {
            builder = builder
                .try_allow(source, target)
                .expect("generated transitions are valid");
        }

        let machine = builder
            .build()
            .expect("at least one transition was generated");

        for transition in machine.transitions() {
            prop_assert!(machine.targets_from(transition.source()).any(|target| target == transition.target()));
        }
    }

    #[test]
    fn every_target_in_targets_from_is_a_valid_transition(
        transitions in valid_transition_pairs(),
    ) {
        let mut builder = Machine::builder();

        for (source, target) in &transitions {
            builder = builder
                .try_allow(source, target)
                .expect("generated transitions are valid");
        }

        let machine = builder
            .build()
            .expect("at least one transition was generated");

        for source in machine.sources() {
            prop_assert!(machine.targets_from(source).all(|target| machine.can_transition(source, target)));
        }
    }
}
