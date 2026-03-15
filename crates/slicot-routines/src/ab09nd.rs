//! Pure-Rust implementation of `AB09ND` (singular perturbation approximation).
//!
//! Model reduction by singular perturbation approximation (matchdc): balance the system,
//! partition state into kept (1..r) and discarded (r..n), set dx2/dt = 0 and eliminate x2
//! to get reduced (Ar, Br, Cr, Dr).

use crate::sb03md_solve;
use nalgebra::DMatrix;
use thiserror::Error;

/// Errors returned by the pure-Rust `AB09ND` implementation.
#[derive(Debug, Error)]
pub enum Ab09NdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("system is not stable (Lyapunov failed)")]
    NotStable,
    #[error("Gramian is not positive definite (Cholesky failed)")]
    GramianNotPositiveDefinite,
    #[error("reduction order {order} must be in 1..{n}")]
    InvalidOrder { order: usize, n: usize },
    #[error("A22 block is singular (cannot invert for SPA)")]
    SingularA22,
    #[error(transparent)]
    Lyapunov(#[from] crate::Sb03MdError),
}

/// Result: reduced system (Ar, Br, Cr, Dr) by singular perturbation approximation.
#[derive(Clone, Debug, PartialEq)]
pub struct Ab09NdResult {
    pub order: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
}

fn mat_mul(ar: usize, ac: usize, a: &[Vec<f64>], _br: usize, bc: usize, b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut out = vec![vec![0.0; bc]; ar];
    for i in 0..ar {
        for j in 0..bc {
            let mut s = 0.0;
            for k in 0..ac {
                s += a[i][k] * b[k][j];
            }
            out[i][j] = s;
        }
    }
    out
}

fn mat_mul_at_b(ar: usize, ac: usize, a: &[Vec<f64>], _br: usize, bc: usize, b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut out = vec![vec![0.0; bc]; ac];
    for i in 0..ac {
        for j in 0..bc {
            let mut s = 0.0;
            for k in 0..ar {
                s += a[k][i] * b[k][j];
            }
            out[i][j] = s;
        }
    }
    out
}

fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let r = m.len();
    let c = m.first().map_or(0, Vec::len);
    let mut t = vec![vec![0.0; r]; c];
    for i in 0..r {
        for j in 0..c {
            t[j][i] = m[i][j];
        }
    }
    t
}

fn negate(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    m.iter().map(|row| row.iter().map(|&x| -x).collect()).collect()
}

