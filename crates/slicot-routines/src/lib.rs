#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Routine identifiers and shared metadata for pure-Rust SLICOT ports.

mod compatibility;
mod module_map;
mod routine_id;

pub use compatibility::{phase_one_compatibility, PythonControlUsage};
pub use module_map::{target_rust_module_for_stem, TargetRustModule};
pub use routine_id::{RoutineId, PHASE_ONE_ROUTINES};
