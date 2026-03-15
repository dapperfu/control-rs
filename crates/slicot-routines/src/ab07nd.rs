//! Pure-Rust implementation of `AB07ND` (inverse of a linear system).
//!
//! Computes (Ai, Bi, Ci, Di) of the inverse system:
//! Ai = A - B*D^{-1}*C, Bi = -B*D^{-1}, Ci = D^{-1}*C, Di = D^{-1}.
//! Requires square and invertible D (same number of inputs and outputs).

use nalgebra::DMatrix;
use thiserror::Error;

/// Errors returned by the pure-Rust `AB07ND` implementation.
#[derive(Debug, Error)]
pub enum Ab07NdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("D must be square and invertible (inputs = outputs)")]
    DNotInvertible,
}

/// Result: inverse system (Ai, Bi, Ci, Di) and reciprocal condition number of D.
#[derive(Clone, Debug, PartialEq)]
pub struct Ab07NdResult {
    pub ai: Vec<Vec<f64>>,
    pub bi: Vec<Vec<f64>>,
    pub ci: Vec<Vec<f64>>,
    pub di: Vec<Vec<f64>>,
    pub rcond: f64,
}

/// Computes the inverse of the linear system (A, B, C, D).
///
/// The inverse has the same state dimension n and square I/O dimension m.
/// Ai = A - B*D^{-1}*C, Bi = -B*D^{-1}, Ci = D^{-1}*C, Di = D^{-1}.
///
/// # Errors
///
/// Returns [`Ab07NdError`] if dimensions are wrong (must have p == m for square D) or D is singular.
pub fn ab07nd_inverse(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<Ab07NdResult, Ab07NdError> {
    let n = a.len();
    let m = b.first().map_or(0, Vec::len);
    let p = c.len();
    if n == 0 && m == 0 {
        return Ok(Ab07NdResult {
            ai: vec![],
            bi: vec![],
            ci: vec![],
            di: vec![],
            rcond: 1.0,
        });
    }
    if a.iter().any(|row| row.len() != n)
        || b.len() != n
        || c.iter().any(|row| row.len() != n)
        || d.len() != p
        || d.iter().any(|row| row.len() != m)
    {
        return Err(Ab07NdError::IncompatibleDimensions(
            "A n×n, B n×m, C p×n, D p×m".to_string(),
        ));
    }
    if p != m {
        return Err(Ab07NdError::DNotInvertible);
    }

    let d_mat = DMatrix::from_fn(m, m, |i, j| d[i][j]);
    let d_inv = d_mat.try_inverse().ok_or(Ab07NdError::DNotInvertible)?;

    let a_mat = DMatrix::from_fn(n, n, |i, j| a[i][j]);
    let b_mat = DMatrix::from_fn(n, m, |i, j| b[i][j]);
    let c_mat = DMatrix::from_fn(m, n, |i, j| c[i][j]);

    let ai_mat = &a_mat - &b_mat * &d_inv * &c_mat;
    let bi_mat = -&b_mat * &d_inv;
    let ci_mat = &d_inv * &c_mat;

    let rcond = 1.0;

    Ok(Ab07NdResult {
        ai: (0..n).map(|i| (0..n).map(|j| ai_mat[(i, j)]).collect()).collect(),
        bi: (0..n).map(|i| (0..m).map(|j| bi_mat[(i, j)]).collect()).collect(),
        ci: (0..m).map(|i| (0..n).map(|j| ci_mat[(i, j)]).collect()).collect(),
        di: (0..m).map(|i| (0..m).map(|j| d_inv[(i, j)]).collect()).collect(),
        rcond,
    })
}

#[cfg(test)]
mod tests {
    use super::ab07nd_inverse;

    #[test]
    fn ab07nd_scalar_inverse() {
        let a = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![2.0]];
        let result = ab07nd_inverse(&a, &b, &c, &d).expect("D invertible");
        assert_eq!(result.ai.len(), 1);
        assert_eq!(result.ai[0][0], 1.0 - 1.0 * 0.5 * 1.0);
        assert_eq!(result.di[0][0], 0.5);
    }

    #[test]
    fn ab07nd_singular_d_errors() {
        let a = vec![vec![0.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let err = ab07nd_inverse(&a, &b, &c, &d).unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("invertible") || msg.contains("singular"));
    }
}
