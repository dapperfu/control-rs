//! Pure-Rust implementation of the `SB02MD` continuous algebraic Riccati equation (CARE) subset.
//!
//! Solves A'X + XA - XGX + Q = 0 for the stabilizing solution X using the
//! Hamiltonian eigenstructure method.

use nalgebra::{linalg::SVD, DMatrix};
use num_complex::Complex64;
use thiserror::Error;

/// Errors returned by the pure-Rust `SB02MD` implementation.
#[derive(Debug, Error)]
pub enum Sb02MdError {
    /// The matrix is not square.
    #[error("expected square matrix with {expected} rows, found {actual}")]
    NonSquareMatrix { expected: usize, actual: usize },
    /// The Hamiltonian has no stabilizing solution (e.g. imaginary axis eigenvalues).
    #[error("no stabilizing solution: {0}")]
    NoStabilizingSolution(String),
    /// The stable invariant subspace is singular (U11 not invertible).
    #[error("stable subspace matrix is singular")]
    SingularStableSubspace,
    /// Only continuous-time CARE is supported.
    #[error("DICO='{0}' not supported; only 'C' (continuous) is implemented")]
    UnsupportedDico(char),
}

/// Result of solving the CARE.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb02MdResult {
    /// Stabilizing solution X (n×n symmetric).
    pub x: Vec<Vec<f64>>,
    /// Reciprocal condition number estimate.
    pub rcond: f64,
}

/// Solves the continuous algebraic Riccati equation A'X + XA - XGX + Q = 0
/// for the stabilizing symmetric solution X.
///
/// Uses the Hamiltonian matrix H = [A -G; -Q -A']; the solution is obtained
/// from the stable invariant subspace (eigenvalues with Re(λ) < 0).
///
/// # Errors
///
/// Returns [`Sb02MdError`] if dimensions are wrong, DICO is not 'C', or
/// no stabilizing solution exists.
pub fn sb02md_solve(
    _dico: char,
    a: &[Vec<f64>],
    q: &[Vec<f64>],
    g: &[Vec<f64>],
) -> Result<Sb02MdResult, Sb02MdError> {
    if _dico != 'C' {
        return Err(Sb02MdError::UnsupportedDico(_dico));
    }
    let n = a.len();
    if n == 0 {
        return Ok(Sb02MdResult {
            x: vec![],
            rcond: 1.0,
        });
    }
    if a.iter().any(|row| row.len() != n) {
        return Err(Sb02MdError::NonSquareMatrix {
            expected: n,
            actual: a.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if q.len() != n || q.iter().any(|row| row.len() != n) {
        return Err(Sb02MdError::NonSquareMatrix {
            expected: n,
            actual: q.len(),
        });
    }
    if g.len() != n || g.iter().any(|row| row.len() != n) {
        return Err(Sb02MdError::NonSquareMatrix {
            expected: n,
            actual: g.len(),
        });
    }

    let h = build_hamiltonian(a, q, g);
    let (v1, v2) = stable_invariant_subspace(&h, n)?;
    let u11 = DMatrix::from_fn(n, n, |i, j| v1[i][j]);
    let u21 = DMatrix::from_fn(n, n, |i, j| v2[i][j]);

    let u11_inv = u11
        .clone()
        .try_inverse()
        .ok_or(Sb02MdError::SingularStableSubspace)?;
    let mut x_mat = &u21 * &u11_inv;
    symmetrize(&mut x_mat);
    let x = matrix_to_vec_vec(&x_mat);

    let rcond = estimate_rcond(&u11, &u11_inv, &x_mat);

    Ok(Sb02MdResult { x, rcond })
}

fn build_hamiltonian(a: &[Vec<f64>], q: &[Vec<f64>], g: &[Vec<f64>]) -> DMatrix<f64> {
    let n = a.len();
    let at = transpose_slice(a);
    let mut h = DMatrix::zeros(2 * n, 2 * n);
    for i in 0..n {
        for j in 0..n {
            h[(i, j)] = a[i][j];
            h[(i, n + j)] = -g[i][j];
            h[(n + i, j)] = -q[i][j];
            h[(n + i, n + j)] = -at[i][j];
        }
    }
    h
}

fn transpose_slice(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = m.len();
    let cols = m.first().map_or(0, Vec::len);
    let mut t = vec![vec![0.0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            t[j][i] = m[i][j];
        }
    }
    t
}

fn stable_invariant_subspace(
    h: &DMatrix<f64>,
    n: usize,
) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>), Sb02MdError> {
    let eigenvalues = h.complex_eigenvalues();
    let mut stable_indices = Vec::with_capacity(n);
    for (i, &lam) in eigenvalues.iter().enumerate() {
        if lam.re < -1.0e-10 {
            stable_indices.push(i);
        }
    }
    if stable_indices.len() != n {
        return Err(Sb02MdError::NoStabilizingSolution(format!(
            "expected {} stable eigenvalues, found {}",
            n,
            stable_indices.len()
        )));
    }

    let mut columns: Vec<Vec<f64>> = Vec::with_capacity(n);
    let mut used = vec![false; eigenvalues.len()];
    for &idx in &stable_indices {
        if columns.len() >= n {
            break;
        }
        if used[idx] {
            continue;
        }
        let lam = eigenvalues[idx];
        if lam.im.abs() < 1.0e-10 {
            let v = real_null_vector(h, lam.re)?;
            columns.push(v);
            used[idx] = true;
        } else {
            let v = complex_null_vector(h, lam)?;
            let v_re: Vec<f64> = v.iter().map(|c| c.re).collect();
            let v_im: Vec<f64> = v.iter().map(|c| c.im).collect();
            columns.push(v_re);
            columns.push(v_im);
            used[idx] = true;
            if let Some(conj_idx) = eigenvalues
                .iter()
                .position(|&l| (l - lam.conj()).norm() < 1.0e-10)
            {
                used[conj_idx] = true;
            }
        }
    }
    if columns.len() < n {
        return Err(Sb02MdError::NoStabilizingSolution(
            "could not form real basis of stable subspace".to_string(),
        ));
    }
    columns.truncate(n);
    let v1: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| columns[j][i]).collect()).collect();
    let v2: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| columns[j][n + i]).collect())
        .collect();
    Ok((v1, v2))
}

