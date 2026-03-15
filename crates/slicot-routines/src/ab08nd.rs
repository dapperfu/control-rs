//! Pure-Rust implementation of `AB08ND` (invariant zeros via regular pencil).
//!
//! When D is square and invertible, invariant zeros are the eigenvalues of
//! A - B D^{-1} C. When D is singular or non-square, the full pencil (QZ) is required;
//! that case returns [`Ab08NdError::D singular or non-square`].

use nalgebra::DMatrix;
use num_complex::Complex64;
use thiserror::Error;

/// Errors returned by the pure-Rust `AB08ND` implementation.
#[derive(Debug, Error)]
pub enum Ab08NdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("D must be square and invertible for this implementation; D singular or non-square")]
    DNotInvertible,
}

/// Result: invariant zeros (real part for real zeros, or finite generalized eigenvalues).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ab08NdResult {
    pub invariant_zeros: Vec<f64>,
    /// Complex zeros (for non-real zeros, both real and imaginary parts).
    pub invariant_zeros_complex: Vec<Complex64>,
}

/// Computes invariant zeros of (A,B,C,D).
///
/// When D is square and invertible: zeros = eigenvalues of A - B D^{-1} C.
/// When D is singular or non-square: returns [`Ab08NdError::DNotInvertible`].
///
/// # Errors
///
/// Returns [`Ab08NdError`] if dimensions are wrong or D is not invertible.
pub fn ab08nd_zeros(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<Ab08NdResult, Ab08NdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Ab08NdResult {
            invariant_zeros: vec![],
            invariant_zeros_complex: vec![],
        });
    }
    let m = b.first().map_or(0, Vec::len);
    let p = c.len();
    if a.iter().any(|row| row.len() != n)
        || b.len() != n
        || c.iter().any(|row| row.len() != n)
        || d.len() != p
        || d.iter().any(|row| row.len() != m)
    {
        return Err(Ab08NdError::IncompatibleDimensions(
            "A n×n, B n×m, C p×n, D p×m".to_string(),
        ));
    }
    if p != m {
        return Err(Ab08NdError::DNotInvertible);
    }

    let d_mat = DMatrix::from_fn(p, m, |i, j| d[i][j]);
    let d_inv = d_mat.try_inverse().ok_or(Ab08NdError::DNotInvertible)?;

    let a_mat = DMatrix::from_fn(n, n, |i, j| a[i][j]);
    let b_mat = DMatrix::from_fn(n, m, |i, j| b[i][j]);
    let c_mat = DMatrix::from_fn(p, n, |i, j| c[i][j]);

    let b_dinv_c = &b_mat * &d_inv * &c_mat;
    let a_z = &a_mat - &b_dinv_c;

    let evals = a_z.complex_eigenvalues();
    let invariant_zeros: Vec<f64> = evals.iter().map(|c| c.re).collect();
    let invariant_zeros_complex: Vec<Complex64> = evals.iter().copied().collect();

    Ok(Ab08NdResult {
        invariant_zeros,
        invariant_zeros_complex,
    })
}

#[cfg(test)]
mod tests {
    use super::{ab08nd_zeros, Ab08NdError};

    #[test]
    fn ab08nd_siso_d_zero_returns_error() {
        let a = vec![vec![0.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let err = ab08nd_zeros(&a, &b, &c, &d).unwrap_err();
        assert!(matches!(err, Ab08NdError::DNotInvertible));
    }

    #[test]
    fn ab08nd_d_invertible_returns_zeros() {
        let a = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![1.0]];
        let result = ab08nd_zeros(&a, &b, &c, &d).expect("D=1 invertible");
        assert_eq!(result.invariant_zeros.len(), 1);
        assert!((result.invariant_zeros[0] - 0.0).abs() < 1e-10);
    }
}
