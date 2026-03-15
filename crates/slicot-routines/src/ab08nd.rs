//! Pure-Rust implementation of `AB08ND` (invariant zeros via regular pencil).
//!
//! Builds the regular pencil whose generalized eigenvalues are the invariant zeros
//! of (A,B,C,D). Full implementation requires QZ/generalized eigenvalue decomposition;
//! this module is a documented stub until that dependency is available.

use thiserror::Error;

/// Errors returned by the pure-Rust `AB08ND` implementation.
#[derive(Debug, Error)]
pub enum Ab08NdError {
    #[error("AB08ND is not yet implemented; requires QZ/generalized eigenvalue decomposition")]
    NotImplemented,
}

/// Result: invariant zeros, orders of infinite zeros, Kronecker indices (not yet produced).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ab08NdResult {
    pub invariant_zeros: Vec<f64>,
}

/// Computes invariant zeros of (A,B,C,D) via the regular pencil (generalized eigenvalues).
///
/// # Errors
///
/// Currently always returns [`Ab08NdError::NotImplemented`].
pub fn ab08nd_zeros(
    _a: &[Vec<f64>],
    _b: &[Vec<f64>],
    _c: &[Vec<f64>],
    _d: &[Vec<f64>],
) -> Result<Ab08NdResult, Ab08NdError> {
    Err(Ab08NdError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::{ab08nd_zeros, Ab08NdError};

    #[test]
    fn ab08nd_returns_not_implemented() {
        let a = vec![vec![0.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let err = ab08nd_zeros(&a, &b, &c, &d).unwrap_err();
        assert!(matches!(err, Ab08NdError::NotImplemented));
    }
}
