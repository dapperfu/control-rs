//! Pure-Rust implementation of the `SB04ND` continuous Sylvester solver.
//!
//! Solves A X + X B = C. Implemented by delegating to SB04MD.

use thiserror::Error;

use crate::sb04md::{sb04md_solve, Sb04MdError, Sb04MdResult};

/// Errors returned by the pure-Rust `SB04ND` implementation.
#[derive(Debug, Error)]
pub enum Sb04NdError {
    /// Delegation to SB04MD failed.
    #[error(transparent)]
    Sylvester(#[from] Sb04MdError),
}

/// Output bundle for the pure-Rust `SB04ND` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04NdResult {
    pub x: Vec<Vec<f64>>,
}

/// Solves the continuous-time Sylvester equation A X + X B = C.
///
/// # Errors
///
/// Returns [`Sb04NdError`] if dimensions are incompatible or the Kronecker
/// system is singular.
pub fn sb04nd_solve(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
) -> Result<Sb04NdResult, Sb04NdError> {
    let Sb04MdResult { x, .. } = sb04md_solve(a, b, c)?;
    Ok(Sb04NdResult { x })
}
