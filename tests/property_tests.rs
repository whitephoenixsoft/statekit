use std::collections::HashSet;
use proptest::prelude::*;

use statekit::{Machine, StateError};

type Model = HashSet<(String, String)>;

fn build_machine(transitions: &[(String, String)]) -> Machine {
    let mut builder = Machine::builder();

    for (source, target) in transitions {
        builder = builder
            .try_allow(source, target)
            .expect("generated transitions are valid");
    }

    builder
        .build()
        .expect("at least one transition was generated")
}

fn build_model(transitions: &[(String, String)]) -> Model {
    transitions.iter().cloned().collect()
}

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

fn transitions_with_existing_probe() -> impl Strategy<Value = (Vec<(String, String)>, (String, String))> {
    valid_transition_pairs()
        .prop_flat_map(|transitions| {
            let len = transitions.len();

            (Just(transitions), 0..len)
        })
        .prop_map(|(transitions, index)| {
            let probe = transitions[index].clone();

            (transitions, probe)
        })
}

fn transitions_with_missing_probe() -> impl Strategy<Value = (Vec<(String, String)>, (String, String))> {
    valid_transition_pairs()
        .prop_flat_map(|transitions| {
            (
                Just(transitions),
                valid_transition_pair(),
            )
        })
        .prop_filter(
            "probe must not already exist",
            |(transitions, probe)| !transitions.contains(probe),
        )
}

fn transitions_with_existing_source()
    -> impl Strategy<Value = (Vec<(String, String)>, String)>
{
    valid_transition_pairs()
        .prop_flat_map(|transitions| {
            (
                Just(transitions),
                valid_state_name(),
            )
        })
        .prop_filter(
            "source must have no outgoing transitions",
            |(transitions, source)| {
                !transitions
                    .iter()
                    .any(|(candidate, _)| candidate == source)
            },
        )
}

fn transitions_with_source_without_outgoing_transitions()
    -> impl Strategy<Value = (Vec<(String, String)>, String)>
{
    valid_transition_pairs()
        .prop_flat_map(|transitions| {
            (
                Just(transitions),   
                valid_transition_pair()
            )
        })
        .prop_filter(
            "source must not already exist",
            |(transition, probe)| !transition.iter().any(|(candidate, _)| candidate == &probe.0)
        )
        .prop_map(|(transitions, probe)| {
            let source = probe.0.clone(); 
            (transitions, source)
        })
}

