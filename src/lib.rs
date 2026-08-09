#![deny(missing_docs)]

//! # Statekit
//!
//! An immutable state-machine definition for validating predefined transitions.
//!
//! This crate is useful when states are dynamic, configuration-driven, or stored
//! as data, allowing applications to avoid hard-coding state transitions.
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
//! - State names may not begin or end with whitespace.
//! - Self-transitions are rejected.
//! - Cycles between distinct states are permitted.
mod builder;
mod error;
mod machine;
mod state_name;

pub use builder::MachineBuilder;
pub use error::StateError;
pub use machine::Machine;
pub(crate) use state_name::StateName;
