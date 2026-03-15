#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Test harness utilities for mirroring the upstream SLICOT example corpus.

mod catalog;
mod error;
mod inventory;
mod python_control;
mod sb02md;
mod sb03md;
mod sb03od;
mod sb04md;
mod sb04qd;
mod sg03ad;
mod tb04ad;
mod tb05ad;

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
pub use sb04md::{
    load_sb04md_case, parse_sb04md_input_file, parse_sb04md_result_file, Sb04MdCase,
    Sb04MdExampleError, Sb04MdInput, Sb04MdOutput,
};
pub use sb04qd::{
    load_sb04qd_case, parse_sb04qd_input_file, parse_sb04qd_result_file, Sb04QdCase,
    Sb04QdExampleError, Sb04QdInput, Sb04QdOutput,
};
pub use sg03ad::{
    load_sg03ad_case, parse_sg03ad_input_file, parse_sg03ad_result_file, Sg03AdCase,
    Sg03AdExampleError, Sg03AdInput, Sg03AdOutput,
};
pub use tb04ad::{
    load_tb04ad_case, parse_tb04ad_input_file, parse_tb04ad_result_file, Tb04AdCase,
    Tb04AdExampleError, Tb04AdInput, Tb04AdOutput, TransferPolynomial,
};
pub use tb05ad::{
    load_tb05ad_case, parse_tb05ad_input_file, parse_tb05ad_result_file, Tb05AdCase,
    Tb05AdExampleError, Tb05AdInput, Tb05AdOutput,
};