fn real_null_vector(h: &DMatrix<f64>, lambda: f64) -> Result<Vec<f64>, Sb02MdError> {
    let n = h.nrows();
    let shifted = h - (lambda * DMatrix::identity(n, n));
    let svd = SVD::new(shifted.clone(), true, true);
    let v_t = svd.v_t.as_ref().ok_or(Sb02MdError::SingularStableSubspace)?;
    let mut col = vec![0.0; n];
    for i in 0..n {
        col[i] = v_t[(n - 1, i)];
    }
    Ok(col)
}

fn complex_null_vector(h: &DMatrix<f64>, lambda: Complex64) -> Result<Vec<Complex64>, Sb02MdError> {
    let n = h.nrows();
    let h_complex: DMatrix<Complex64> = DMatrix::from_fn(n, n, |i, j| Complex64::new(h[(i, j)], 0.0));
    let id = DMatrix::from_fn(n, n, |i, j| {
        if i == j {
            lambda
        } else {
            Complex64::new(0.0, 0.0)
        }
    });
    let shifted = h_complex - id;
    let svd = SVD::new(shifted, true, true);
    let v_t = svd.v_t.as_ref().ok_or(Sb02MdError::SingularStableSubspace)?;
    let mut col = vec![Complex64::new(0.0, 0.0); n];
    for i in 0..n {
        col[i] = v_t[(n - 1, i)];
    }
    Ok(col)
}

fn symmetrize(m: &mut DMatrix<f64>) {
    let n = m.nrows();
    for i in 0..n {
        for j in (i + 1)..n {
            let s = (m[(i, j)] + m[(j, i)]) / 2.0;
            m[(i, j)] = s;
            m[(j, i)] = s;
        }
    }
}

fn matrix_to_vec_vec(m: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| m[(i, j)]).collect())
        .collect()
}

fn estimate_rcond(
    u11: &DMatrix<f64>,
    u11_inv: &DMatrix<f64>,
    x: &DMatrix<f64>,
) -> f64 {
    let u11_norm = one_norm(u11);
    let u11_inv_norm = one_norm(u11_inv);
    if u11_norm <= 0.0 || u11_inv_norm <= 0.0 {
        return 0.0;
    }
    let cond_u11 = u11_norm * u11_inv_norm;
    let x_norm = one_norm(x);
    if x_norm <= 0.0 {
        return 1.0 / cond_u11;
    }
    (1.0 / cond_u11).min(1.0 / (x_norm * u11_inv_norm))
}

fn one_norm(m: &DMatrix<f64>) -> f64 {
    (0..m.ncols())
        .map(|j| (0..m.nrows()).map(|i| m[(i, j)].abs()).sum::<f64>())
        .fold(0.0_f64, f64::max)
}
