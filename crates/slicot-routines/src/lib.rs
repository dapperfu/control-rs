#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Routine identifiers and shared metadata for pure-Rust SLICOT ports.

mod ab08nd;
mod ab09ad;
mod ab09md;
mod ab09nd;
mod ab13bd;
mod ab13dd;
mod ab13md;
mod compatibility;
mod mb03rd;
mod module_map;
mod routine_id;
mod sb01bd;
mod sb02md;
mod sb02mt;
mod sb03md;
mod sb03od;
mod sb04md;
mod sb04qd;
mod sb10ad;
mod sb10hd;
mod sg02ad;
mod sg03ad;
mod tb01pd;
mod tb04ad;
mod tb05ad;
mod td04ad;

pub use compatibility::{phase_one_compatibility, PythonControlUsage};
pub use mb03rd::{mb03rd_block_diagonalize, Mb03RdError, Mb03RdResult};
pub use module_map::{target_rust_module_for_stem, TargetRustModule};
pub use ab08nd::{ab08nd_zeros, Ab08NdError, Ab08NdResult};
pub use ab09ad::{ab09ad_balance_truncate, Ab09AdError, Ab09AdResult};
pub use ab09md::{ab09md_balance_truncate, Ab09MdError, Ab09MdResult};
pub use ab09nd::{ab09nd_spa, Ab09NdError, Ab09NdResult};
pub use ab13bd::{ab13bd_norm, Ab13BdError};
pub use ab13dd::{ab13dd_norm, Ab13DdError, Ab13DdResult};
pub use ab13md::{ab13md_norm, Ab13MdError};
pub use routine_id::{RoutineId, PHASE_ONE_ROUTINES};
pub use sb01bd::{sb01bd_place, Sb01BdError, Sb01BdResult};
pub use sb02md::{sb02md_solve, Sb02MdError, Sb02MdResult};
pub use sb02mt::{sb02mt_transform, Sb02MtError, Sb02MtResult};
pub use sb03md::{sb03md_solve, Sb03MdError, Sb03MdResult};
pub use sb03od::{sb03od_factor, Sb03OdError, Sb03OdResult};
pub use sb04md::{sb04md_solve, Sb04MdError, Sb04MdResult};
pub use sb04qd::{sb04qd_solve, Sb04QdError, Sb04QdResult};
pub use sb10ad::{sb10ad_hinfsyn, Sb10AdError, Sb10AdResult};
pub use sb10hd::{sb10hd_h2syn, Sb10HdError, Sb10HdResult};
pub use sg02ad::{sg02ad_solve, Sg02AdError, Sg02AdResult};
pub use sg03ad::{sg03ad_solve, Sg03AdError, Sg03AdResult};
pub use tb01pd::{tb01pd_minreal, Tb01PdError, Tb01PdResult};
pub use tb04ad::{tb04ad_transfer_matrix, Tb04AdError, Tb04AdResult, Tb04AdTransferPolynomial};
pub use tb05ad::{tb05ad_frequency_response, Tb05AdError, Tb05AdResult};
pub use td04ad::{td04ad_tf2ss, Td04AdError, Td04AdResult};
