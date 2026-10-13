//! Experimental generic state identity API for compatibility research.
//!
//! This module is intentionally separate from the crate-root API. It tests
//! consumer-defined identities without changing the existing `Machine` type.
//! It is not a production API commitment.

use crate::{StateError, StateName};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::sync::Arc;

/// An immutable machine whose states use a consumer-defined identity type.
///
/// The default identity is `String`. Prefer [`Machine::builder`] for validated
/// string names, or [`Machine::builder_with_identity`] for custom identities.
#[derive(Debug, PartialEq, Eq)]
pub struct Machine<S = String> {
    transitions: Arc<HashSet<Transition<S>>>,
}

impl Machine<String> {
    /// Creates a builder for validated string state names.
    pub fn builder() -> MachineBuilder<String> {
        MachineBuilder::new()
    }

    /// Creates an instance at an existing state name.
    pub fn instance(&self, initial: &str) -> Result<MachineInstance<String>, StateError> {
        if !self.contains_state(initial) {
            return Err(StateError::UnknownInitialState {
                state: initial.to_owned(),
            });
        }

        Ok(MachineInstance {
            transitions: Arc::clone(&self.transitions),
            current: initial.to_owned(),
        })
    }

    /// Returns whether a string-named state appears in the machine.
    pub fn contains_state(&self, state: &str) -> bool {
        self.transitions.iter().any(|edge| {
            edge.source == state || edge.target == state
        })
    }

    /// Returns whether a string-named transition is allowed.
    pub fn can_transition(&self, from: &str, to: &str) -> bool {
        self.transitions
            .iter()
            .any(|edge| edge.source == from && edge.target == to)
    }

    /// Validates a string-named transition.
    pub fn validate_transition(&self, from: &str, to: &str) -> Result<(), StateError> {
        if self.can_transition(from, to) {
            Ok(())
        } else {
            Err(StateError::InvalidTransition {
                from: from.to_owned(),
                to: to.to_owned(),
            })
        }
    }

    /// Returns the state names as borrowed string slices.
    ///
    /// Iteration order is unspecified.
    pub fn states(&self) -> impl Iterator<Item = &str> {
        let mut states = HashSet::new();
        for edge in self.transitions.iter() {
            states.insert(edge.source.as_str());
            states.insert(edge.target.as_str());
        }
        states.into_iter()
    }

    /// Returns the distinct source names as borrowed string slices.
    ///
    /// Iteration order is unspecified.
    pub fn sources(&self) -> impl Iterator<Item = &str> {
        let mut states = HashSet::new();
        for edge in self.transitions.iter() {
            states.insert(edge.source.as_str());
        }
        states.into_iter()
    }

    /// Returns the outgoing target names for `from` as borrowed string slices.
    ///
    /// Iteration order is unspecified.
    pub fn targets_from(&self, from: &str) -> impl Iterator<Item = &str> {
        self.transitions
            .iter()
            .filter(move |edge| edge.source == from)
            .map(|edge| edge.target.as_str())
    }
}

