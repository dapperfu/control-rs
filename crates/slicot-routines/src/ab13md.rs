//! Stub for `AB13MD` (H-infinity norm variant).
//!
//! Not directly used by python-control. Exposed so phase-one API is complete;
//! returns [`Ab13MdError::NotImplemented`] until the routine is ported.

use thiserror::Error;

/// Errors returned by the pure-Rust `AB13MD` implementation.
#[derive(Debug, Error)]
pub enum Ab13MdError {
    /// This routine is not yet implemented.
    #[error("AB13MD is not yet implemented")]
    NotImplemented,
}

/// Stub: AB13MD is not yet implemented.
///
/// # Errors
///
/// Always returns [`Ab13MdError::NotImplemented`].
#[allow(clippy::module_name_repetitions)]
pub fn ab13md_norm(
    _dico: char,
    _a: &[Vec<f64>],
    _b: &[Vec<f64>],
    _c: &[Vec<f64>],
    _d: &[Vec<f64>],
) -> Result<f64, Ab13MdError> {
    Err(Ab13MdError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::{ab13md_norm, Ab13MdError};

    #[test]
    fn ab13md_returns_not_implemented() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let err = ab13md_norm('C', &a, &b, &c, &d).unwrap_err();
        assert!(matches!(err, Ab13MdError::NotImplemented));
    }
}
