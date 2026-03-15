#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Routine identifiers and shared metadata for pure-Rust SLICOT ports.

mod ab13bd;
mod compatibility;
mod module_map;
mod routine_id;
mod sb02md;
mod sb03md;
mod sb03od;
mod sb04md;
mod sb04qd;
mod sg03ad;
mod tb04ad;
mod tb05ad;

pub use compatibility::{phase_one_compatibility, PythonControlUsage};
pub use module_map::{target_rust_module_for_stem, TargetRustModule};
pub use ab13bd::{ab13bd_norm, Ab13BdError};
pub use routine_id::{RoutineId, PHASE_ONE_ROUTINES};
pub use sb02md::{sb02md_solve, Sb02MdError, Sb02MdResult};
pub use sb03md::{sb03md_solve, Sb03MdError, Sb03MdResult};
pub use sb03od::{sb03od_factor, Sb03OdError, Sb03OdResult};
pub use sb04md::{sb04md_solve, Sb04MdError, Sb04MdResult};
pub use sb04qd::{sb04qd_solve, Sb04QdError, Sb04QdResult};
pub use sg03ad::{sg03ad_solve, Sg03AdError, Sg03AdResult};
pub use tb04ad::{tb04ad_transfer_matrix, Tb04AdError, Tb04AdResult, Tb04AdTransferPolynomial};
pub use tb05ad::{tb05ad_frequency_response, Tb05AdError, Tb05AdResult};
