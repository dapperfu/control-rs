//! Pure-Rust implementation of the `SB03SD` discrete Lyapunov solver.
//!
//! Solves A X A' - X = scale*C (discrete-time Lyapunov). Implemented by
//! delegating to the SB03MD discrete-time path.

use thiserror::Error;

use crate::sb03md::{sb03md_solve, Sb03MdError, Sb03MdResult};

/// Errors returned by the pure-Rust `SB03SD` implementation.
#[derive(Debug, Error)]
pub enum Sb03SdError {
    /// Delegation to SB03MD failed.
    #[error(transparent)]
    Lyapunov(#[from] Sb03MdError),
}

/// Output bundle for the pure-Rust `SB03SD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03SdResult {
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Solves the discrete-time Lyapunov equation A X A' - X = scale*C.
///
/// # Errors
///
/// Returns [`Sb03SdError`] if dimensions are incompatible or the Kronecker
/// system is singular.
pub fn sb03sd_solve(a: &[Vec<f64>], c: &[Vec<f64>]) -> Result<Sb03SdResult, Sb03SdError> {
    let Sb03MdResult { x, scale } = sb03md_solve('D', 'X', 'N', 'N', a, c)?;
    Ok(Sb03SdResult { x, scale })
}
