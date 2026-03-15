//! Pure-Rust implementation of `AB09AD` (balanced truncation for stable systems).
//!
//! Model reduction by balanced truncation: compute Gramians, balance, truncate.
//! Full implementation depends on controllability Lyapunov (A W + W A' = -B B');
//! this module is a documented stub until that path is available in SB03MD or equivalent.

use thiserror::Error;

/// Errors returned by the pure-Rust `AB09AD` implementation.
#[derive(Debug, Error)]
pub enum Ab09AdError {
    #[error("AB09AD is not yet implemented; requires controllability Gramian solver")]
    NotImplemented,
}

/// Result: reduced (Ar, Br, Cr, Dr) and Hankel singular values (not yet produced).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ab09AdResult {
    pub order: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
}

/// Balanced truncation model reduction for stable systems.
///
/// # Errors
///
/// Currently always returns [`Ab09AdError::NotImplemented`].
pub fn ab09ad_balance_truncate(
    _a: &[Vec<f64>],
    _b: &[Vec<f64>],
    _c: &[Vec<f64>],
    _d: &[Vec<f64>],
    _order: usize,
) -> Result<Ab09AdResult, Ab09AdError> {
    Err(Ab09AdError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::{ab09ad_balance_truncate, Ab09AdError};

    #[test]
    fn ab09ad_returns_not_implemented() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let err = ab09ad_balance_truncate(&a, &b, &c, &d, 1).unwrap_err();
        assert!(matches!(err, Ab09AdError::NotImplemented));
    }
}
