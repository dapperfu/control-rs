//! Pure-Rust implementation of `AB13MD` (H-infinity norm variant).
//!
//! Not directly used by python-control. Implemented by delegating to AB13DD,
//! which computes the L-infinity (H-infinity) norm via frequency sweep.

use crate::ab13dd_norm;
use thiserror::Error;

/// Errors returned by the pure-Rust `AB13MD` implementation.
#[derive(Debug, Error)]
pub enum Ab13MdError {
    /// This routine delegates to AB13DD; the norm computation failed.
    #[error("AB13MD (via AB13DD): {0}")]
    NormFailed(String),
}

/// Computes the H-infinity (L-infinity) norm of the transfer function
/// G(λ) = C(λI - A)^{-1}B + D.
///
/// Implemented by calling [`crate::ab13dd_norm`]; returns the norm value.
///
/// # Errors
///
/// Returns [`Ab13MdError`] if dimensions are wrong or the system has poles on the
/// stability boundary.
#[allow(clippy::module_name_repetitions)]
pub fn ab13md_norm(
    dico: char,
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<f64, Ab13MdError> {
    let result = ab13dd_norm(dico, a, b, c, d).map_err(|e| Ab13MdError::NormFailed(e.to_string()))?;
    Ok(result.norm)
}

#[cfg(test)]
mod tests {
    use super::ab13md_norm;

    #[test]
    fn ab13md_returns_norm_for_stable_system() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let norm = ab13md_norm('C', &a, &b, &c, &d).expect("stable");
        assert!(norm > 0.0 && norm <= 1.0 + 1e-6);
    }

    #[test]
    fn ab13md_discrete() {
        let a = vec![vec![0.5]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let norm = ab13md_norm('D', &a, &b, &c, &d).expect("stable discrete");
        assert!(norm > 0.0);
    }
}
