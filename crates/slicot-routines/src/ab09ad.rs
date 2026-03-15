//! Pure-Rust implementation of `AB09AD` (balanced truncation for stable systems).
//!
//! Model reduction by balanced truncation: compute controllability and observability
//! Gramians via SB03MD, Cholesky factors, SVD for Hankel singular values, then
//! balancing transformation and truncation to order r.

use crate::sb03md_solve;
use nalgebra::DMatrix;
use thiserror::Error;

/// Errors returned by the pure-Rust `AB09AD` implementation.
#[derive(Debug, Error)]
pub enum Ab09AdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("system is not stable (Lyapunov failed)")]
    NotStable,
    #[error("Gramian is not positive definite (Cholesky failed)")]
    GramianNotPositiveDefinite,
    #[error("reduction order {order} must be in 1..{n}")]
    InvalidOrder { order: usize, n: usize },
    #[error(transparent)]
    Lyapunov(#[from] crate::Sb03MdError),
}

/// Result: reduced (Ar, Br, Cr, Dr) and Hankel singular values.
#[derive(Clone, Debug, PartialEq)]
pub struct Ab09AdResult {
    pub order: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
    pub hankel_singular_values: Vec<f64>,
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
    m.iter()
        .map(|row| row.iter().map(|&x| -x).collect())
        .collect()
}

/// Balanced truncation model reduction for stable systems (continuous-time only in this subset).
///
/// Solves A' Wc + Wc A = -B B' and A' Wo + Wo A = -C' C, then balances and truncates to `order`.
///
/// # Errors
///
/// Returns [`Ab09AdError`] if dimensions are wrong, system is not stable, or Cholesky fails.
pub fn ab09ad_balance_truncate(
    dico: char,
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
    order: usize,
) -> Result<Ab09AdResult, Ab09AdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Ab09AdResult {
            order: 0,
            a: vec![],
            b: vec![],
            c: vec![],
            d: d.to_vec(),
            hankel_singular_values: vec![],
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
        return Err(Ab09AdError::IncompatibleDimensions(
            "A n×n, B n×m, C p×n, D p×m".to_string(),
        ));
    }
    if order > n || order == 0 {
        return Err(Ab09AdError::InvalidOrder { order, n });
    }

    let bbt = mat_mul(n, m, b, m, n, &transpose(b));
    let neg_bbt = negate(&bbt);
    let wc_result = sb03md_solve(dico, 'X', 'N', 'N', a, &neg_bbt).map_err(|_| Ab09AdError::NotStable)?;
    let wc = wc_result.x;

    let ctc = mat_mul_at_b(p, n, c, p, n, c);
    let neg_ctc = negate(&ctc);
    let wo_result = sb03md_solve(dico, 'X', 'N', 'N', a, &neg_ctc).map_err(|_| Ab09AdError::NotStable)?;
    let wo = wo_result.x;

    let wc_mat = DMatrix::from_fn(n, n, |i, j| wc[i][j]);
    let wo_mat = DMatrix::from_fn(n, n, |i, j| wo[i][j]);

    let cholesky_wc = wc_mat.cholesky().ok_or(Ab09AdError::GramianNotPositiveDefinite)?;
    let lc = cholesky_wc.l();
    let cholesky_wo = wo_mat.cholesky().ok_or(Ab09AdError::GramianNotPositiveDefinite)?;
    let lo = cholesky_wo.l();

    let lct_lot = lc.transpose() * lo.transpose();
    let svd = nalgebra::linalg::SVD::new(lct_lot, true, true);
    let sigma = svd.singular_values;
    let u = svd.u.unwrap();
    let v_t = svd.v_t.unwrap();

    let mut inv_sqrt_sigma = DMatrix::zeros(n, n);
    for i in 0..n {
        if sigma[i] > 1e-20 {
            inv_sqrt_sigma[(i, i)] = 1.0 / sigma[i].sqrt();
        }
    }

    let tr = &lc * &u * &inv_sqrt_sigma;
    let tl = (lo.transpose() * v_t.transpose()) * &inv_sqrt_sigma;

    let order = order.min(n);
    let tr_r = tr.view((0, 0), (n, order));
    let tl_r = tl.view((0, 0), (order, n));

    let a_mat = DMatrix::from_fn(n, n, |i, j| a[i][j]);
    let b_mat = DMatrix::from_fn(n, m, |i, j| b[i][j]);
    let c_mat = DMatrix::from_fn(p, n, |i, j| c[i][j]);
    let d_mat = DMatrix::from_fn(p, m, |i, j| d[i][j]);

    let ar_mat = &tl_r * &a_mat * &tr_r;
    let br_mat = &tl_r * &b_mat;
    let cr_mat = &c_mat * &tr_r;

    let hsv: Vec<f64> = (0..n).map(|i| sigma[i]).collect();

    Ok(Ab09AdResult {
        order,
        a: (0..order).map(|i| (0..order).map(|j| ar_mat[(i, j)]).collect()).collect(),
        b: (0..order).map(|i| (0..m).map(|j| br_mat[(i, j)]).collect()).collect(),
        c: (0..p).map(|i| (0..order).map(|j| cr_mat[(i, j)]).collect()).collect(),
        d: (0..p).map(|i| (0..m).map(|j| d_mat[(i, j)]).collect()).collect(),
        hankel_singular_values: hsv,
    })
}

#[cfg(test)]
mod tests {
    use super::ab09ad_balance_truncate;

    #[test]
    fn ab09ad_reduces_scalar_to_order_one() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let result = ab09ad_balance_truncate('C', &a, &b, &c, &d, 1).expect("stable scalar");
        assert_eq!(result.order, 1);
        assert_eq!(result.a.len(), 1);
        assert_eq!(result.a[0].len(), 1);
        assert!((result.a[0][0] + 1.0).abs() < 1e-8);
        assert!(!result.hankel_singular_values.is_empty());
    }

    #[test]
    fn ab09ad_returns_err_for_unstable() {
        let a = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let err = ab09ad_balance_truncate('C', &a, &b, &c, &d, 1).unwrap_err();
        let msg = format!("{}", err);
        assert!(
            msg.contains("stable") || msg.contains("Lyapunov") || msg.contains("Gramian"),
            "unstable system should error: {}",
            msg
        );
    }
}
