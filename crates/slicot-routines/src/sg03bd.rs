//! Pure-Rust implementation of the `SG03BD` generalized Lyapunov Cholesky factor.
//!
//! Solves A' X E + E' X A = -scale² B' B and returns the upper Cholesky factor U
//! such that X = U' U. Implemented via [`sg03ad_solve`](crate::sg03ad_solve) and
//! Cholesky factorization.

use thiserror::Error;

use crate::sg03ad::{sg03ad_solve, Sg03AdError, Sg03AdResult};

/// Errors returned by the pure-Rust `SG03BD` implementation.
#[derive(Debug, Error)]
pub enum Sg03BdError {
    /// Forwarded from the underlying SG03AD solve.
    #[error(transparent)]
    Lyapunov(#[from] Sg03AdError),
    /// The solution matrix X is not positive definite; Cholesky factorization failed.
    #[error("solution matrix is not positive definite; Cholesky factorization failed")]
    NotPositiveDefinite,
}

/// Output bundle for the pure-Rust `SG03BD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03BdResult {
    /// Upper Cholesky factor U such that X = U' U.
    pub u: Vec<Vec<f64>>,
    /// Scale factor.
    pub scale: f64,
}

/// Forms Y = -B' B. B is stored as rows (m×n), so B'B is n×n.
fn neg_btb(b: &[Vec<f64>], n: usize) -> Vec<Vec<f64>> {
    let mut y = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for row in b {
                sum += row[i] * row[j];
            }
            y[i][j] = -sum;
        }
    }
    y
}

/// Lower Cholesky L such that X = L L'. Returns L transposed so that U' U = X (U = L').
fn cholesky_upper(x: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, Sg03BdError> {
    let n = x.len();
    let mut l = vec![vec![0.0; n]; n];
    for row in 0..n {
        for col in 0..=row {
            let mut sum = x[row][col];
            for k in 0..col {
                sum -= l[row][k] * l[col][k];
            }
            if row == col {
                if sum <= 0.0 {
                    return Err(Sg03BdError::NotPositiveDefinite);
                }
                l[row][col] = sum.sqrt();
            } else {
                if l[col][col].abs() < f64::EPSILON {
                    return Err(Sg03BdError::NotPositiveDefinite);
                }
                l[row][col] = sum / l[col][col];
            }
        }
    }
    Ok(transpose(&l))
}

fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = m.len();
    let cols = m.first().map_or(0, Vec::len);
    let mut t = vec![vec![0.0; rows]; cols];
    for (i, row) in m.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            t[j][i] = v;
        }
    }
    t
}

/// Solves the generalized Lyapunov equation A' X E + E' X A = -scale² B' B and
/// returns the upper Cholesky factor U such that X = U' U (SG03BD API).
///
/// Supported: `dico = 'C'` or `'D'`, `fact = 'N'`, `trans = 'N'` or `'T'`.
/// B is stored as rows (m×n).
///
/// # Errors
///
/// Returns [`Sg03BdError`] if the Lyapunov solve fails or X is not positive definite.
pub fn sg03bd_solve(
    dico: char,
    fact: char,
    trans: char,
    a: &[Vec<f64>],
    e: &[Vec<f64>],
    b: &[Vec<f64>],
) -> Result<Sg03BdResult, Sg03BdError> {
    let n = a.len();
    let y = neg_btb(b, n);
    let Sg03AdResult { x, scale, .. } =
        sg03ad_solve(dico, 'X', fact, trans, a, e, &y)?;
    let u = cholesky_upper(&x)?;
    Ok(Sg03BdResult { u, scale })
}
