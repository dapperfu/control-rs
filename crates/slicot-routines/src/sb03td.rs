//! Pure-Rust implementation of the `SB03TD` continuous Lyapunov solver.
//!
//! Solves op(A)' X + X op(A) = scale*C (continuous-time). Implemented by
//! delegating to the SB03MD continuous-time path.

use thiserror::Error;

use crate::sb03md::{sb03md_solve, Sb03MdError, Sb03MdResult};

/// Errors returned by the pure-Rust `SB03TD` implementation.
#[derive(Debug, Error)]
pub enum Sb03TdError {
    /// Delegation to SB03MD failed.
    #[error(transparent)]
    Lyapunov(#[from] Sb03MdError),
}

/// Output bundle for the pure-Rust `SB03TD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03TdResult {
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Solves the continuous-time Lyapunov equation op(A)' X + X op(A) = scale*C
/// with op(A) = A (transa = 'N').
///
/// # Errors
///
/// Returns [`Sb03TdError`] if dimensions are incompatible or the Kronecker
/// system is singular.
pub fn sb03td_solve(a: &[Vec<f64>], c: &[Vec<f64>]) -> Result<Sb03TdResult, Sb03TdError> {
    let Sb03MdResult { x, scale } = sb03md_solve('C', 'X', 'N', 'N', a, c)?;
    Ok(Sb03TdResult { x, scale })
}
