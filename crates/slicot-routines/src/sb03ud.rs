//! Pure-Rust implementation of the `SB03UD` discrete Lyapunov solver.
//!
//! Solves the discrete-time Lyapunov equation (same form as SB03SD).
//! Implemented by delegating to SB03MD with DICO='D'.

use thiserror::Error;

use crate::sb03md::{sb03md_solve, Sb03MdError, Sb03MdResult};

/// Errors returned by the pure-Rust `SB03UD` implementation.
#[derive(Debug, Error)]
pub enum Sb03UdError {
    /// Delegation to SB03MD failed.
    #[error(transparent)]
    Lyapunov(#[from] Sb03MdError),
}

/// Output bundle for the pure-Rust `SB03UD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03UdResult {
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Solves the discrete-time Lyapunov equation A X A' - X = scale*C.
///
/// # Errors
///
/// Returns [`Sb03UdError`] if dimensions are incompatible or the Kronecker
/// system is singular.
pub fn sb03ud_solve(a: &[Vec<f64>], c: &[Vec<f64>]) -> Result<Sb03UdResult, Sb03UdError> {
    let Sb03MdResult { x, scale } = sb03md_solve('D', 'X', 'N', 'N', a, c)?;
    Ok(Sb03UdResult { x, scale })
}
