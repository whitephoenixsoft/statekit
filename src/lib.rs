#![deny(missing_docs)]

//! # Statekit
//!
//! A lightweight, immutable structure for validating, inspecting, and analyzing
//! directed state relationships.
//!
//! This crate is useful when states are dynamic, configuration-driven, or stored
//! as data. This allows applications to avoid hard-coding state transitions, and
//! directed relationships, such as control flows, dependencies, and paths.
//!
//! State and transition invariants are enforced while constructing the machine
//! through [`MachineBuilder`].
//!
//! # Examples
//! ```
//! use statekit::{Machine, StateError};
//!
//! fn main() -> Result<(), StateError> {
//!     let machine = Machine::builder()
//!         .try_allow("queued", "running")?
//!         .try_allow("running", "completed")?
//!         .try_allow("running", "failed")?
//!         .build()?;
//!
//!     machine.validate_transition("queued", "running")?;
//!
//!     assert!(machine.can_transition("queued", "running"));
//!     assert!(!machine.can_transition("queued", "completed"));
//!
//!     let mut instance = machine.instance("queued")?;
//!
//!     instance.transition_to("running")?;
//!
//!     assert_eq!(instance.state(), "running");
//!     assert!(instance.can_transition_to("completed"));
//!     assert!(!instance.can_transition_to("queued"));
//!
//!     for transition in machine.transitions() {
//!         println!("{} -> {}", transition.source(), transition.target());
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Invariants
//!
//! - State names must not be empty or consist entirely of whitespace.
//! - State names must not begin or end with Unicode whitespace.
//! - State names are case-sensitive.
//! - Cycles are permitted, including self transitions.
//! - Duplicate transitions between the same source and target are stored as a single logical transition.
//! - Transitions with non-existing edges are rejected.
//! - Instances must be initialized with an existing state.
//! - Instances can only transition to adjacent states.
//!
//! # Additional documentation
//!
//! See the project README for the specification, migration guide, changelog,
//! and benchmark documentation.
mod builder;
mod error;
mod instance;
mod machine;
mod transition;

mod inner;
mod state_name;
mod transitions;

pub use builder::MachineBuilder;
pub use error::StateError;
pub use instance::MachineInstance;
pub use machine::Machine;
pub use transition::Transition;

pub(crate) use inner::MachineInner;
pub(crate) use state_name::StateName;
pub(crate) use transitions::Transitions;
