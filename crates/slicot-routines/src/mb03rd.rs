//! Pure-Rust implementation of `MB03RD` (block diagonalization of real Schur form).
//!
//! Reorders blocks of a real Schur matrix so that blocks with `select[i] == true` come first.
//! If the input is a general matrix A, computes Schur form A = Q T Q' then reorders T.

use nalgebra::{linalg::Schur, DMatrix};
use thiserror::Error;

/// Errors returned by the pure-Rust `MB03RD` implementation.
#[derive(Debug, Error)]
pub enum Mb03RdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("select length {actual} must equal matrix order {expected}")]
    SelectLength { expected: usize, actual: usize },
}

/// Result: reordered block-diagonal blocks (1×1 or 2×2), and the reordered Schur matrix.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mb03RdResult {
    pub blocks: Vec<Vec<Vec<f64>>>,
    pub t_ordered: Vec<Vec<f64>>,
    pub q_ordered: Vec<Vec<f64>>,
}

fn to_dmatrix(m: &[Vec<f64>]) -> DMatrix<f64> {
    let r = m.len();
    let c = m.first().map_or(0, Vec::len);
    DMatrix::from_fn(r, c, |i, j| m[i][j])
}

fn from_dmatrix(d: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..d.nrows()).map(|i| (0..d.ncols()).map(|j| d[(i, j)]).collect()).collect()
}

/// Identifies diagonal blocks (1×1 or 2×2) of real Schur T and builds permutation so selected first.
fn block_permutation(t: &DMatrix<f64>, select: &[bool]) -> (Vec<Vec<Vec<f64>>>, Vec<usize>) {
    let n = t.nrows();
    let mut block_info = Vec::new();
    let mut i = 0;
    while i < n {
        let size = if i + 1 < n && (t[(i + 1, i)].abs() > 1e-14) { 2 } else { 1 };
        let sel = select.get(i).copied().unwrap_or(false);
        let mut blk = vec![vec![0.0; size]; size];
        for r in 0..size {
            for c in 0..size {
                blk[r][c] = t[(i + r, i + c)];
            }
        }
        block_info.push((i, size, sel, blk));
        i += size;
    }
    let mut selected_blocks = Vec::new();
    let mut selected_indices = Vec::new();
    let mut other_blocks = Vec::new();
    let mut other_indices = Vec::new();
    for (start, size, sel, blk) in block_info {
        if sel {
            selected_blocks.push(blk);
            for j in 0..size {
                selected_indices.push(start + j);
            }
        } else {
            other_blocks.push(blk);
            for j in 0..size {
                other_indices.push(start + j);
            }
        }
    }
    let mut perm = selected_indices;
    perm.append(&mut other_indices);
    let mut blocks = selected_blocks;
    blocks.append(&mut other_blocks);
    (blocks, perm)
}

/// Block diagonalizes real Schur form by reordering blocks.
///
/// If `schur_a` is already in real Schur form (upper quasi-triangular), it is reordered.
/// If it is a general matrix, Schur decomposition is computed first.
/// `select[i]` is the selection flag for the block containing row/column `i`.
///
/// # Errors
///
/// Returns [`Mb03RdError`] if dimensions are wrong or select length does not match order.
pub fn mb03rd_block_diagonalize(
    schur_a: &[Vec<f64>],
    select: &[bool],
) -> Result<Mb03RdResult, Mb03RdError> {
    let n = schur_a.len();
    if n == 0 {
        return Ok(Mb03RdResult {
            blocks: vec![],
            t_ordered: vec![],
            q_ordered: vec![],
        });
    }
    if schur_a.iter().any(|row| row.len() != n) {
        return Err(Mb03RdError::IncompatibleDimensions(
            "matrix must be square".to_string(),
        ));
    }
    if select.len() != n {
        return Err(Mb03RdError::SelectLength {
            expected: n,
            actual: select.len(),
        });
    }

    let a_mat = to_dmatrix(schur_a);
    let schur = Schur::new(a_mat);
    let (q_mat, t_mat) = schur.unpack();

    let (blocks, perm) = block_permutation(&t_mat, select);

    let mut p_rev = DMatrix::zeros(n, n);
    for (i, &j) in perm.iter().enumerate() {
        p_rev[(i, j)] = 1.0;
    }
    let t_ordered = &p_rev * &t_mat * p_rev.transpose();
    let q_ordered = &q_mat * p_rev.transpose();

    Ok(Mb03RdResult {
        blocks,
        t_ordered: from_dmatrix(&t_ordered),
        q_ordered: from_dmatrix(&q_ordered),
    })
}

#[cfg(test)]
mod tests {
    use super::{mb03rd_block_diagonalize, Mb03RdError};

    #[test]
    fn mb03rd_reorders_2x2_diagonal() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 2.0]];
        let select = vec![false, true];
        let result = mb03rd_block_diagonalize(&a, &select).expect("2x2");
        assert_eq!(result.blocks.len(), 2);
        assert_eq!(result.t_ordered.len(), 2);
        assert_eq!(result.q_ordered.len(), 2);
    }

    #[test]
    fn mb03rd_select_length_mismatch_errors() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 2.0]];
        let select = vec![true];
        let err = mb03rd_block_diagonalize(&a, &select).unwrap_err();
        assert!(matches!(err, Mb03RdError::SelectLength { .. }));
    }
}
