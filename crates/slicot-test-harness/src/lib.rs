#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Test harness utilities for mirroring the upstream SLICOT example corpus.

mod catalog;
mod error;
mod inventory;
mod python_control;
mod tb04ad;
mod tb05ad;

pub use catalog::{discover_example_cases, ExampleCase};
pub use error::CatalogError;
pub use inventory::{discover_routine_inventory, RoutineInventoryEntry};
pub use python_control::{
    phase_one_python_control_targets, resolve_phase_one_python_control_targets,
    PythonControlTarget, ResolvedPythonControlTarget,
};
pub use tb04ad::{
    load_tb04ad_case, parse_tb04ad_input_file, parse_tb04ad_result_file, Tb04AdCase,
    Tb04AdExampleError, Tb04AdInput, Tb04AdOutput, TransferPolynomial,
};
pub use tb05ad::{
    load_tb05ad_case, parse_tb05ad_input_file, parse_tb05ad_result_file, Tb05AdCase,
    Tb05AdExampleError, Tb05AdInput, Tb05AdOutput,
};
