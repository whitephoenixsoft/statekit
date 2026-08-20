#![deny(missing_docs)]

//! # Statekit
//!
//! An immutable state-transition validator for predefined workflows.
//!
//! This crate is useful when states are dynamic, configuration-driven, or stored
//! as data, allowing applications to avoid hard-coding state transitions.
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
//!     assert!(machine.can_transition("queued", "running"));
//!     assert!(!machine.can_transition("queued", "completed"));
//!
//!     machine.validate_transition("queued", "running")?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Invariants
//!
//! - A machine contains at least one transition.
//! - State names are non-empty.
//! - State names may not begin or end with Unicode whitespace.
//! - Self-transitions are rejected.
//! - Cycles between distinct states are permitted.
//!
//! # Additional documentation
//!
//! - [Statekit Specification](https://github.com/whitephoenixsoft/statekit/blob/main/docs/statekit-specification.md)
//! - [Migration Guide](https://github.com/whitephoenixsoft/statekit/blob/main/MIGRATION.md)
//! - [Change Log](https://github.com/whitephoenixsoft/statekit/blob/main/CHANGELOG.md)
mod builder;
mod error;
mod machine;
mod state_name;
mod transition;

pub use builder::MachineBuilder;
pub use error::StateError;
pub use machine::Machine;
pub use transition::Transition;
pub(crate) use state_name::StateName;
