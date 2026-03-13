//! Error types for the upstream SLICOT example catalog.

use std::path::PathBuf;

use thiserror::Error;

/// Errors produced while discovering upstream SLICOT examples.
#[derive(Debug, Error)]
pub enum CatalogError {
    /// The examples root could not be enumerated.
    #[error("failed to read SLICOT examples directory {path}: {source}")]
    ReadDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    /// A directory entry inside the examples root could not be read.
    #[error("failed to read an entry from SLICOT examples directory {path}: {source}")]
    ReadEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    /// An example file name did not match the upstream `T*.f` naming convention.
    #[error("invalid SLICOT example filename: {path}")]
    InvalidExampleName { path: PathBuf },
}
