//! Pure-Rust implementation of `AB09ND` (singular perturbation approximation).
//!
//! Model reduction by singular perturbation approximation (matchdc).
//! This module is a documented stub until AB09AD and SPA formulas are available.

use thiserror::Error;

/// Errors returned by the pure-Rust `AB09ND` implementation.
#[derive(Debug, Error)]
pub enum Ab09NdError {
    #[error("AB09ND is not yet implemented")]
    NotImplemented,
}

/// Result: reduced system (not yet produced).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ab09NdResult {
    pub order: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
}

/// Singular perturbation approximation (stable part).
///
/// # Errors
///
/// Currently always returns [`Ab09NdError::NotImplemented`].
pub fn ab09nd_spa(
    _a: &[Vec<f64>],
    _b: &[Vec<f64>],
    _c: &[Vec<f64>],
    _d: &[Vec<f64>],
    _order: usize,
) -> Result<Ab09NdResult, Ab09NdError> {
    Err(Ab09NdError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::{ab09nd_spa, Ab09NdError};

    #[test]
    fn ab09nd_returns_not_implemented() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let err = ab09nd_spa(&a, &b, &c, &d, 1).unwrap_err();
        assert!(matches!(err, Ab09NdError::NotImplemented));
    }
}
