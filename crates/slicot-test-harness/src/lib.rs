#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Test harness utilities for mirroring the upstream SLICOT example corpus.

mod catalog;
mod error;

pub use catalog::{discover_example_cases, ExampleCase};
pub use error::CatalogError;
