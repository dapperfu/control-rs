//! Pure-Rust implementation of `AB09MD` (balanced truncation, unstable case).
//!
//! Reduces the stable part when the system has unstable poles.
//! This module is a documented stub until AB09AD and stable/unstable split are available.

use thiserror::Error;

/// Errors returned by the pure-Rust `AB09MD` implementation.
#[derive(Debug, Error)]
pub enum Ab09MdError {
    #[error("AB09MD is not yet implemented")]
    NotImplemented,
}

/// Result: reduced stable part (not yet produced).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ab09MdResult {
    pub order: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
}

/// Balanced truncation for systems with unstable part (reduce stable part).
///
/// # Errors
///
/// Currently always returns [`Ab09MdError::NotImplemented`].
pub fn ab09md_balance_truncate(
    _a: &[Vec<f64>],
    _b: &[Vec<f64>],
    _c: &[Vec<f64>],
    _d: &[Vec<f64>],
    _order: usize,
) -> Result<Ab09MdResult, Ab09MdError> {
    Err(Ab09MdError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::{ab09md_balance_truncate, Ab09MdError};

    #[test]
    fn ab09md_returns_not_implemented() {
        let a = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let err = ab09md_balance_truncate(&a, &b, &c, &d, 1).unwrap_err();
        assert!(matches!(err, Ab09MdError::NotImplemented));
    }
}
