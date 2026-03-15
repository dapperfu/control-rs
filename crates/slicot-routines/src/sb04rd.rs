//! Pure-Rust implementation of the `SB04RD` discrete Sylvester solver.
//!
//! Solves X + A X B = C. Implemented by delegating to SB04QD.

use thiserror::Error;

use crate::sb04qd::{sb04qd_solve, Sb04QdError, Sb04QdResult};

/// Errors returned by the pure-Rust `SB04RD` implementation.
#[derive(Debug, Error)]
pub enum Sb04RdError {
    /// Delegation to SB04QD failed.
    #[error(transparent)]
    Sylvester(#[from] Sb04QdError),
}

/// Output bundle for the pure-Rust `SB04RD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04RdResult {
    pub x: Vec<Vec<f64>>,
}

/// Solves the discrete-time Sylvester equation X + A X B = C.
///
/// # Errors
///
/// Returns [`Sb04RdError`] if dimensions are incompatible or the Kronecker
/// system is singular.
pub fn sb04rd_solve(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
) -> Result<Sb04RdResult, Sb04RdError> {
    let Sb04QdResult { x, .. } = sb04qd_solve(a, b, c)?;
    Ok(Sb04RdResult { x })
}
