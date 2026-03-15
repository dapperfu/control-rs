//! Pure-Rust implementation of `SB02MT`: CARE preprocessing transformation.
//!
//! Converts optimal control (A, B, Q, R, L) to standard form:
//! G = B R^{-1} B', A_bar = A - B R^{-1} L', Q_bar = Q - L R^{-1} L'.

use nalgebra::DMatrix;
use thiserror::Error;

/// Errors returned by the pure-Rust `SB02MT` implementation.
#[derive(Debug, Error)]
pub enum Sb02MtError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("R is singular")]
    SingularR,
}

/// Result of SB02MT transform: matrices for standard CARE A'X + X A_bar - X G X + Q_bar = 0.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb02MtResult {
    /// Transformed state matrix A - B R^{-1} L' (n×n).
    pub a_bar: Vec<Vec<f64>>,
    /// Transformed cost matrix Q - L R^{-1} L' (n×n).
    pub q_bar: Vec<Vec<f64>>,
    /// G = B R^{-1} B' (n×n symmetric).
    pub g: Vec<Vec<f64>>,
}

/// Computes the preprocessing transformation for CARE with coupling term L:
/// G = B R^{-1} B', A_bar = A - B R^{-1} L', Q_bar = Q - L R^{-1} L'.
///
/// The resulting matrices satisfy the standard CARE: A_bar' X + X A_bar - X G X + Q_bar = 0.
///
/// # Errors
///
/// Returns [`Sb02MtError`] if dimensions are wrong or R is singular.
pub fn sb02mt_transform(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    q: &[Vec<f64>],
    r: &[Vec<f64>],
    l: &[Vec<f64>],
) -> Result<Sb02MtResult, Sb02MtError> {
    let n = a.len();
    if n == 0 {
        return Ok(Sb02MtResult {
            a_bar: vec![],
            q_bar: vec![],
            g: vec![],
        });
    }
    let m = b.first().map_or(0, Vec::len);
    if a.iter().any(|row| row.len() != n)
        || b.len() != n
        || q.len() != n
        || q.iter().any(|row| row.len() != n)
        || r.len() != m
        || r.iter().any(|row| row.len() != m)
        || l.len() != n
        || l.iter().any(|row| row.len() != m)
    {
        return Err(Sb02MtError::IncompatibleDimensions(
            "A n×n, B n×m, Q n×n, R m×m, L n×m".to_string(),
        ));
    }

    let r_mat = DMatrix::from_fn(m, m, |i, j| r[i][j]);
    let r_inv = r_mat
        .try_inverse()
        .ok_or(Sb02MtError::SingularR)?;
    let b_mat = DMatrix::from_fn(n, m, |i, j| b[i][j]);
    let l_mat = DMatrix::from_fn(n, m, |i, j| l[i][j]);
    let a_mat = DMatrix::from_fn(n, n, |i, j| a[i][j]);
    let q_mat = DMatrix::from_fn(n, n, |i, j| q[i][j]);

    let g_mat = &b_mat * &r_inv * b_mat.transpose();
    let blt = &b_mat * &r_inv * l_mat.transpose();
    let a_bar_mat = &a_mat - &blt;
    let lrt_lt = &l_mat * &r_inv * l_mat.transpose();
    let q_bar_mat = &q_mat - &lrt_lt;

    let to_vec = |m: &DMatrix<f64>| {
        (0..m.nrows())
            .map(|i| (0..m.ncols()).map(|j| m[(i, j)]).collect())
            .collect()
    };

    Ok(Sb02MtResult {
        a_bar: to_vec(&a_bar_mat),
        q_bar: to_vec(&q_bar_mat),
        g: to_vec(&g_mat),
    })
}

#[cfg(test)]
mod tests {
    use super::sb02mt_transform;

    #[test]
    fn sb02mt_scalar_l_zero() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let q = vec![vec![1.0]];
        let r = vec![vec![1.0]];
        let l = vec![vec![0.0]];
        let result = sb02mt_transform(&a, &b, &q, &r, &l).expect("transform");
        assert_eq!(result.a_bar[0][0], -1.0);
        assert_eq!(result.q_bar[0][0], 1.0);
        assert_eq!(result.g[0][0], 1.0);
    }

    #[test]
    fn sb02mt_with_coupling() {
        let a = vec![vec![0.0, 1.0], vec![0.0, 0.0]];
        let b = vec![vec![0.0], vec![1.0]];
        let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let r = vec![vec![1.0]];
        let l = vec![vec![0.5], vec![0.0]];
        let result = sb02mt_transform(&a, &b, &q, &r, &l).expect("transform");
        // A_bar = A - B R^{-1} L' = [[0,1],[0,0]] - [[0],[1]] * 0.5 = [[0,1],[0,0]] - [[0,0],[0.5,0]] = [[0,1],[-0.5,0]]
        assert!((result.a_bar[1][0] - (-0.5)).abs() < 1e-10);
        // Q_bar = Q - L R^{-1} L' = I - [[0.25,0],[0,0]] = [[0.75,0],[0,1]]
        assert!((result.q_bar[0][0] - 0.75).abs() < 1e-10);
        // G = B R^{-1} B' = [[0],[1]] [[0,1]] = [[0,0],[0,1]]
        assert!((result.g[1][1] - 1.0).abs() < 1e-10);
    }
}
