//#![deny(missing_docs)]
//! Statekit
//! # Example Code
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
//!     let state = machine.transition("queued", "running");
//!
//!     assert!(state.is_ok());
//!
//!     Ok(())
//! }
//! ```
mod builder;
mod error;
mod machine;
mod model;

pub use model::StateName;
pub use builder::MachineBuilder;
pub use error::StateError;
pub use machine::Machine;

