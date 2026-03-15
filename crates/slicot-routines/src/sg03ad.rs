//! Pure-Rust implementation of the `SG03AD` generalized Lyapunov solver subset.

use num_complex::Complex64;
use slicot_linalg::{matrix_one_norm, solve_complex_system, ComplexMatrixError};
use thiserror::Error;

/// Errors returned by the pure-Rust `SG03AD` implementation.
#[derive(Debug, Error)]
pub enum Sg03AdError {
    /// The current subset only supports the continuous-time equation.
    #[error("DICO='{dico}' is not supported by the current pure-Rust SG03AD subset")]
    UnsupportedDico { dico: char },
    /// The current subset only supports direct factorization mode.
    #[error("FACT='{fact}' is not supported by the current pure-Rust SG03AD subset")]
    UnsupportedFact { fact: char },
    /// The current subset only supports solution-producing modes.
    #[error("JOB='{job}' is not supported by the current pure-Rust SG03AD subset")]
    UnsupportedJob { job: char },
    /// The left coefficient matrix is not square.
    #[error("expected square matrix A, found {rows}x{columns}")]
    NonSquareLeftMatrix { rows: usize, columns: usize },
    /// The descriptor matrix is not square.
    #[error("expected square matrix E, found {rows}x{columns}")]
    NonSquareDescriptorMatrix { rows: usize, columns: usize },
    /// The right-hand side matrix has incompatible dimensions.
    #[error("expected Y with shape {expected}x{expected}, found {actual_rows}x{actual_columns}")]
    IncompatibleRightHandSide {
        expected: usize,
        actual_rows: usize,
        actual_columns: usize,
    },
    /// The linear solve failed.
    #[error(transparent)]
    LinearSolve(#[from] ComplexMatrixError),
}

/// Output bundle for the pure-Rust `SG03AD` implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03AdResult {
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
    pub sep: Option<f64>,
    pub ferr: Option<f64>,
}