/// Singular perturbation approximation for stable systems (continuous-time subset).
///
/// Computes balanced realization via Gramians, then partitions at order `r` and applies
/// SPA: Ar = A11 - A12*inv(A22)*A21, Br = B1 - A12*inv(A22)*B2, Cr = C1 - C2*inv(A22)*A21,
/// Dr = D - C2*inv(A22)*B2.
///
/// # Errors
///
/// Returns [`Ab09NdError`] if dimensions are wrong, system is not stable, or A22 is singular.
pub fn ab09nd_spa(
    dico: char,
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
    order: usize,
) -> Result<Ab09NdResult, Ab09NdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Ab09NdResult {
            order: 0,
            a: vec![],
            b: vec![],
            c: vec![],
            d: d.to_vec(),
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
        return Err(Ab09NdError::IncompatibleDimensions(
            "A n×n, B n×m, C p×n, D p×m".to_string(),
        ));
    }
    if order >= n || order == 0 {
        return Err(Ab09NdError::InvalidOrder { order, n });
    }

    let bbt = mat_mul(n, m, b, m, n, &transpose(b));
    let neg_bbt = negate(&bbt);
    let wc_result = sb03md_solve(dico, 'X', 'N', 'N', a, &neg_bbt).map_err(|_| Ab09NdError::NotStable)?;
    let wc = wc_result.x;

    let ctc = mat_mul_at_b(p, n, c, p, n, c);
    let neg_ctc = negate(&ctc);
    let wo_result = sb03md_solve(dico, 'X', 'N', 'N', a, &neg_ctc).map_err(|_| Ab09NdError::NotStable)?;
    let wo = wo_result.x;

    let wc_mat = DMatrix::from_fn(n, n, |i, j| wc[i][j]);
    let wo_mat = DMatrix::from_fn(n, n, |i, j| wo[i][j]);

    let cholesky_wc = wc_mat.cholesky().ok_or(Ab09NdError::GramianNotPositiveDefinite)?;
    let lc = cholesky_wc.l();
    let cholesky_wo = wo_mat.cholesky().ok_or(Ab09NdError::GramianNotPositiveDefinite)?;
    let lo = cholesky_wo.l();

    let lct_lot = lc.transpose() * lo.transpose();
    let svd = nalgebra::linalg::SVD::new(lct_lot, true, true);
    let sigma = svd.singular_values;
    let u = svd.u.unwrap();
    let _v_t = svd.v_t.unwrap();

    let mut inv_sqrt_sigma = DMatrix::zeros(n, n);
    for i in 0..n {
        if sigma[i] > 1e-20 {
            inv_sqrt_sigma[(i, i)] = 1.0 / sigma[i].sqrt();
        }
    }

    let tr = &lc * &u * &inv_sqrt_sigma;
    let tr_inv = tr.clone().try_inverse().ok_or(Ab09NdError::SingularA22)?;

    let a_mat = DMatrix::from_fn(n, n, |i, j| a[i][j]);
    let b_mat = DMatrix::from_fn(n, m, |i, j| b[i][j]);
    let c_mat = DMatrix::from_fn(p, n, |i, j| c[i][j]);
    let d_mat = DMatrix::from_fn(p, m, |i, j| d[i][j]);

    let a_bal = &tr_inv * &a_mat * &tr;
    let b_bal = &tr_inv * &b_mat;
    let c_bal = &c_mat * &tr;

    let r = order;
    let n2 = n - r;

    let a11 = a_bal.view((0, 0), (r, r));
    let a12 = a_bal.view((0, r), (r, n2));
    let a21 = a_bal.view((r, 0), (n2, r));
    let a22 = a_bal.view((r, r), (n2, n2));
    let b1 = b_bal.view((0, 0), (r, m));
    let b2 = b_bal.view((r, 0), (n2, m));
    let c1 = c_bal.view((0, 0), (p, r));
    let c2 = c_bal.view((0, r), (p, n2));

    let a22_inv = a22.clone_owned().try_inverse().ok_or(Ab09NdError::SingularA22)?;

    let ar_mat = a11 - &a12 * &a22_inv * &a21;
    let br_mat = b1 - &a12 * &a22_inv * &b2;
    let cr_mat = c1 - &c2 * &a22_inv * &a21;
    let dr_mat = d_mat - &c2 * &a22_inv * &b2;

    Ok(Ab09NdResult {
        order: r,
        a: (0..r).map(|i| (0..r).map(|j| ar_mat[(i, j)]).collect()).collect(),
        b: (0..r).map(|i| (0..m).map(|j| br_mat[(i, j)]).collect()).collect(),
        c: (0..p).map(|i| (0..r).map(|j| cr_mat[(i, j)]).collect()).collect(),
        d: (0..p).map(|i| (0..m).map(|j| dr_mat[(i, j)]).collect()).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::ab09nd_spa;

    #[test]
    fn ab09nd_spa_reduces_scalar_to_order_one() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let result = ab09nd_spa('C', &a, &b, &c, &d, 1).unwrap_err();
        let msg = format!("{}", result);
        assert!(msg.contains("InvalidOrder") || msg.contains("order"));
    }

    #[test]
    fn ab09nd_spa_two_state_reduce_to_one() {
        let a = vec![vec![-1.0, 0.0], vec![0.0, -2.0]];
        let b = vec![vec![1.0], vec![1.0]];
        let c = vec![vec![1.0, 1.0]];
        let d = vec![vec![0.0]];
        let result = ab09nd_spa('C', &a, &b, &c, &d, 1).expect("stable 2-state");
        assert_eq!(result.order, 1);
        assert_eq!(result.a.len(), 1);
        assert_eq!(result.a[0].len(), 1);
        assert_eq!(result.b.len(), 1);
        assert_eq!(result.c.len(), 1);
        assert_eq!(result.d.len(), 1);
    }
}
