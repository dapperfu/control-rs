//! Pure-Rust implementation of `SB01BD` (pole placement, Varga) subset.
//!
//! Single-input case: state feedback F such that eig(A - B*F) = desired poles.
//! Uses Ackermann formula via controllability matrix and desired characteristic polynomial.

use nalgebra::DMatrix;
use thiserror::Error;

/// Errors returned by the pure-Rust `SB01BD` implementation.
#[derive(Debug, Error)]
pub enum Sb01BdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("system is not controllable")]
    NotControllable,
    #[error("only single-input (m=1) is implemented")]
    MultiInputNotImplemented,
}

/// Result: state feedback gain F (1×n for single-input).
#[derive(Clone, Debug, PartialEq)]
pub struct Sb01BdResult {
    pub f: Vec<Vec<f64>>,
}

/// Places closed-loop poles for single-input system: computes F such that
/// eigenvalues of A - B*F are the given real poles.
///
/// Uses Ackermann formula: F = -e_n' * inv(Ct) * phi(A), where Ct is the
/// controllability matrix and phi is the desired closed-loop characteristic polynomial.
///
/// # Errors
///
/// Returns [`Sb01BdError`] if dimensions are wrong, system is not controllable, or multi-input.
pub fn sb01bd_place(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    poles: &[f64],
) -> Result<Sb01BdResult, Sb01BdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Sb01BdResult { f: vec![] });
    }
    if a.iter().any(|row| row.len() != n) || b.len() != n {
        return Err(Sb01BdError::IncompatibleDimensions("A n×n, B n×m".to_string()));
    }
    let m = b[0].len();
    if m != 1 {
        return Err(Sb01BdError::MultiInputNotImplemented);
    }
    if poles.len() != n {
        return Err(Sb01BdError::IncompatibleDimensions(
            "poles length must equal n".to_string(),
        ));
    }

    let a_mat = DMatrix::from_fn(n, n, |i, j| a[i][j]);
    let b_mat = DMatrix::from_fn(n, 1, |i, _| b[i][0]);

    let ct = DMatrix::from_fn(n, n, |i, j| {
        let mut col = b_mat.column(0).into_owned();
        for _ in 0..j {
            col = &a_mat * &col;
        }
        col[i]
    });
    let ct_inv = ct
        .try_inverse()
        .ok_or(Sb01BdError::NotControllable)?;

    let id = DMatrix::identity(n, n);
    let mut phi_a = id.clone();
    for &p in poles {
        let term = &a_mat - &id * p;
        phi_a = &term * phi_a;
    }

    let e_n = ct_inv.row(n - 1);
    let f_row = e_n * &phi_a;
    let f = vec![(0..n).map(|j| f_row[j]).collect()];

    Ok(Sb01BdResult { f })
}

#[cfg(test)]
mod tests {
    use super::sb01bd_place;

    #[test]
    fn sb01bd_place_scalar() {
        let a = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let poles = vec![-2.0];
        let result = sb01bd_place(&a, &b, &poles).expect("place");
        assert_eq!(result.f.len(), 1);
        assert_eq!(result.f[0].len(), 1);
        let f = result.f[0][0];
        assert!((1.0 - f + 2.0).abs() < 1e-10, "eig(A-BF) = 1-f = -2 => f=3");
    }
}
