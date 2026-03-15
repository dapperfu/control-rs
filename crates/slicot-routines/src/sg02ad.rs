//! Pure-Rust implementation of an `SG02AD` subset: continuous CARE with E = I, L = 0.
//!
//! Solves Q + A'X + XA - X B R^{-1} B' X = 0 by forming G = B R^{-1} B' and
//! calling SB02MD. Only the case E = I (identity) and L = 0 is supported.

use crate::sb02md_solve;
use nalgebra::DMatrix;
use thiserror::Error;

/// Errors returned by the pure-Rust `SG02AD` implementation.
#[derive(Debug, Error)]
pub enum Sg02AdError {
    /// Only continuous-time and the generalized form E = I, L = 0 are supported.
    #[error("unsupported: {0}")]
    Unsupported(String),
    /// Dimension mismatch in inputs.
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    /// R is singular or CARE solve failed.
    #[error(transparent)]
    Care(#[from] crate::Sb02MdError),
}

/// Result of solving the generalized Riccati equation (solution X only).
#[derive(Clone, Debug, PartialEq)]
pub struct Sg02AdResult {
    /// Stabilizing solution X (n×n symmetric).
    pub x: Vec<Vec<f64>>,
}

/// Solves the continuous-time generalized Riccati equation with E = I and L = 0:
/// Q + A'X + XA - (E'XB) R^{-1} (E'XB)' = 0  =>  Q + A'X + XA - X B R^{-1} B' X = 0.
///
/// This routine supports only the subset where E is the identity matrix and L is zero.
/// It forms G = B R^{-1} B' and calls SB02MD.
///
/// # Errors
///
/// Returns [`Sg02AdError`] if dico is not 'C', E is not I, L is not zero, or CARE fails.
pub fn sg02ad_solve(
    dico: char,
    a: &[Vec<f64>],
    e: &[Vec<f64>],
    b: &[Vec<f64>],
    q: &[Vec<f64>],
    r: &[Vec<f64>],
    l: &[Vec<f64>],
) -> Result<Sg02AdResult, Sg02AdError> {
    if dico != 'C' {
        return Err(Sg02AdError::Unsupported(
            "only continuous-time (dico='C') is implemented".to_string(),
        ));
    }
    let n = a.len();
    if n == 0 {
        return Ok(Sg02AdResult { x: vec![] });
    }
    let m = b.first().map_or(0, Vec::len);
    if a.iter().any(|row| row.len() != n)
        || e.len() != n
        || e.iter().any(|row| row.len() != n)
        || b.len() != n
        || q.len() != n
        || q.iter().any(|row| row.len() != n)
        || r.len() != m
        || r.iter().any(|row| row.len() != m)
        || l.len() != n
        || l.iter().any(|row| row.len() != m)
    {
        return Err(Sg02AdError::IncompatibleDimensions(
            "A n×n, E n×n, B n×m, Q n×n, R m×m, L n×m".to_string(),
        ));
    }

    const TOL: f64 = 1.0e-10;
    for i in 0..n {
        for j in 0..n {
            let expected = if i == j { 1.0 } else { 0.0 };
            if (e[i][j] - expected).abs() > TOL {
                return Err(Sg02AdError::Unsupported(
                    "only E = I (identity) is implemented".to_string(),
                ));
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            if l[i][j].abs() > TOL {
                return Err(Sg02AdError::Unsupported(
                    "only L = 0 is implemented".to_string(),
                ));
            }
        }
    }

    let r_mat = DMatrix::from_fn(m, m, |i, j| r[i][j]);
    let r_inv = r_mat
        .try_inverse()
        .ok_or_else(|| Sg02AdError::Unsupported("R is singular".to_string()))?;
    let b_mat = DMatrix::from_fn(n, m, |i, j| b[i][j]);
    let g_mat = &b_mat * &r_inv * b_mat.transpose();
    let g: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| g_mat[(i, j)]).collect())
        .collect();

    let care_result = sb02md_solve('C', a, q, &g)?;
    Ok(Sg02AdResult { x: care_result.x })
}

#[cfg(test)]
mod tests {
    use super::sg02ad_solve;

    #[test]
    fn sg02ad_scalar_continuous_e_identity_l_zero() {
        // CARE: A=-1, Q=1, G=1 => 1 + (-1)*x + x*(-1) - x*1*x = 0 => 1 - 2x - x^2 = 0 => x^2 + 2x - 1 = 0 => x = -1 + sqrt(2)
        let a = vec![vec![-1.0]];
        let e = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let q = vec![vec![1.0]];
        let r = vec![vec![1.0]];
        let l = vec![vec![0.0]];
        let result = sg02ad_solve('C', &a, &e, &b, &q, &r, &l).expect("E=I, L=0");
        let x = result.x[0][0];
        let expected = -1.0 + 2.0_f64.sqrt();
        assert!((x - expected).abs() < 1.0e-10, "x={x} expected {expected}");
    }
}