impl<S> Machine<S>
where
    S: Eq + Hash,
{
    /// Creates a builder for consumer-defined identities.
    ///
    /// This distinct name preserves the inference and validation path of the
    /// root `Machine::builder()` API for ordinary string consumers.
    pub fn builder_with_identity() -> MachineBuilder<S> {
        MachineBuilder::new()
    }

    /// Returns the number of unique transitions.
    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// Returns borrowed transition records.
    pub fn transitions(&self) -> impl Iterator<Item = &Transition<S>> {
        self.transitions.iter()
    }

    /// Returns whether `state` appears as either endpoint.
    pub fn contains_state_identity(&self, state: &S) -> bool {
        self.transitions
            .iter()
            .any(|edge| &edge.source == state || &edge.target == state)
    }

    /// Returns whether the directed transition exists.
    pub fn can_transition_identity(&self, from: &S, to: &S) -> bool {
        self.transitions
            .iter()
            .any(|edge| &edge.source == from && &edge.target == to)
    }

    /// Returns an error when the directed transition is not present.
    pub fn validate_transition_identity(
        &self,
        from: &S,
        to: &S,
    ) -> Result<(), IdentityError<S>>
    where
        S: Clone,
    {
        if self.can_transition_identity(from, to) {
            Ok(())
        } else {
            Err(IdentityError::InvalidTransition {
                from: from.clone(),
                to: to.clone(),
            })
        }
    }

    /// Returns the states directly reachable from `from`.
    ///
    /// Iteration order is unspecified.
    pub fn targets_from_identity<'a>(&'a self, from: &'a S) -> impl Iterator<Item = &'a S> {
        self.transitions
            .iter()
            .filter(move |edge| &edge.source == from)
            .map(|edge| &edge.target)
    }

    /// Returns all distinct state identities.
    ///
    /// Iteration order is unspecified.
    pub fn states_identity(&self) -> impl Iterator<Item = &S> {
        let mut states = HashSet::new();
        for edge in self.transitions.iter() {
            states.insert(&edge.source);
            states.insert(&edge.target);
        }
        states.into_iter()
    }

    /// Returns all distinct source identities.
    ///
    /// Iteration order is unspecified.
    pub fn sources_identity(&self) -> impl Iterator<Item = &S> {
        let mut states = HashSet::new();
        for edge in self.transitions.iter() {
            states.insert(&edge.source);
        }
        states.into_iter()
    }

    /// Creates an independent instance at an existing identity.
    pub fn instance_identity(&self, initial: S) -> Result<MachineInstance<S>, IdentityError<S>> {
        if !self.contains_state_identity(&initial) {
            return Err(IdentityError::UnknownInitialState { state: initial });
        }

        Ok(MachineInstance {
            transitions: Arc::clone(&self.transitions),
            current: initial,
        })
    }

    /// Returns whether `state` has no outgoing transitions.
    pub fn is_terminal_identity(&self, state: &S) -> bool {
        self.targets_from_identity(state).next().is_none()
    }
}

/// A builder for a [`Machine`] in this experimental module.
#[derive(Debug)]
pub struct MachineBuilder<S = String> {
    transitions: HashSet<Transition<S>>,
}

impl<S> MachineBuilder<S> {
    fn new() -> Self {
        Self {
            transitions: HashSet::new(),
        }
    }
}

impl MachineBuilder<String> {
    /// Adds a transition between validated string names.
    pub fn try_allow(
        mut self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> Result<Self, IdentityError<String>> {
        let source = StateName::try_from(from.as_ref()).map_err(IdentityError::from)?;
        let target = StateName::try_from(to.as_ref()).map_err(IdentityError::from)?;
        let source = source.into_string();
        let target = target.into_string();

        if source == target {
            return Err(IdentityError::SelfTransition { state: source });
        }

        self.transitions.insert(Transition { source, target });
        Ok(self)
    }
}

impl<S> MachineBuilder<S>
where
    S: Eq + Hash,
{
    /// Adds a transition between consumer-defined state identities.
    pub fn try_allow_identity(
        mut self,
        source: S,
        target: S,
    ) -> Result<Self, IdentityError<S>> {
        if source == target {
            return Err(IdentityError::SelfTransition { state: source });
        }

        self.transitions.insert(Transition { source, target });
        Ok(self)
    }

    /// Builds an immutable machine, rejecting an empty transition set.
    pub fn build(self) -> Result<Machine<S>, IdentityError<S>> {
        if self.transitions.is_empty() {
            return Err(IdentityError::NoTransitions);
        }

        Ok(Machine {
            transitions: Arc::new(self.transitions),
        })
    }
}

/// A directed transition over a state identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transition<S> {
    source: S,
    target: S,
}

impl<S> Transition<S> {
    /// Returns the source identity.
    pub fn source_identity(&self) -> &S {
        &self.source
    }

    /// Returns the target identity.
    pub fn target_identity(&self) -> &S {
        &self.target
    }
}

impl Transition<String> {
    /// Returns the source name for a string-identity transition.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the target name for a string-identity transition.
    pub fn target(&self) -> &str {
        &self.target
    }
}

/// A mutable traversal position over an immutable generic machine.
#[derive(Debug, PartialEq, Eq)]
pub struct MachineInstance<S = String> {
    transitions: Arc<HashSet<Transition<S>>>,
    current: S,
}

impl<S> MachineInstance<S> {
    /// Returns the current identity by reference.
    pub fn state_identity(&self) -> &S {
        &self.current
    }