/// Solves the supported subset of the generalized continuous-time Lyapunov
/// equation
/// `op(A)^T X op(E) + op(E)^T X op(A) = scale * Y`.
///
/// Supported subset:
/// - `dico = 'C'`
/// - `fact = 'N'`
/// - `job = 'B'` or `job = 'X'`
/// - `trans = 'N'` or `trans = 'T'`
///
/// # Errors
///
/// Returns [`Sg03AdError`] if the input dimensions are incompatible, the
/// requested mode is unsupported, or the associated Kronecker system is
/// singular.
pub fn sg03ad_solve(
    dico: char,
    job: char,
    fact: char,
    trans: char,
    a: &[Vec<f64>],
    e: &[Vec<f64>],
    y: &[Vec<f64>],
) -> Result<Sg03AdResult, Sg03AdError> {
    if !matches!(dico, 'C') {
        return Err(Sg03AdError::UnsupportedDico { dico });
    }
    if !matches!(fact, 'N') {
        return Err(Sg03AdError::UnsupportedFact { fact });
    }
    if !matches!(job, 'B' | 'X') {
        return Err(Sg03AdError::UnsupportedJob { job });
    }

    let order = a.len();
    if a.iter().any(|row| row.len() != order) {
        return Err(Sg03AdError::NonSquareLeftMatrix {
            rows: order,
            columns: a.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if e.len() != order || e.iter().any(|row| row.len() != order) {
        return Err(Sg03AdError::NonSquareDescriptorMatrix {
            rows: e.len(),
            columns: e.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if y.len() != order || y.iter().any(|row| row.len() != order) {
        return Err(Sg03AdError::IncompatibleRightHandSide {
            expected: order,
            actual_rows: y.len(),
            actual_columns: y.iter().map(Vec::len).max().unwrap_or(0),
        });
    }

    let aop = if matches!(trans, 'N') {
        a.to_vec()
    } else {
        transpose(a)
    };
    let eop = if matches!(trans, 'N') {
        e.to_vec()
    } else {
        transpose(e)
    };
    let aop_transpose = transpose(&aop);
    let eop_transpose = transpose(&eop);
    let kronecker_system = build_generalized_continuous_system(&aop_transpose, &eop_transpose);

    let rhs = vectorize_symmetric_matrix(y)
        .into_iter()
        .map(|value| vec![Complex64::new(value, 0.0)])
        .collect::<Vec<_>>();
    let solution = solve_complex_system(&kronecker_system, &rhs)?;
    let x = unvectorize_square_matrix(
        &solution.into_iter().map(|column| column[0].re).collect::<Vec<_>>(),
        order,
    );

    let (sep, ferr) = if matches!(job, 'B') {
        let sep_value = estimate_sep(&kronecker_system)?;
        let ferr_value = estimate_ferr(&aop_transpose, &eop_transpose, &x, y);
        (Some(sep_value), Some(ferr_value))
    } else {
        (None, None)
    };

    Ok(Sg03AdResult {
        x,
        scale: 1.0,
        sep,
        ferr,
    })
}

fn build_generalized_continuous_system(
    a_transpose: &[Vec<f64>],
    e_transpose: &[Vec<f64>],
) -> Vec<Vec<Complex64>> {
    let left = kronecker(e_transpose, a_transpose);
    let right = kronecker(a_transpose, e_transpose);
    add_real_matrices(&left, &right)
}

fn estimate_sep(kronecker_system: &[Vec<Complex64>]) -> Result<f64, Sg03AdError> {
    let order = kronecker_system.len();
    let identity_matrix = (0..order)
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
    let inverse = solve_complex_system(kronecker_system, &identity_matrix)?;
    Ok(1.0 / matrix_one_norm(&inverse))
}

fn estimate_ferr(
    a_transpose: &[Vec<f64>],
    e_transpose: &[Vec<f64>],
    x: &[Vec<f64>],
    y: &[Vec<f64>],
) -> f64 {
    let left_term = multiply_three_real_matrices(a_transpose, x, &transpose(e_transpose));
    let right_term = multiply_three_real_matrices(e_transpose, x, &transpose(a_transpose));
    let residual = subtract_real_square_matrices(&add_plain_real_matrices(&left_term, &right_term), y);
    frobenius_norm(&residual) / frobenius_norm(x)
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

fn add_plain_real_matrices(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<f64>> {
    left.iter()
        .zip(right)
        .map(|(left_row, right_row)| {
            left_row
                .iter()
                .zip(right_row)
                .map(|(left_value, right_value)| left_value + right_value)
                .collect()
        })
        .collect()
}

fn subtract_real_square_matrices(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<f64>> {
    left.iter()
        .zip(right)
        .map(|(left_row, right_row)| {
            left_row
                .iter()
                .zip(right_row)
                .map(|(left_value, right_value)| left_value - right_value)
                .collect()
        })
        .collect()
}

fn multiply_three_real_matrices(
    left: &[Vec<f64>],
    middle: &[Vec<f64>],
    right: &[Vec<f64>],
) -> Vec<Vec<f64>> {
    let first = multiply_real_matrices(left, middle);
    multiply_real_matrices(&first, right)
}

fn multiply_real_matrices(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let row_count = left.len();
    let column_count = right.first().map_or(0, Vec::len);
    let inner_dimension = right.len();
    let mut output = vec![vec![0.0; column_count]; row_count];

    for (row_index, left_row) in left.iter().enumerate() {
        for column_index in 0..column_count {
            let mut sum = 0.0;
            for inner_index in 0..inner_dimension {
                sum += left_row[inner_index] * right[inner_index][column_index];
            }
            output[row_index][column_index] = sum;
        }
    }

    output
}

fn vectorize_symmetric_matrix(matrix: &[Vec<f64>]) -> Vec<f64> {
    let order = matrix.len();
    let mut values = Vec::with_capacity(order * order);
    for column_index in 0..order {
        for row in matrix.iter().take(order) {
            values.push(row[column_index]);
        }
    }
    values
}

fn unvectorize_square_matrix(values: &[f64], order: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; order]; order];
    for column_index in 0..order {
        for row_index in 0..order {
            matrix[row_index][column_index] = values[column_index * order + row_index];
        }
    }
    matrix
}

fn frobenius_norm(matrix: &[Vec<f64>]) -> f64 {
    matrix
        .iter()
        .flat_map(|row| row.iter())
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::sg03ad_solve;

    #[test]
    fn solves_scalar_generalized_continuous_case() {
        let a = vec![vec![2.0]];
        let e = vec![vec![3.0]];
        let y = vec![vec![12.0]];
        let result = sg03ad_solve('C', 'B', 'N', 'N', &a, &e, &y).expect("solve should succeed");

        assert!((result.x[0][0] - 1.0).abs() < 1.0e-12);
        assert!(result.sep.is_some());
        assert!(result.ferr.is_some());
    }
}
