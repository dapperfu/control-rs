//! Pure-Rust implementation of the `TB01PD` minimal realization subset.
//!
//! Computes a minimal state-space realization (A, B, C) by projecting onto the
//! controllable then observable subspace via SVD. The resulting basis may differ
//! from SLICOT's; equivalence is checked via Markov parameters or tolerance.

use nalgebra::{linalg::SVD, DMatrix};
use thiserror::Error;

/// Errors returned by the pure-Rust `TB01PD` implementation.
#[derive(Debug, Error)]
pub enum Tb01PdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
}

/// Result of minimal realization.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb01PdResult {
    /// Minimal order (state dimension).
    pub order: usize,
    /// State dynamics matrix (order × order).
    pub a: Vec<Vec<f64>>,
    /// Input matrix (order × m).
    pub b: Vec<Vec<f64>>,
    /// Output matrix (p × order).
    pub c: Vec<Vec<f64>>,
}

/// Computes a minimal state-space realization of (A, B, C) by removing
/// uncontrollable and unobservable states via SVD-based projection.
///
/// # Errors
///
/// Returns [`Tb01PdError`] if dimensions are incompatible.
pub fn tb01pd_minreal(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
) -> Result<Tb01PdResult, Tb01PdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Tb01PdResult {
            order: 0,
            a: vec![],
            b: vec![],
            c: vec![],
        });
    }
    if a.iter().any(|row| row.len() != n) {
        return Err(Tb01PdError::IncompatibleDimensions("A must be square".to_string()));
    }
    let _m = b.first().map_or(0, Vec::len);
    let _p = c.len();
    if b.len() != n || c.iter().any(|row| row.len() != n) {
        return Err(Tb01PdError::IncompatibleDimensions(
            "B rows and C columns must match n".to_string(),
        ));
    }

    let (a_mat, b_mat, c_mat) = to_nalgebra(a, b, c);

    // Controllability matrix: [B, A*B, A^2*B, ..., A^{n-1}*B]
    let cont = controllability_matrix(&a_mat, &b_mat);
    let r_c = rank_svd(&cont, 1.0e-10);
    if r_c == 0 {
        return Ok(Tb01PdResult {
            order: 0,
            a: vec![],
            b: vec![],
            c: vec![],
        });
    }
    let u_c = left_basis(&cont, r_c);
    let a_c = &u_c.transpose() * &a_mat * &u_c;
    let b_c = u_c.transpose() * &b_mat;
    let c_c = &c_mat * &u_c;

    // Observability matrix of reduced system: [C_c; C_c*A_c; ...]
    // Row space of O (right singular vectors) is the observable subspace in state space.
    let obs = observability_matrix(&a_c, &c_c);
    let r = rank_svd(&obs, 1.0e-10);
    if r == 0 {
        return Ok(Tb01PdResult {
            order: 0,
            a: vec![],
            b: vec![],
            c: vec![],
        });
    }
    let u_o = right_basis(&obs, r);
    let am = &u_o * &a_c * u_o.transpose();
    let bm = &u_o * &b_c;
    let cm = &c_c * u_o.transpose();

    Ok(Tb01PdResult {
        order: r,
        a: matrix_to_vec_vec(&am),
        b: matrix_to_vec_vec(&bm),
        c: matrix_to_vec_vec(&cm),
    })
}

fn to_nalgebra(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
) -> (DMatrix<f64>, DMatrix<f64>, DMatrix<f64>) {
    let n = a.len();
    let m = b.first().map_or(0, Vec::len);
    let p = c.len();
    let a_mat = DMatrix::from_fn(n, n, |i, j| a[i][j]);
    let b_mat = DMatrix::from_fn(n, m, |i, j| b[i][j]);
    let c_mat = DMatrix::from_fn(p, n, |i, j| c[i][j]);
    (a_mat, b_mat, c_mat)
}

fn controllability_matrix(a: &DMatrix<f64>, b: &DMatrix<f64>) -> DMatrix<f64> {
    let n = a.nrows();
    let m = b.ncols();
    let mut blocks = vec![b.clone()];
    let mut apow = a.clone();
    for _ in 1..n {
        let next = &apow * b;
        blocks.push(next);
        apow = &apow * a;
    }
    DMatrix::from_fn(n, n * m, |i, j| {
        let block = j / m;
        let col = j % m;
        blocks[block][(i, col)]
    })
}

fn observability_matrix(a: &DMatrix<f64>, c: &DMatrix<f64>) -> DMatrix<f64> {
    let n = a.nrows();
    let p = c.nrows();
    let mut rows = Vec::with_capacity(n * p);
    let mut crow = c.clone();
    for _ in 0..n {
        for i in 0..p {
            let row: Vec<f64> = (0..crow.ncols()).map(|j| crow[(i, j)]).collect();
            rows.push(row);
        }
        crow = crow * a;
    }
    let nrows = rows.len();
    let ncols = rows.first().map_or(0, Vec::len);
    DMatrix::from_fn(nrows, ncols, |i, j| rows[i][j])
}

fn rank_svd(m: &DMatrix<f64>, tol: f64) -> usize {
    let svd = SVD::new(m.clone(), false, false);
    svd.singular_values
        .iter()
        .filter(|s| **s > tol)
        .count()
}

fn left_basis(m: &DMatrix<f64>, rank: usize) -> DMatrix<f64> {
    let svd = SVD::new(m.clone(), true, false);
    let u = svd.u.expect("U requested");
    let nrows = u.nrows();
    DMatrix::from_fn(nrows, rank, |i, j| u[(i, j)])
}

/// First `rank` right singular vectors (first `rank` rows of V^T), state-space basis for row space.
fn right_basis(m: &DMatrix<f64>, rank: usize) -> DMatrix<f64> {
    let svd = SVD::new(m.clone(), false, true);
    let v_t = svd.v_t.expect("V^T requested");
    let ncols = v_t.ncols();
    DMatrix::from_fn(rank, ncols, |i, j| v_t[(i, j)])
}

fn matrix_to_vec_vec(m: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| m[(i, j)]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::tb01pd_minreal;

    #[test]
    fn minimal_realization_reduces_when_not_fully_controllable_observable() {
        // A=I2, B=[1;0], C=[1,0]: only one state is controllable and observable -> order 1.
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![vec![1.0], vec![0.0]];
        let c = vec![vec![1.0, 0.0]];
        let result = tb01pd_minreal(&a, &b, &c).expect("minreal should succeed");
        assert_eq!(result.order, 1);
        assert_eq!(result.a.len(), 1);
        assert_eq!(result.b.len(), 1);
        assert_eq!(result.c.len(), 1);
    }

    #[test]
    fn minimal_realization_preserves_order_when_already_minimal() {
        // Controllable and observable 2-state system: [C; C*A] and [B, A*B] full rank.
        let a = vec![vec![0.0, 1.0], vec![0.0, 0.0]];
        let b = vec![vec![0.0], vec![1.0]];
        let c = vec![vec![1.0, 0.0]];
        let result = tb01pd_minreal(&a, &b, &c).expect("minreal should succeed");
        assert_eq!(result.order, 2);
        assert_eq!(result.a.len(), 2);
        assert_eq!(result.b.len(), 2);
        assert_eq!(result.c.len(), 1);
    }
}
