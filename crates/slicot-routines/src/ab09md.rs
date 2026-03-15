//! Pure-Rust implementation of `AB09MD` (balanced truncation, unstable case).
//!
//! When the system has both stable and unstable poles: compute real Schur form of A,
//! reorder so stable eigenvalues (Re(λ) < 0 for continuous) come first, extract the
//! stable subsystem, then run balanced truncation (AB09AD) on it.

use crate::ab09ad_balance_truncate;
use nalgebra::{linalg::Schur, DMatrix};
use thiserror::Error;

/// Errors returned by the pure-Rust `AB09MD` implementation.
#[derive(Debug, Error)]
pub enum Ab09MdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("reduction order {order} must be in 1..{n_stable} (stable states)")]
    InvalidOrder { order: usize, n_stable: usize },
    #[error("no stable part (all eigenvalues in closed right half-plane)")]
    NoStablePart,
    #[error(transparent)]
    BalanceTruncate(#[from] crate::Ab09AdError),
}

/// Result: reduced stable part (Ar, Br, Cr, Dr).
#[derive(Clone, Debug, PartialEq)]
pub struct Ab09MdResult {
    pub order: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
}

fn to_dmatrix(m: &[Vec<f64>]) -> DMatrix<f64> {
    let r = m.len();
    let c = m.first().map_or(0, Vec::len);
    DMatrix::from_fn(r, c, |i, j| m[i][j])
}

fn from_dmatrix(d: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..d.nrows()).map(|i| (0..d.ncols()).map(|j| d[(i, j)]).collect()).collect()
}

/// Returns the number of stable eigenvalues (Re < 0) and a permutation that puts stable first.
fn stable_permutation(t: &DMatrix<f64>, dico: char) -> (usize, Vec<usize>) {
    let n = t.nrows();
    let evals = t.complex_eigenvalues();
    let mut block_starts = Vec::new();
    let mut i = 0;
    while i < n {
        if i + 1 < n && (t[(i + 1, i)].abs() > 1e-14) {
            block_starts.push((i, 2));
            i += 2;
        } else {
            block_starts.push((i, 1));
            i += 1;
        }
    }
    let mut ev_idx = 0;
    let mut stable_indices = Vec::new();
    let mut unstable_indices = Vec::new();
    for &(start, size) in &block_starts {
        let re = evals[ev_idx].re;
        let stable = if dico == 'C' { re < 0.0 } else { re * re + evals[ev_idx].im * evals[ev_idx].im < 1.0 };
        for j in 0..size {
            if stable {
                stable_indices.push(start + j);
            } else {
                unstable_indices.push(start + j);
            }
        }
        ev_idx += size;
    }
    let k = stable_indices.len();
    let mut perm = stable_indices;
    perm.append(&mut unstable_indices);
    (k, perm)
}

/// Balanced truncation for systems with unstable part: reduce the stable part only.
///
/// # Errors
///
/// Returns [`Ab09MdError`] if dimensions are wrong, there is no stable part, or AB09AD fails.
pub fn ab09md_balance_truncate(
    dico: char,
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
    order: usize,
) -> Result<Ab09MdResult, Ab09MdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Ab09MdResult {
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
        return Err(Ab09MdError::IncompatibleDimensions(
            "A n×n, B n×m, C p×n, D p×m".to_string(),
        ));
    }

    let a_mat = to_dmatrix(a);
    let schur = Schur::new(a_mat);
    let (q_mat, t_mat) = schur.unpack();
    let (k_stable, perm) = stable_permutation(&t_mat, dico);
    if k_stable == 0 {
        return Err(Ab09MdError::NoStablePart);
    }
    if order > k_stable || order == 0 {
        return Err(Ab09MdError::InvalidOrder {
            order,
            n_stable: k_stable,
        });
    }

    let n = t_mat.nrows();
    let mut p_rev = DMatrix::zeros(n, n);
    for (i, &j) in perm.iter().enumerate() {
        p_rev[(i, j)] = 1.0;
    }
    let t_ordered = &p_rev * &t_mat * p_rev.transpose();
    let b_mat = DMatrix::from_fn(n, m, |i, j| b[i][j]);
    let c_mat = DMatrix::from_fn(p, n, |i, j| c[i][j]);
    let qt_b = q_mat.transpose() * &b_mat;
    let b_ordered = &p_rev * &qt_b;
    let c_ordered = &c_mat * &q_mat * p_rev.transpose();

    let as_mat = t_ordered.view((0, 0), (k_stable, k_stable)).clone_owned();
    let bs_mat = b_ordered.view((0, 0), (k_stable, m)).clone_owned();
    let cs_mat = c_ordered.view((0, 0), (p, k_stable)).clone_owned();

    let as_vec = from_dmatrix(&as_mat);
    let bs_vec = from_dmatrix(&bs_mat);
    let cs_vec = from_dmatrix(&cs_mat);

    let reduced = ab09ad_balance_truncate(dico, &as_vec, &bs_vec, &cs_vec, d, order)?;

    Ok(Ab09MdResult {
        order: reduced.order,
        a: reduced.a,
        b: reduced.b,
        c: reduced.c,
        d: reduced.d,
    })
}

#[cfg(test)]
mod tests {
    use super::ab09md_balance_truncate;

    #[test]
    fn ab09md_all_stable_is_ab09ad() {
        let a = vec![vec![-1.0, 0.0], vec![0.0, -2.0]];
        let b = vec![vec![1.0], vec![1.0]];
        let c = vec![vec![1.0, 1.0]];
        let d = vec![vec![0.0]];
        let result = ab09md_balance_truncate('C', &a, &b, &c, &d, 1).expect("all stable");
        assert_eq!(result.order, 1);
        assert_eq!(result.a.len(), 1);
        assert_eq!(result.d.len(), 1);
    }

    #[test]
    fn ab09md_no_stable_part_errors() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 2.0]];
        let b = vec![vec![1.0], vec![1.0]];
        let c = vec![vec![1.0, 1.0]];
        let d = vec![vec![0.0]];
        let err = ab09md_balance_truncate('C', &a, &b, &c, &d, 1).unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("stable") || msg.contains("NoStable"));
    }
}
