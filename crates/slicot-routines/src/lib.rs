#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Routine identifiers and shared metadata for pure-Rust SLICOT ports.

mod ab07nd;
mod ab08nd;
mod control_utils;
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
mod sb03qd;
mod sb03sd;
mod sb03td;
mod sb03ud;
mod sb04md;
mod sb04nd;
mod sb04qd;
mod sb04rd;
mod sb10ad;
mod sb10hd;
mod sg02ad;
mod sg03ad;
mod sg03bd;
mod tb01pd;
mod tb04ad;
mod tb05ad;
mod td04ad;

pub use compatibility::{phase_one_compatibility, PythonControlUsage};
pub use control_utils::{
    controllability_gramian, controllability_matrix, dc_gain, h2_norm, is_controllable,
    is_observable, is_stable_continuous, is_stable_discrete, linf_norm, matrix_rank,
    observability_gramian, observability_matrix, open_loop_poles, ss_num_inputs, ss_num_outputs,
    ss_order, ControlUtilsError,
};
pub use mb03rd::{mb03rd_block_diagonalize, Mb03RdError, Mb03RdResult};
pub use module_map::{target_rust_module_for_stem, TargetRustModule};
pub use ab07nd::{ab07nd_inverse, Ab07NdError, Ab07NdResult};
pub use ab08nd::{ab08nd_zeros, Ab08NdError, Ab08NdResult};
pub use ab09ad::{ab09ad_balance_truncate, Ab09AdError, Ab09AdResult};
pub use ab09md::{ab09md_balance_truncate, Ab09MdError, Ab09MdResult};
pub use ab09nd::{ab09nd_spa, Ab09NdError, Ab09NdResult};
pub use ab13bd::{ab13bd_norm, Ab13BdError};
pub use ab13dd::{ab13dd_norm, ab13dd_norm_descriptor, Ab13DdError, Ab13DdResult};
pub use ab13md::{ab13md_norm, Ab13MdError};
pub use routine_id::{RoutineId, PHASE_ONE_ROUTINES, PHASE_TWO_ROUTINES};
pub use sb01bd::{sb01bd_place, Sb01BdError, Sb01BdResult};
pub use sb02md::{sb02md_solve, Sb02MdError, Sb02MdResult};
pub use sb02mt::{sb02mt_transform, Sb02MtError, Sb02MtResult};
pub use sb03md::{sb03md_solve, Sb03MdError, Sb03MdResult};
pub use sb03od::{sb03od_factor, Sb03OdError, Sb03OdResult};
pub use sb04md::{sb04md_solve, Sb04MdError, Sb04MdResult};
pub use sb04qd::{sb04pd_solve, sb04qd_solve, Sb04QdError, Sb04QdResult};
pub use sb10ad::{sb10ad_hinfsyn, Sb10AdError, Sb10AdResult};
pub use sb10hd::{sb10hd_h2syn, Sb10HdError, Sb10HdResult};
pub use sg02ad::{sg02ad_solve, Sg02AdError, Sg02AdResult};
pub use sg03ad::{sg03ad_solve, Sg03AdError, Sg03AdResult};
pub use sg03bd::{sg03bd_solve, Sg03BdError, Sg03BdResult};
pub use tb01pd::{tb01pd_minreal, Tb01PdError, Tb01PdResult};
pub use tb04ad::{tb04ad_transfer_matrix, Tb04AdError, Tb04AdResult, Tb04AdTransferPolynomial};
pub use tb05ad::{
    tb05ad_frequency_response, tb05ad_frequency_response_descriptor, Tb05AdError, Tb05AdResult,
};
pub use td04ad::{td04ad_tf2ss, Td04AdError, Td04AdResult};
pub use sb03qd::{sb03qd_solve, Sb03QdError, Sb03QdResult};
pub use sb03sd::{sb03sd_solve, Sb03SdError, Sb03SdResult};
pub use sb03td::{sb03td_solve, Sb03TdError, Sb03TdResult};
pub use sb03ud::{sb03ud_solve, Sb03UdError, Sb03UdResult};
pub use sb04nd::{sb04nd_solve, Sb04NdError, Sb04NdResult};
pub use sb04rd::{sb04rd_solve, Sb04RdError, Sb04RdResult};
