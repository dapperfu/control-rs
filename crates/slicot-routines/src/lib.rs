#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Routine identifiers and shared metadata for pure-Rust SLICOT ports.

mod routine_id;

pub use routine_id::{RoutineId, PHASE_ONE_ROUTINES};
