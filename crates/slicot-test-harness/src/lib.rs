#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Test harness utilities for mirroring the upstream SLICOT example corpus.

mod ab13bd;
mod catalog;
mod error;
mod inventory;
mod python_control;
mod sb02md;
mod sb03md;
mod sb03od;
mod sb03qd;
mod sb03sd;
mod sb03td;
mod sb03ud;
mod sb04md;
mod sb04nd;
mod sb04pd;
mod sb04qd;
mod sb04rd;
mod sg03ad;
mod sg03bd;
mod tb04ad;
mod tb05ad;

pub use ab13bd::{
    load_ab13bd_case, parse_ab13bd_input_file, parse_ab13bd_result_file, Ab13BdCase,
    Ab13BdExampleError, Ab13BdInput, Ab13BdOutput,
};
pub use catalog::{discover_example_cases, ExampleCase};
pub use error::CatalogError;
pub use inventory::{discover_routine_inventory, RoutineInventoryEntry};
pub use python_control::{
    phase_one_python_control_targets, resolve_phase_one_python_control_targets,
    PythonControlTarget, ResolvedPythonControlTarget,
};
pub use sb02md::{
    load_sb02md_case, parse_sb02md_input_file, parse_sb02md_result_file, Sb02MdCase,
    Sb02MdExampleError, Sb02MdInput, Sb02MdOutput,
};
pub use sb03md::{
    load_sb03md_case, parse_sb03md_input_file, parse_sb03md_result_file, Sb03MdCase,
    Sb03MdExampleError, Sb03MdInput, Sb03MdOutput,
};
pub use sb03od::{
    load_sb03od_case, parse_sb03od_input_file, parse_sb03od_result_file, Sb03OdCase,
    Sb03OdExampleError, Sb03OdInput, Sb03OdOutput,
};
pub use sb03sd::{
    load_sb03sd_case, parse_sb03sd_input_file, parse_sb03sd_result_file, Sb03SdCase,
    Sb03SdExampleError, Sb03SdInput, Sb03SdOutput,
};
pub use sb03qd::{
    load_sb03qd_case, parse_sb03qd_input_file, parse_sb03qd_result_file, Sb03QdCase,
    Sb03QdExampleError, Sb03QdInput, Sb03QdOutput,
};
pub use sb03td::{
    load_sb03td_case, parse_sb03td_input_file, parse_sb03td_result_file, Sb03TdCase,
    Sb03TdExampleError, Sb03TdInput, Sb03TdOutput,
};
pub use sb03ud::{
    load_sb03ud_case, parse_sb03ud_input_file, parse_sb03ud_result_file, Sb03UdCase,
    Sb03UdExampleError, Sb03UdInput, Sb03UdOutput,
};
pub use sb04md::{
    load_sb04md_case, parse_sb04md_input_file, parse_sb04md_result_file, Sb04MdCase,
    Sb04MdExampleError, Sb04MdInput, Sb04MdOutput,
};
pub use sb04nd::{
    load_sb04nd_case, parse_sb04nd_input_file, parse_sb04nd_result_file, Sb04NdCase,
    Sb04NdExampleError, Sb04NdInput, Sb04NdOutput,
};
pub use sb04pd::{
    load_sb04pd_case, parse_sb04pd_input_file, parse_sb04pd_result_file, Sb04PdCase,
    Sb04PdExampleError, Sb04PdInput, Sb04PdOutput,
};
pub use sb04qd::{
    load_sb04qd_case, parse_sb04qd_input_file, parse_sb04qd_result_file, Sb04QdCase,
    Sb04QdExampleError, Sb04QdInput, Sb04QdOutput,
};
pub use sb04rd::{
    load_sb04rd_case, parse_sb04rd_input_file, parse_sb04rd_result_file, Sb04RdCase,
    Sb04RdExampleError, Sb04RdInput, Sb04RdOutput,
};
pub use sg03ad::{
    load_sg03ad_case, parse_sg03ad_input_file, parse_sg03ad_result_file, Sg03AdCase,
    Sg03AdExampleError, Sg03AdInput, Sg03AdOutput,
};
pub use sg03bd::{
    load_sg03bd_case, parse_sg03bd_input_file, parse_sg03bd_result_file, Sg03BdCase,
    Sg03BdExampleError, Sg03BdInput, Sg03BdOutput,
};
pub use tb04ad::{
    load_tb04ad_case, parse_tb04ad_input_file, parse_tb04ad_result_file, Tb04AdCase,
    Tb04AdExampleError, Tb04AdInput, Tb04AdOutput, TransferPolynomial,
};
pub use tb05ad::{
    load_tb05ad_case, parse_tb05ad_input_file, parse_tb05ad_result_file, Tb05AdCase,
    Tb05AdExampleError, Tb05AdInput, Tb05AdOutput,
};
