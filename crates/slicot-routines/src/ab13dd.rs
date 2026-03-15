//! Pure-Rust implementation of `AB13DD` L-infinity (H-infinity) norm subset.
//!
//! Computes the L-infinity norm of G(s) = C(sI - A)^{-1}B + D (continuous) or
//! G(z) = C(zI - A)^{-1}B + D (discrete) by frequency sweep and maximum singular value.

use crate::tb05ad_frequency_response;
use nalgebra::{linalg::SVD, DMatrix};
use num_complex::Complex64;
use thiserror::Error;

/// Errors returned by the pure-Rust `AB13DD` implementation.
#[derive(Debug, Error)]
pub enum Ab13DdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("only continuous (dico='C') or discrete (dico='D') supported")]
    UnsupportedDico,
    #[error("frequency response failed: {0}")]
    FrequencyResponse(String),
}

/// Result of L-infinity norm computation.
#[derive(Clone, Debug, PartialEq)]
pub struct Ab13DdResult {
    /// L-infinity (H-infinity) norm.
    pub norm: f64,
    /// Frequency (rad/s or rad/sample) at which peak gain occurs, if found.
    pub peak_frequency: Option<f64>,
}

/// Number of frequency points for sweep (log-spaced).
const N_FREQ: usize = 256;

/// Computes the L-infinity norm of the transfer function by sweeping frequency
/// and taking the maximum singular value of G(j*omega) (continuous) or G(exp(j*omega)) (discrete).
///
/// Supports only standard state-space (no descriptor E). System must have no poles on the
/// stability boundary (imaginary axis for continuous, unit circle for discrete) for a finite norm.
///
/// # Errors
///
/// Returns [`Ab13DdError`] if dimensions are wrong or frequency evaluation fails.
pub fn ab13dd_norm(
    dico: char,
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<Ab13DdResult, Ab13DdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Ab13DdResult {
            norm: 0.0,
            peak_frequency: None,
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
        return Err(Ab13DdError::IncompatibleDimensions(
            "A n×n, B n×m, C p×n, D p×m".to_string(),
        ));
    }
    if dico != 'C' && dico != 'D' {
        return Err(Ab13DdError::UnsupportedDico);
    }

    // Log-spaced frequencies: low ~1e-4 to high ~1e4 (continuous) or 0.001 to pi (discrete)
    let (omega_min, omega_max) = if dico == 'C' {
        (1.0e-4, 1.0e4)
    } else {
        (0.001, std::f64::consts::PI - 0.001)
    };

    let mut max_sigma = 0.0_f64;
    let mut peak_omega: Option<f64> = None;

    for i in 0..N_FREQ {
        let t = (i as f64) / ((N_FREQ - 1) as f64);
        let omega = omega_min * (omega_max / omega_min).powf(t);
        let freq = if dico == 'C' {
            Complex64::new(0.0, omega)
        } else {
            Complex64::new(omega.cos(), omega.sin())
        };

        let resp = tb05ad_frequency_response('N', 'G', a, b, c, freq)
            .map_err(|e| Ab13DdError::FrequencyResponse(e.to_string()))?;

        let mut g_mat = DMatrix::from_fn(p, m, |i, j| resp.g[i][j]);
        for i in 0..p {
            for j in 0..m {
                g_mat[(i, j)] = resp.g[i][j] + Complex64::new(d[i][j], 0.0);
            }
        }

        let svd = SVD::new(g_mat, false, false);
        let sigma_max = svd
            .singular_values
            .iter()
            .cloned()
            .fold(0.0_f64, |a, b| a.max(b));
        if sigma_max > max_sigma {
            max_sigma = sigma_max;
            peak_omega = Some(omega);
        }
    }

    Ok(Ab13DdResult {
        norm: max_sigma,
        peak_frequency: peak_omega,
    })
}

#[cfg(test)]
mod tests {
    use super::ab13dd_norm;

    #[test]
    fn linf_norm_scalar_stable_continuous() {
        // G(s) = 1/(s+1), so |G(j w)| = 1/sqrt(1+w^2), max at w=0 is 1
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let result = ab13dd_norm('C', &a, &b, &c, &d).expect("norm");
        assert!((result.norm - 1.0).abs() < 0.05, "expected ~1, got {}", result.norm);
    }
}