    /// Returns a handle to the shared immutable machine definition.
    pub fn machine(&self) -> Machine<S> {
        Machine {
            transitions: Arc::clone(&self.transitions),
        }
    }
}

impl<S> MachineInstance<S>
where
    S: Eq + Hash,
{
    /// Returns whether a transition to `target` is allowed.
    pub fn can_transition_to_identity(&self, target: &S) -> bool {
        self.transitions
            .iter()
            .any(|edge| edge.source == self.current && &edge.target == target)
    }

    /// Changes the current identity when the transition is allowed.
    pub fn transition_to_identity(&mut self, target: S) -> Result<(), IdentityError<S>>
    where
        S: Clone,
    {
        if !self.can_transition_to_identity(&target) {
            return Err(IdentityError::InvalidTransition {
                from: self.current.clone(),
                to: target,
            });
        }

        self.current = target;
        Ok(())
    }

    /// Returns whether the current state has no outgoing transitions.
    pub fn is_terminal_identity(&self) -> bool {
        !self
            .transitions
            .iter()
            .any(|edge| edge.source == self.current)
    }
}

impl MachineInstance<String> {
    /// Returns the current string state name.
    pub fn state(&self) -> &str {
        &self.current
    }

    /// Returns whether a transition to `target` is allowed.
    pub fn can_transition_to(&self, target: &str) -> bool {
        self.transitions
            .iter()
            .any(|edge| edge.source == self.current && edge.target == target)
    }

    /// Changes the current state when the transition is allowed.
    pub fn transition_to(&mut self, target: &str) -> Result<(), StateError> {
        if !self.can_transition_to(target) {
            return Err(StateError::InvalidTransition {
                from: self.current.clone(),
                to: target.to_owned(),
            });
        }

        self.current = target.to_owned();
        Ok(())
    }

    /// Returns whether the current state has no outgoing transitions.
    pub fn is_terminal(&self) -> bool {
        !self
            .transitions
            .iter()
            .any(|edge| edge.source == self.current)
    }
}

/// Errors returned by the experimental generic-identity API.
#[derive(Debug, PartialEq, Eq)]
pub enum IdentityError<S = String> {
    /// The machine must contain at least one transition.
    NoTransitions,
    /// A string name is empty or whitespace-only.
    EmptyState,
    /// A string name has leading or trailing whitespace.
    AmbiguousStateName {
        /// The offending string value.
        state: String,
    },
    /// A transition has equal source and target identities.
    SelfTransition {
        /// The identity used for both endpoints.
        state: S,
    },
    /// A requested transition is absent from the machine.
    InvalidTransition {
        /// The requested source identity.
        from: S,
        /// The requested target identity.
        to: S,
    },
    /// An initial identity does not occur in the machine.
    UnknownInitialState {
        /// The requested initial identity.
        state: S,
    },
}

impl From<StateError> for IdentityError<String> {
    fn from(error: StateError) -> Self {
        match error {
            StateError::NoTransitions => Self::NoTransitions,
            StateError::EmptyState => Self::EmptyState,
            StateError::AmbiguousStateName { state } => Self::AmbiguousStateName { state },
            StateError::SelfTransition { state } => Self::SelfTransition { state },
            StateError::InvalidTransition { from, to } => Self::InvalidTransition { from, to },
            StateError::UnknownInitialState { state } => Self::UnknownInitialState { state },
        }
    }
}

impl<S: fmt::Debug> fmt::Display for IdentityError<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoTransitions => write!(f, "state machine must define at least one transition"),
            Self::EmptyState => write!(f, "state name cannot be empty or whitespace-only"),
            Self::AmbiguousStateName { state } => write!(
                f,
                "state name {state:?} must not begin or end with Unicode whitespace"
            ),
            Self::SelfTransition { state } => {
                write!(f, "self-transitions are not allowed for state {state:?}")
            }
            Self::InvalidTransition { from, to } => {
                write!(f, "transition from {from:?} to {to:?} is not allowed")
            }
            Self::UnknownInitialState { state } => {
                write!(f, "instance initial state {state:?} does not exist")
            }
        }
    }
}

impl<S> Error for IdentityError<S>
where
    S: fmt::Debug + 'static,
{
}
