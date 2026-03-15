//! Pure-Rust implementation of the `SB03QD` Lyapunov solver subset.
//!
//! Solves the continuous-time Lyapunov equation. Implemented by
//! delegating to SB03MD with DICO='C'.

use thiserror::Error;

use crate::sb03md::{sb03md_solve, Sb03MdError, Sb03MdResult};

/// Errors returned by the pure-Rust `SB03QD` implementation.
#[derive(Debug, Error)]
pub enum Sb03QdError {
    /// Delegation to SB03MD failed.
    #[error(transparent)]
    Lyapunov(#[from] Sb03MdError),
}

/// Output bundle for the pure-Rust `SB03QD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03QdResult {
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Solves the continuous-time Lyapunov equation op(A)' X + X op(A) = scale*C.
///
/// # Errors
///
/// Returns [`Sb03QdError`] if dimensions are incompatible or the Kronecker
/// system is singular.
pub fn sb03qd_solve(a: &[Vec<f64>], c: &[Vec<f64>]) -> Result<Sb03QdResult, Sb03QdError> {
    let Sb03MdResult { x, scale } = sb03md_solve('C', 'X', 'N', 'N', a, c)?;
    Ok(Sb03QdResult { x, scale })
}
