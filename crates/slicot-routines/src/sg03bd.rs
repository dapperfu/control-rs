//! Pure-Rust implementation of the `SG03BD` generalized Lyapunov Cholesky factor.
//!
//! Solves A' X E + E' X A = -scale² B' B for the solution X and returns the
//! upper Cholesky factor U with X = U' U. Implemented via SG03AD then Cholesky.

use nalgebra::DMatrix;
use thiserror::Error;

use crate::sg03ad::{sg03ad_solve, Sg03AdError};

/// Errors returned by the pure-Rust `SG03BD` implementation.
#[derive(Debug, Error)]
pub enum Sg03BdError {
    /// Delegation to SG03AD failed.
    #[error(transparent)]
    Lyapunov(#[from] Sg03AdError),
    /// The solution X is not symmetric positive definite (Cholesky failed).
    #[error("solution matrix is not positive definite")]
    NotPositiveDefinite,
}

/// Output bundle for the pure-Rust `SG03BD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03BdResult {
    pub scale: f64,
    /// Upper triangular U such that X = U' U.
    pub u: Vec<Vec<f64>>,
}

/// Computes the upper Cholesky factor U of the solution X of the generalized
/// Lyapunov equation A' X E + E' X A = -scale² B' B.
///
/// Builds Y = -B' B, solves for X via SG03AD, then factors X = U' U.
///
/// # Errors
///
/// Returns [`Sg03BdError`] if dimensions are incompatible, the Kronecker system
/// is singular, or X is not positive definite.
pub fn sg03bd_solve(
    a: &[Vec<f64>],
    e: &[Vec<f64>],
    b: &[Vec<f64>],
) -> Result<Sg03BdResult, Sg03BdError> {
    let n = a.len();
    let _m = b.len();
    let y = negate_bt_b(b);
    let result = sg03ad_solve('C', 'X', 'N', 'N', a, e, &y)?;
    let x = result.x;
    let x_mat = DMatrix::from_fn(n, n, |i, j| x[i][j]);
    let cholesky = nalgebra::linalg::Cholesky::new(x_mat)
        .ok_or(Sg03BdError::NotPositiveDefinite)?;
    let l = cholesky.l();
    let u = l.transpose();
    let u_vec = (0..n)
        .map(|i| (0..n).map(|j| u[(i, j)]).collect())
        .collect();
    Ok(Sg03BdResult {
        scale: result.scale,
        u: u_vec,
    })
}

fn negate_bt_b(b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let m = b.len();
    let n = b.first().map_or(0, Vec::len);
    let b_mat = DMatrix::from_fn(m, n, |i, j| b[i][j]);
    let bt_b = b_mat.transpose() * &b_mat;
    let neg = -bt_b;
    (0..n)
        .map(|i| (0..n).map(|j| neg[(i, j)]).collect())
        .collect()
}
