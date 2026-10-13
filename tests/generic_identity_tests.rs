use statekit::generic_identity::{IdentityError, Machine as IdentityMachine};
use statekit::{Machine, StateError};
use proptest::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Phase {
    Idle,
    Running,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NumericState(u32);

#[test]
fn experimental_string_machine_keeps_validated_borrowed_surface() -> Result<(), StateError> {
    let machine = IdentityMachine::builder()
        .try_allow("idle", "running")
        .expect("valid names")
        .try_allow("running", "done")
        .expect("valid names")
        .build()
        .expect("non-empty machine");

    assert!(machine.can_transition("idle", "running"));
    assert_eq!(machine.targets_from("idle").collect::<Vec<_>>(), vec!["running"]);
    assert!(matches!(
        IdentityMachine::builder().try_allow(" idle", "running"),
        Err(IdentityError::AmbiguousStateName { state }) if state == " idle"
    ));

    let root = Machine::builder()
        .try_allow("idle", "running")?
        .build()?;
    assert!(root.can_transition("idle", "running"));
    Ok(())
}

#[test]
fn enum_identity_rejects_self_edges_and_duplicates() {
    let result = IdentityMachine::<Phase>::builder_with_identity()
        .try_allow_identity(Phase::Idle, Phase::Idle);
    assert!(matches!(result, Err(IdentityError::SelfTransition { state: Phase::Idle })));

    let machine = IdentityMachine::<Phase>::builder_with_identity()
        .try_allow_identity(Phase::Idle, Phase::Running)
        .expect("distinct states")
        .try_allow_identity(Phase::Idle, Phase::Running)
        .expect("duplicate edge is idempotent")
        .build()
        .expect("one edge is non-empty");

    assert_eq!(machine.transition_count(), 1);
    assert!(machine.can_transition_identity(&Phase::Idle, &Phase::Running));
}

#[test]
fn numeric_newtype_identity_supports_borrowed_structure_iteration() {
    let machine = IdentityMachine::<NumericState>::builder_with_identity()
        .try_allow_identity(NumericState(1), NumericState(2))
        .expect("distinct states")
        .try_allow_identity(NumericState(2), NumericState(3))
        .expect("distinct states")
        .build()
        .expect("non-empty machine");

    assert!(machine.contains_state_identity(&NumericState(3)));
    assert_eq!(machine.targets_from_identity(&NumericState(1)).count(), 1);
    assert_eq!(
        machine.transitions().next().unwrap().source_identity(),
        &NumericState(1)
    );
}

#[test]
fn equality_is_logical_and_independent_of_insertion_order() {
    let first = IdentityMachine::<Phase>::builder_with_identity()
        .try_allow_identity(Phase::Idle, Phase::Running)
        .expect("distinct states")
        .try_allow_identity(Phase::Running, Phase::Done)
        .expect("distinct states")
        .build()
        .expect("non-empty machine");
    let second = IdentityMachine::<Phase>::builder_with_identity()
        .try_allow_identity(Phase::Running, Phase::Done)
        .expect("distinct states")
        .try_allow_identity(Phase::Idle, Phase::Running)
        .expect("distinct states")
        .build()
        .expect("non-empty machine");

    assert_eq!(first, second);
}

#[test]
fn instances_are_independent_and_failed_moves_are_atomic() {
    let machine = IdentityMachine::<Phase>::builder_with_identity()
        .try_allow_identity(Phase::Idle, Phase::Running)
        .expect("distinct states")
        .try_allow_identity(Phase::Running, Phase::Done)
        .expect("distinct states")
        .build()
        .expect("non-empty machine");
    let mut left = machine.instance_identity(Phase::Idle).expect("known state");
    let right = machine.instance_identity(Phase::Idle).expect("known state");

    let result = left.transition_to_identity(Phase::Done);
    assert!(matches!(result, Err(IdentityError::InvalidTransition { .. })));
    assert_eq!(left.state_identity(), &Phase::Idle);

    left.transition_to_identity(Phase::Running).expect("allowed edge");
    assert_eq!(left.state_identity(), &Phase::Running);
    assert_eq!(right.state_identity(), &Phase::Idle);
    assert_eq!(left.machine(), right.machine());
}

#[test]
fn custom_identity_type_does_not_require_debug_for_topology_use() {
    struct Quiet(u8);
    impl PartialEq for Quiet {
        fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
    }
    impl Eq for Quiet {}
    impl std::hash::Hash for Quiet {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            std::hash::Hash::hash(&self.0, state);
        }
    }

    let machine = IdentityMachine::<Quiet>::builder_with_identity()
        .try_allow_identity(Quiet(1), Quiet(2))
        .unwrap_or_else(|_| panic!("valid edge"))
        .build()
        .unwrap_or_else(|_| panic!("non-empty machine"));

    assert!(machine.can_transition_identity(&Quiet(1), &Quiet(2)));
}

#[test]
fn generic_builder_rejects_empty_machine_and_unknown_initial_state() {
    let empty = IdentityMachine::<Phase>::builder_with_identity().build();
    assert!(matches!(empty, Err(IdentityError::NoTransitions)));

    let machine = IdentityMachine::<Phase>::builder_with_identity()
        .try_allow_identity(Phase::Idle, Phase::Running)
        .expect("distinct states")
        .build()
        .expect("non-empty machine");
    let missing = machine.instance_identity(Phase::Done);
    assert!(matches!(
        missing,
        Err(IdentityError::UnknownInitialState { state: Phase::Done })
    ));
}

proptest! {
    #[test]
    fn generic_membership_matches_a_transition_set_model(
        raw in prop::collection::vec((0u8..8, 0u8..8), 1..50)
            .prop_filter("include at least one non-self transition", |edges| {
                edges.iter().any(|(from, to)| from != to)
            })
    ) {
        let mut builder = IdentityMachine::<NumericState>::builder_with_identity();
        let mut model = HashSet::new();

        for (from, to) in raw {
            if from == to {
                continue;
            }
            model.insert((from, to));
            builder = builder
                .try_allow_identity(NumericState(from as u32), NumericState(to as u32))
                .expect("distinct generated states");
        }

        let machine = builder.build().expect("at least one generated edge");
        for from in 0u8..8 {
            for to in 0u8..8 {
                prop_assert_eq!(
                    machine.can_transition_identity(
                        &NumericState(from as u32),
                        &NumericState(to as u32),
                    ),
                    model.contains(&(from, to)),
                );
            }
        }
    }
}
