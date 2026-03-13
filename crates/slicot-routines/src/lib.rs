#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Routine identifiers and shared metadata for pure-Rust SLICOT ports.

mod compatibility;
mod module_map;
mod routine_id;
mod sb03md;
mod sb04md;
mod tb05ad;

pub use compatibility::{phase_one_compatibility, PythonControlUsage};
pub use module_map::{target_rust_module_for_stem, TargetRustModule};
pub use routine_id::{RoutineId, PHASE_ONE_ROUTINES};
pub use sb03md::{sb03md_solve, Sb03MdError, Sb03MdResult};
pub use sb04md::{sb04md_solve, Sb04MdError, Sb04MdResult};
pub use tb05ad::{tb05ad_frequency_response, Tb05AdError, Tb05AdResult};
