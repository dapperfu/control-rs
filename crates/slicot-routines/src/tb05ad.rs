//! Pure-Rust implementation of the `TB05AD` frequency-response routine.

use nalgebra::DMatrix;
use num_complex::Complex64;
use slicot_linalg::{
    matrix_one_norm, multiply_real_by_complex, solve_complex_system, ComplexMatrixError,
};
use thiserror::Error;

/// Errors returned by the pure-Rust `TB05AD` implementation.
#[derive(Debug, Error)]
pub enum Tb05AdError {
    /// The state matrix is not square.
    #[error("expected a square state matrix, found {rows}x{columns}")]
    NonSquareStateMatrix { rows: usize, columns: usize },
    /// The input matrix row count does not match the state dimension.
    #[error("input matrix row count {actual} does not match state dimension {expected}")]
    IncompatibleInputMatrix { expected: usize, actual: usize },
    /// The output matrix column count does not match the state dimension.
    #[error("output matrix column count {actual} does not match state dimension {expected}")]
    IncompatibleOutputMatrix { expected: usize, actual: usize },
    /// The linear solve failed.
    #[error(transparent)]
    LinearSolve(#[from] ComplexMatrixError),
}

/// Output bundle for the pure-Rust `TB05AD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb05AdResult {
    pub transformed_a: Vec<Vec<f64>>,
    pub transformed_b: Vec<Vec<f64>>,
    pub transformed_c: Vec<Vec<f64>>,
    pub g: Vec<Vec<Complex64>>,
    pub hinvb: Vec<Vec<Complex64>>,
    pub eigenvalues: Option<Vec<Complex64>>,
    pub rcond: Option<f64>,
}

/// Evaluates the state-space frequency response
/// `G(freq) = C * (freq * I - A)^(-1) * B`.
///
/// This initial pure-Rust implementation preserves the external behavior needed
/// by the upstream `TB05AD` example and by `python-control`'s repeated
/// frequency-response path. It does not yet apply balancing or Hessenberg
/// reduction, so the returned transformed matrices are currently the original
/// input matrices.
///
/// # Errors
///
/// Returns [`Tb05AdError`] if the matrix dimensions are incompatible or the
/// shifted system matrix is singular to working precision.
pub fn tb05ad_frequency_response(
    baleig: char,
    inita: char,
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    freq: Complex64,
) -> Result<Tb05AdResult, Tb05AdError> {
    let state_count = a.len();
    if a.iter().any(|row| row.len() != state_count) {
        return Err(Tb05AdError::NonSquareStateMatrix {
            rows: state_count,
            columns: a.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if b.len() != state_count {
        return Err(Tb05AdError::IncompatibleInputMatrix {
            expected: state_count,
            actual: b.len(),
        });
    }
    if c.iter().any(|row| row.len() != state_count) {
        return Err(Tb05AdError::IncompatibleOutputMatrix {
            expected: state_count,
            actual: c.iter().map(Vec::len).max().unwrap_or(0),
        });
    }

    let shifted_matrix = build_shifted_matrix(a, freq);
    let complex_b = b
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| Complex64::new(*value, 0.0))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let hinvb = solve_complex_system(&shifted_matrix, &complex_b)?;
    let g = multiply_real_by_complex(c, &hinvb);

    let needs_rcond = matches!(baleig, 'C' | 'A');
    let rcond = needs_rcond
        .then(|| estimate_rcond(&shifted_matrix))
        .transpose()?;

    let needs_eigenvalues = matches!(inita, 'G') && matches!(baleig, 'B' | 'E' | 'A');
    let eigenvalues = needs_eigenvalues.then(|| compute_eigenvalues(a));

    Ok(Tb05AdResult {
        transformed_a: a.to_vec(),
        transformed_b: b.to_vec(),
        transformed_c: c.to_vec(),
        g,
        hinvb,
        eigenvalues,
        rcond,
    })
}

fn build_shifted_matrix(a: &[Vec<f64>], freq: Complex64) -> Vec<Vec<Complex64>> {
    a.iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(column_index, value)| {
                    if row_index == column_index {
                        freq - Complex64::new(*value, 0.0)
                    } else {
                        Complex64::new(-*value, 0.0)
                    }
                })
                .collect()
        })
        .collect()
}

fn estimate_rcond(shifted_matrix: &[Vec<Complex64>]) -> Result<f64, Tb05AdError> {
    let order = shifted_matrix.len();
    let identity = (0..order)
        .map(|row_index| {
            (0..order)
                .map(|column_index| {
                    if row_index == column_index {
                        Complex64::new(1.0, 0.0)
                    } else {
                        Complex64::new(0.0, 0.0)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let inverse = solve_complex_system(shifted_matrix, &identity)?;
    let matrix_norm = matrix_one_norm(shifted_matrix);
    let inverse_norm = matrix_one_norm(&inverse);
    Ok(1.0 / (matrix_norm * inverse_norm))
}

fn compute_eigenvalues(a: &[Vec<f64>]) -> Vec<Complex64> {
    let order = a.len();
    let flattened = a.iter().flatten().copied().collect::<Vec<_>>();
    let matrix = DMatrix::from_row_slice(order, order, &flattened);
    matrix.complex_eigenvalues().iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::tb05ad_frequency_response;
    use num_complex::Complex64;

    #[test]
    fn evaluates_scalar_frequency_response() {
        let a = vec![vec![1.0]];
        let b = vec![vec![2.0]];
        let c = vec![vec![3.0]];

        let result = tb05ad_frequency_response('N', 'G', &a, &b, &c, Complex64::new(2.0, 0.0))
            .expect("frequency response should evaluate");

        assert_eq!(result.g.len(), 1);
        assert!((result.g[0][0] - Complex64::new(6.0, 0.0)).norm() < 1.0e-12);
    }
}
