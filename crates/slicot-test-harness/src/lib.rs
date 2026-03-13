#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Test harness utilities for mirroring the upstream SLICOT example corpus.

mod catalog;
mod error;
mod tb04ad;

pub use catalog::{discover_example_cases, ExampleCase};
pub use error::CatalogError;
pub use tb04ad::{
    load_tb04ad_case, parse_tb04ad_input_file, parse_tb04ad_result_file, Tb04AdCase,
    Tb04AdExampleError, Tb04AdInput, Tb04AdOutput, TransferPolynomial,
};