fn transitions_with_missing_state()
    -> impl Strategy<Value = (Vec<(String, String)>, (String, String))>
{
    valid_transition_pairs()
        .prop_flat_map(|transitions| {
            (
                Just(transitions),   
                valid_transition_pair()
            )
        })
        .prop_filter(
            "states must not already exist",
            |(transitions, probe)| {
                !transitions.iter().any(|(source, target)| {
                    source == &probe.0
                        || target == &probe.0
                        || source == &probe.1
                        || target == &probe.1
                })
            }
        )
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
    fn whitespace_only_target_is_rejected(
        source in valid_state_name(),
        target in whitespace_only_state_name(),
    ) {
        let result = Machine::builder()
            .try_allow(&source, &target);
    
        prop_assert!(matches!(
            result,
            Err(StateError::EmptyState)
        ));
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
        let machine = build_machine(&transitions);

        prop_assert_eq!(
            machine.transition_count(),
            machine.transitions().count(),
        );
    }

    #[test]
    fn every_exposed_transition_is_queryable(
        transitions in valid_transition_pairs(),
    ) {
        let machine = build_machine(&transitions);

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
        let machine = build_machine(&transitions);

        let states: Vec<_> = machine.states().collect();

        for source in machine.sources() {
            prop_assert!(states.contains(&source));
        }

    }

    #[test]
    fn every_transition_target_is_in_targets_from(
        transitions in valid_transition_pairs(),
    ) {
        let machine = build_machine(&transitions);

        for transition in machine.transitions() {
            prop_assert!(machine.targets_from(transition.source()).any(|target| target == transition.target()));
        }
    }

    #[test]
    fn every_target_in_targets_from_is_a_valid_transition(
        transitions in valid_transition_pairs(),
    ) {
        let machine = build_machine(&transitions);

        for source in machine.sources() {
            let mut targets = machine.targets_from(source).peekable();

            prop_assert!(targets.peek().is_some());

            prop_assert!(targets.all(|target| machine.can_transition(source, target)));
        }
    }

    #[test]
    fn model_based_valid_transitions_yields_equivalent_transition_count(
        transitions in valid_transition_pairs(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);

        prop_assert_eq!(machine.transition_count(), model.len());
    }

    #[test]
    fn model_and_machine_agree_on_existing_transition(
        (transitions, probe) in transitions_with_existing_probe(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);

        let (source, target) = &probe;

        prop_assert!(model.contains(&probe));

        prop_assert_eq!(machine.can_transition(source, target), model.contains(&probe));
    }

    #[test]
    fn model_and_machine_agree_on_missing_transition(
        (transitions, probe) in transitions_with_missing_probe(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);

        let (source, target) = &probe;

        prop_assert!(!model.contains(&probe));

        prop_assert_eq!(machine.can_transition(source, target), model.contains(&probe));
    }

    #[test]
    fn model_and_machine_agree_on_states(
        transitions in valid_transition_pairs(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);

        let model_states: HashSet<&str> = model
            .iter()
            .flat_map(|(source, target)| {
                [source.as_str(), target.as_str()]
            })
            .collect();

        let machine_states: HashSet<&str> =
            machine.states().collect();

        prop_assert_eq!(machine_states, model_states);
    }

    #[test]
    fn model_and_machine_agree_on_sources(
        transitions in valid_transition_pairs(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);

        let model_sources: HashSet<&str> = model
            .iter()
            .map(|(source, _)| source.as_str())
            .collect();

        let machine_sources: HashSet<&str> =
            machine.sources().collect();

        prop_assert_eq!(machine_sources, model_sources);
    }

    #[test]
    fn model_and_machine_agree_on_targets_from(
        transitions in valid_transition_pairs(),
        source in valid_state_name(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);

        let model_targets: HashSet<&str> = model
            .iter()
            .filter(|(candidate_source, _)| candidate_source == &source)
            .map(|(_, target)| target.as_str())
            .collect();

        let machine_targets: HashSet<&str> =
            machine.targets_from(&source).collect();

        prop_assert_eq!(machine_targets, model_targets);
    }

    #[test]
    fn model_and_machine_agree_on_targets_for_existing_source(
        (transitions, source) in transitions_with_existing_source(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);

        let model_targets: HashSet<&str> = model
            .iter()
            .filter(|(candidate_source, _)| candidate_source == &source)
            .map(|(_, target)| target.as_str())
            .collect();

        let machine_targets: HashSet<&str> =
            machine.targets_from(&source).collect();

        prop_assert!(!model_targets.is_empty());
        prop_assert_eq!(machine_targets, model_targets);
    }

    #[test]
    fn model_and_machine_agree_on_targets_for_source_without_outgoing_transitions(
        (transitions, source) in transitions_with_source_without_outgoing_transitions(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);

        let model_targets: HashSet<&str> = model
            .iter()
            .filter(|(candidate_source, _)| candidate_source == &source)
            .map(|(_, target)| target.as_str())
            .collect();

        let machine_targets: HashSet<&str> =
            machine.targets_from(&source).collect();

        prop_assert!(model_targets.is_empty());
        prop_assert_eq!(machine_targets, model_targets);
    }
    
    #[test]
    fn model_and_machine_agree_on_transitions(
        transitions in valid_transition_pairs(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);
    
        let machine_transitions: Model = machine
            .transitions()
            .map(|transition| {
                (
                    transition.source().to_owned(),
                    transition.target().to_owned(),
                )
            })
            .collect();
    
        prop_assert_eq!(machine_transitions, model);
    }
    
    #[test]
    fn model_and_machine_agree_on_valid_transition(
        (transitions, probe) in transitions_with_existing_probe(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);
        let (source, target) = &probe;
    
        prop_assert_eq!(
            machine.validate_transition(source, target).is_ok(),
            model.contains(&probe),
        );
    }
    
    #[test]
    fn model_and_machine_agree_on_invalid_transition(
        (transitions, probe) in transitions_with_missing_probe(),
    ) {
        let machine = build_machine(&transitions);
        let model = build_model(&transitions);
    
        let (source, target) = &probe;
    
        prop_assert!(!model.contains(&probe));
    
        prop_assert_eq!(
            machine.validate_transition(source, target).is_ok(),
            model.contains(&probe),
        );
    }
    
    #[test]
    fn contains_state_recognizes_existing_endpoint(
        (transitions, probe) in transitions_with_existing_probe(),
    ) {
        let machine = build_machine(&transitions);
    
        let (source, target) = &probe;
    
        prop_assert!(machine.contains_state(source));
        prop_assert!(machine.contains_state(target));
    }
    
    #[test]
    fn contains_state_recognizes_missing_endpoint(
        (transitions, probe) in transitions_with_missing_state(),
    ) {
        let machine = build_machine(&transitions);
    
        let (source, target) = &probe;
    
        prop_assert!(!machine.contains_state(source));
        prop_assert!(!machine.contains_state(target));
    }

    #[test]
    fn duplicate_transitions_collapse(
        (source, target) in valid_transition_pair(),
        repetitions in 2usize..20,
    ) {
        let mut builder = Machine::builder();

        for _ in 0..repetitions {
            builder = builder
                .try_allow(&source, &target)
                .expect("generated transition is valid");
        }

        let machine = builder
            .build()
            .expect("at least one transition was generated");

        prop_assert_eq!(machine.transition_count(), 1);
        prop_assert!(machine.can_transition(&source, &target));
    }

    
    #[test]
    fn accepted_arbitrary_transitions_preserve_invariants(
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
                .expect("successful construction contains a transition");

            prop_assert!(!transition.source().trim().is_empty());
            prop_assert!(!transition.target().trim().is_empty());

            prop_assert_eq!(
                transition.source(),
                transition.source().trim(),
            );

            prop_assert_eq!(
                transition.target(),
                transition.target().trim(),
            );

            prop_assert_ne!(
                transition.source(),
                transition.target(),
            );
        }
    }
}
