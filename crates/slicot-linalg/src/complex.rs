//! Complex dense linear algebra helpers for early SLICOT ports.

use std::cmp::Ordering;

use num_complex::Complex64;
use thiserror::Error;

/// Errors returned by the complex dense linear algebra helpers.
#[derive(Debug, Error)]
pub enum ComplexMatrixError {
    /// The input matrix does not match the expected square shape.
    #[error("expected a square matrix with {expected} rows, found {actual}")]
    NonSquareMatrix { expected: usize, actual: usize },
    /// The right-hand side has a mismatched row count.
    #[error("expected right-hand side with {expected} rows, found {actual}")]
    IncompatibleRightHandSide { expected: usize, actual: usize },
    /// The matrix is singular to working precision.
    #[error("matrix is singular to working precision")]
    SingularMatrix,
}

/// Solves `matrix * x = rhs` for one or more right-hand sides using Gaussian
/// elimination with partial pivoting.
///
/// # Errors
///
/// Returns [`ComplexMatrixError`] if the inputs have incompatible dimensions or
/// if the matrix is singular to working precision.
pub fn solve_complex_system(
    matrix: &[Vec<Complex64>],
    rhs: &[Vec<Complex64>],
) -> Result<Vec<Vec<Complex64>>, ComplexMatrixError> {
    let order = matrix.len();
    if order == 0 {
        return Ok(Vec::new());
    }
    if matrix.iter().any(|row| row.len() != order) {
        return Err(ComplexMatrixError::NonSquareMatrix {
            expected: order,
            actual: matrix.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if rhs.len() != order {
        return Err(ComplexMatrixError::IncompatibleRightHandSide {
            expected: order,
            actual: rhs.len(),
        });
    }
    let column_count = rhs.first().map_or(0, Vec::len);
    if rhs.iter().any(|row| row.len() != column_count) {
        return Err(ComplexMatrixError::IncompatibleRightHandSide {
            expected: column_count,
            actual: rhs.iter().map(Vec::len).max().unwrap_or(0),
        });
    }

    let mut augmented = matrix
        .iter()
        .zip(rhs)
        .map(|(row, rhs_row)| {
            let mut combined = row.clone();
            combined.extend(rhs_row.iter().copied());
            combined
        })
        .collect::<Vec<_>>();

    for pivot_index in 0..order {
        let pivot_row = (pivot_index..order)
            .max_by(|left_index, right_index| {
                augmented[*left_index][pivot_index]
                    .norm()
                    .partial_cmp(&augmented[*right_index][pivot_index].norm())
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap_or(pivot_index);

        if augmented[pivot_row][pivot_index].norm() <= f64::EPSILON {
            return Err(ComplexMatrixError::SingularMatrix);
        }
        if pivot_row != pivot_index {
            augmented.swap(pivot_row, pivot_index);
        }

        let pivot = augmented[pivot_index][pivot_index];
        let pivot_tail = augmented[pivot_index][(pivot_index + 1)..(order + column_count)].to_vec();
        for row in augmented.iter_mut().take(order).skip(pivot_index + 1) {
            let factor = row[pivot_index] / pivot;
            row[pivot_index] = Complex64::new(0.0, 0.0);
            for (offset, pivot_value) in pivot_tail.iter().enumerate() {
                let column_index = pivot_index + 1 + offset;
                row[column_index] -= factor * *pivot_value;
            }
        }
    }

    let mut solution = vec![vec![Complex64::new(0.0, 0.0); column_count]; order];
    for row_index in (0..order).rev() {
        for rhs_index in 0..column_count {
            let mut value = augmented[row_index][order + rhs_index];
            for (column_index, solution_row) in
                solution.iter().enumerate().take(order).skip(row_index + 1)
            {
                value -= augmented[row_index][column_index] * solution_row[rhs_index];
            }
            solution[row_index][rhs_index] = value / augmented[row_index][row_index];
        }
    }

    Ok(solution)
}

/// Returns the matrix one-norm, i.e. the largest absolute column sum.
#[must_use]
pub fn matrix_one_norm(matrix: &[Vec<Complex64>]) -> f64 {
    matrix.first().map_or(0.0, |row| {
        (0..row.len())
            .map(|column_index| {
                matrix
                    .iter()
                    .map(|row_value| row_value[column_index].norm())
                    .sum::<f64>()
            })
            .fold(0.0, f64::max)
    })
}

/// Returns the Frobenius norm of a real matrix, i.e. the square root of the sum
/// of squares of all entries.
#[must_use]
pub fn matrix_frobenius_norm(matrix: &[Vec<f64>]) -> f64 {
    matrix
        .iter()
        .flat_map(|row| row.iter())
        .map(|&x| x * x)
        .sum::<f64>()
        .sqrt()
}

/// Returns the infinity norm of a real matrix, i.e. the largest absolute row sum.
#[must_use]
pub fn matrix_infinity_norm(matrix: &[Vec<f64>]) -> f64 {
    matrix
        .iter()
        .map(|row| row.iter().map(|&x| x.abs()).sum::<f64>())
        .fold(0.0_f64, f64::max)
}

/// Multiplies a real-valued matrix by a complex-valued matrix.
#[must_use]
pub fn multiply_real_by_complex(
    left: &[Vec<f64>],
    right: &[Vec<Complex64>],
) -> Vec<Vec<Complex64>> {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }

    let row_count = left.len();
    let column_count = right[0].len();
    let inner_dimension = right.len();
    let mut output = vec![vec![Complex64::new(0.0, 0.0); column_count]; row_count];

    for (row_index, left_row) in left.iter().enumerate() {
        for column_index in 0..column_count {
            let mut sum = Complex64::new(0.0, 0.0);
            for inner_index in 0..inner_dimension {
                sum += right[inner_index][column_index] * left_row[inner_index];
            }
            output[row_index][column_index] = sum;
        }
    }

    output
}
