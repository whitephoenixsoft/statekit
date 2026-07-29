#![deny(missing_docs)]
//! Statekit
//!     A simple immutable state-machine that validates allowed predefined transitions.
//!
//! This crate is useful for cases when states are dynamic or when they exist as data
//! so that the application does not have to couple itself to the states.
//!
//! # Examples
//! ```
//! use statekit::{Machine, StateError};
//!
//! fn main() -> Result<(), StateError> {
//!     let machine = Machine::builder()
//!         .allow("queued", "running")
//!         .allow("running", "completed")
//!         .allow("running", "failed")
//!         .build()?;
//!
//!     assert!(machine.can_transition("queued", "running"));
//!     assert!(!machine.can_transition("queued", "completed"));
//!
//!  machine.validate_transition("queued", "running")?;
//!
//!     Ok(())
//! }
//! ```
mod builder;
mod error;
mod machine;
mod state_name;

pub use builder::MachineBuilder;
pub use error::StateError;
pub use machine::Machine;
pub(crate) use state_name::StateName;
