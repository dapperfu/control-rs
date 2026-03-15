//! Pure-Rust implementation of the `SB04QD` discrete Sylvester solver subset.

use nalgebra::{linalg::Schur, DMatrix};
use num_complex::Complex64;
use slicot_linalg::{solve_complex_system, ComplexMatrixError};
use thiserror::Error;

/// Errors returned by the pure-Rust `SB04QD` implementation.
#[derive(Debug, Error)]
pub enum Sb04QdError {
    /// The left coefficient matrix is not square.
    #[error("expected square matrix A, found {rows}x{columns}")]
    NonSquareLeftMatrix { rows: usize, columns: usize },
    /// The right coefficient matrix is not square.
    #[error("expected square matrix B, found {rows}x{columns}")]
    NonSquareRightMatrix { rows: usize, columns: usize },
    /// The right-hand side has incompatible dimensions.
    #[error("expected C with shape {expected_rows}x{expected_columns}, found {actual_rows}x{actual_columns}")]
    IncompatibleRightHandSide {
        expected_rows: usize,
        expected_columns: usize,
        actual_rows: usize,
        actual_columns: usize,
    },
    /// The linear solve failed.
    #[error(transparent)]
    LinearSolve(#[from] ComplexMatrixError),
}

/// Output bundle for the pure-Rust `SB04QD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04QdResult {
    pub x: Vec<Vec<f64>>,
    pub z: Vec<Vec<f64>>,
}

/// Solves the discrete-time Sylvester equation `X + A X B = C`.
///
/// # Errors
///
/// Returns [`Sb04QdError`] if the input dimensions are incompatible or the
/// associated Kronecker system is singular.
pub fn sb04qd_solve(
    left_matrix: &[Vec<f64>],
    right_matrix: &[Vec<f64>],
    rhs_matrix: &[Vec<f64>],
) -> Result<Sb04QdResult, Sb04QdError> {
    let left_order = left_matrix.len();
    let right_order = right_matrix.len();

    if left_matrix.iter().any(|row| row.len() != left_order) {
        return Err(Sb04QdError::NonSquareLeftMatrix {
            rows: left_order,
            columns: left_matrix.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if right_matrix.iter().any(|row| row.len() != right_order) {
        return Err(Sb04QdError::NonSquareRightMatrix {
            rows: right_order,
            columns: right_matrix.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if rhs_matrix.len() != left_order || rhs_matrix.iter().any(|row| row.len() != right_order) {
        return Err(Sb04QdError::IncompatibleRightHandSide {
            expected_rows: left_order,
            expected_columns: right_order,
            actual_rows: rhs_matrix.len(),
            actual_columns: rhs_matrix.iter().map(Vec::len).max().unwrap_or(0),
        });
    }

    let kronecker_system = build_discrete_sylvester_system(left_matrix, right_matrix);
    let rhs = vectorize_rectangular_matrix(rhs_matrix)
        .into_iter()
        .map(|value| vec![Complex64::new(value, 0.0)])
        .collect::<Vec<_>>();
    let solution = solve_complex_system(&kronecker_system, &rhs)?;
    let solution_matrix = unvectorize_rectangular_matrix(
        &solution
            .into_iter()
            .map(|column| column[0].re)
            .collect::<Vec<_>>(),
        left_order,
        right_order,
    );
    let schur_vectors = schur_vectors_of_transpose(right_matrix);

    Ok(Sb04QdResult {
        x: solution_matrix,
        z: schur_vectors,
    })
}

/// Solves the discrete-time Sylvester equation `X + A X B = scale * C` (SB04PD API).
///
/// Equivalent to [`sb04qd_solve`](crate::sb04qd_solve).
pub fn sb04pd_solve(
    left_matrix: &[Vec<f64>],
    right_matrix: &[Vec<f64>],
    rhs_matrix: &[Vec<f64>],
) -> Result<Sb04QdResult, Sb04QdError> {
    sb04qd_solve(left_matrix, right_matrix, rhs_matrix)
}

fn build_discrete_sylvester_system(
    left_matrix: &[Vec<f64>],
    right_matrix: &[Vec<f64>],
) -> Vec<Vec<Complex64>> {
    let left_order = left_matrix.len();
    let right_order = right_matrix.len();
    let kron_term = kronecker(&transpose(right_matrix), left_matrix);
    let identity_matrix = identity(left_order * right_order);
    add_real_matrices(&identity_matrix, &kron_term)
}

fn schur_vectors_of_transpose(right_matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let order = right_matrix.len();
    let transpose_b = transpose(right_matrix);
    let flattened = transpose_b.iter().flatten().copied().collect::<Vec<_>>();
    let matrix = DMatrix::from_row_slice(order, order, &flattened);
    let schur = Schur::new(matrix);
    let (q, _t) = schur.unpack();

    (0..order)
        .map(|row_index| {
            (0..order)
                .map(|column_index| q[(row_index, column_index)])
                .collect()
        })
        .collect()
}

fn transpose(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let row_count = matrix.len();
    let column_count = matrix.first().map_or(0, Vec::len);
    let mut transposed = vec![vec![0.0; row_count]; column_count];

    for (row_index, row) in matrix.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            transposed[column_index][row_index] = *value;
        }
    }

    transposed
}

fn identity(order: usize) -> Vec<Vec<f64>> {
    (0..order)
        .map(|row_index| {
            (0..order)
                .map(|column_index| if row_index == column_index { 1.0 } else { 0.0 })
                .collect()
        })
        .collect()
}

fn kronecker(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let left_rows = left.len();
    let left_cols = left.first().map_or(0, Vec::len);
    let right_rows = right.len();
    let right_cols = right.first().map_or(0, Vec::len);
    let mut output = vec![vec![0.0; left_cols * right_cols]; left_rows * right_rows];

    for left_row in 0..left_rows {
        for left_col in 0..left_cols {
            for right_row in 0..right_rows {
                for right_col in 0..right_cols {
                    output[left_row * right_rows + right_row][left_col * right_cols + right_col] =
                        left[left_row][left_col] * right[right_row][right_col];
                }
            }
        }
    }

    output
}

fn add_real_matrices(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<Complex64>> {
    left.iter()
        .zip(right)
        .map(|(left_row, right_row)| {
            left_row
                .iter()
                .zip(right_row)
                .map(|(left_value, right_value)| Complex64::new(left_value + right_value, 0.0))
                .collect()
        })
        .collect()
}

fn vectorize_rectangular_matrix(matrix: &[Vec<f64>]) -> Vec<f64> {
    let row_count = matrix.len();
    let column_count = matrix.first().map_or(0, Vec::len);
    let mut values = Vec::with_capacity(row_count * column_count);

    for column_index in 0..column_count {
        for row in matrix.iter().take(row_count) {
            values.push(row[column_index]);
        }
    }

    values
}

fn unvectorize_rectangular_matrix(
    values: &[f64],
    row_count: usize,
    column_count: usize,
) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; column_count]; row_count];
    for column_index in 0..column_count {
        for row_index in 0..row_count {
            matrix[row_index][column_index] = values[column_index * row_count + row_index];
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::sb04qd_solve;

    #[test]
    fn solves_scalar_discrete_sylvester_case() {
        let a = vec![vec![2.0]];
        let b = vec![vec![3.0]];
        let c = vec![vec![11.0]];
        let result = sb04qd_solve(&a, &b, &c).expect("solve should succeed");

        assert!((result.x[0][0] - (11.0 / 7.0)).abs() < 1.0e-12);
    }
}
