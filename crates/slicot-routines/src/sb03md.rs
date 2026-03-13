//! Pure-Rust implementation of the `SB03MD` Lyapunov solver subset.

use num_complex::Complex64;
use slicot_linalg::{solve_complex_system, ComplexMatrixError};
use thiserror::Error;

/// Errors returned by the pure-Rust `SB03MD` implementation.
#[derive(Debug, Error)]
pub enum Sb03MdError {
    /// The input matrix is not square.
    #[error("expected a square matrix with {expected} rows, found {actual}")]
    NonSquareMatrix { expected: usize, actual: usize },
    /// The right-hand side does not match the state dimension.
    #[error("right-hand side row count {actual} does not match state dimension {expected}")]
    IncompatibleRightHandSide { expected: usize, actual: usize },
    /// The current pure-Rust subset only supports direct solves without a
    /// precomputed Schur factorization.
    #[error("FACT='{fact}' is not supported by the current pure-Rust SB03MD subset")]
    UnsupportedFact { fact: char },
    /// The current pure-Rust subset only supports solution-only mode.
    #[error("JOB='{job}' is not supported by the current pure-Rust SB03MD subset")]
    UnsupportedJob { job: char },
    /// The current pure-Rust subset only supports continuous- and discrete-time
    /// Lyapunov equations.
    #[error("DICO='{dico}' is not supported by the current pure-Rust SB03MD subset")]
    UnsupportedDico { dico: char },
    /// The linear solve failed.
    #[error(transparent)]
    LinearSolve(#[from] ComplexMatrixError),
}

/// Output bundle for the pure-Rust `SB03MD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03MdResult {
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Solves the supported subset of the `SB03MD` Lyapunov equations.
///
/// Supported subset:
/// - `job = 'X'`
/// - `fact = 'N'`
/// - `dico = 'C'` or `dico = 'D'`
/// - `trana = 'N'` or `trana = 'T'`
///
/// # Errors
///
/// Returns [`Sb03MdError`] if the matrix dimensions are incompatible, the
/// requested mode is not supported by the current subset, or the resulting
/// Kronecker system is singular.
pub fn sb03md_solve(
    dico: char,
    job: char,
    fact: char,
    trana: char,
    a: &[Vec<f64>],
    c: &[Vec<f64>],
) -> Result<Sb03MdResult, Sb03MdError> {
    if !matches!(job, 'X') {
        return Err(Sb03MdError::UnsupportedJob { job });
    }
    if !matches!(fact, 'N') {
        return Err(Sb03MdError::UnsupportedFact { fact });
    }

    let state_count = a.len();
    if a.iter().any(|row| row.len() != state_count) {
        return Err(Sb03MdError::NonSquareMatrix {
            expected: state_count,
            actual: a.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if c.len() != state_count || c.iter().any(|row| row.len() != state_count) {
        return Err(Sb03MdError::IncompatibleRightHandSide {
            expected: state_count,
            actual: c.len(),
        });
    }

    let aop = if matches!(trana, 'N') {
        a.to_vec()
    } else {
        transpose(a)
    };
    let aop_transpose = transpose(&aop);
    let kron_system = match dico {
        'C' => build_continuous_lyapunov_system(&aop_transpose),
        'D' => build_discrete_lyapunov_system(&aop_transpose),
        _ => return Err(Sb03MdError::UnsupportedDico { dico }),
    };
    let rhs = vectorize_real_matrix(c)
        .into_iter()
        .map(|value| vec![Complex64::new(value, 0.0)])
        .collect::<Vec<_>>();
    let solution = solve_complex_system(&kron_system, &rhs)?;
    let x = unvectorize_real_matrix(
        &solution
            .into_iter()
            .map(|column| column[0].re)
            .collect::<Vec<_>>(),
        state_count,
    );

    Ok(Sb03MdResult { x, scale: 1.0 })
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

fn build_continuous_lyapunov_system(a_transpose: &[Vec<f64>]) -> Vec<Vec<Complex64>> {
    let state_count = a_transpose.len();
    let identity = identity(state_count);
    let left = kronecker(&identity, a_transpose);
    let right = kronecker(a_transpose, &identity);
    add_real_matrices(&left, &right)
}

fn build_discrete_lyapunov_system(a_transpose: &[Vec<f64>]) -> Vec<Vec<Complex64>> {
    let state_count = a_transpose.len();
    let left = kronecker(a_transpose, a_transpose);
    let identity = identity(state_count * state_count);
    subtract_real_matrices(&left, &identity)
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

fn subtract_real_matrices(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<Complex64>> {
    left.iter()
        .zip(right)
        .map(|(left_row, right_row)| {
            left_row
                .iter()
                .zip(right_row)
                .map(|(left_value, right_value)| Complex64::new(left_value - right_value, 0.0))
                .collect()
        })
        .collect()
}

fn vectorize_real_matrix(matrix: &[Vec<f64>]) -> Vec<f64> {
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

fn unvectorize_real_matrix(values: &[f64], order: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; order]; order];
    for column_index in 0..order {
        for row_index in 0..order {
            matrix[row_index][column_index] = values[column_index * order + row_index];
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::sb03md_solve;

    #[test]
    fn solves_scalar_continuous_case() {
        let a = vec![vec![-2.0]];
        let c = vec![vec![8.0]];
        let result = sb03md_solve('C', 'X', 'N', 'N', &a, &c).expect("solve should succeed");

        assert_eq!(result.x, vec![vec![-2.0]]);
        assert!((result.scale - 1.0).abs() < 1.0e-12);
    }
}
