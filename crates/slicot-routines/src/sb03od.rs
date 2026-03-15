//! Pure-Rust implementation of the `SB03OD` Cholesky-factor subset.

use thiserror::Error;

use crate::{sb03md_solve, Sb03MdError};

/// Errors returned by the pure-Rust `SB03OD` subset.
#[derive(Debug, Error)]
pub enum Sb03OdError {
    /// The current subset only supports continuous-time equations.
    #[error("DICO='{dico}' is not supported by the current pure-Rust SB03OD subset")]
    UnsupportedDico { dico: char },
    /// The current subset only supports unfactored right-hand sides.
    #[error("FACT='{fact}' is not supported by the current pure-Rust SB03OD subset")]
    UnsupportedFact { fact: char },
    /// The current subset only supports `TRANS='N'`.
    #[error("TRANS='{trans}' is not supported by the current pure-Rust SB03OD subset")]
    UnsupportedTrans { trans: char },
    /// The factor matrix must have at least one row.
    #[error("factor matrix B must not be empty")]
    EmptyFactor,
    /// The factor matrix must have `n` columns.
    #[error("expected factor matrix B with {expected_columns} columns, found {actual_columns}")]
    IncompatibleFactorMatrix {
        expected_columns: usize,
        actual_columns: usize,
    },
    /// The reduced Lyapunov solve failed.
    #[error(transparent)]
    Lyapunov(#[from] Sb03MdError),
    /// The recovered solution is not positive definite.
    #[error("solution matrix is not positive definite")]
    NonPositiveDefinite,
}

/// Output bundle for the pure-Rust `SB03OD` subset.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03OdResult {
    pub u: Vec<Vec<f64>>,
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Solves the supported `SB03OD` subset by forming `X` from `SB03MD` and then
/// taking its Cholesky factor `U` such that `X = U^T U`.
///
/// Supported subset:
/// - `dico = 'C'`
/// - `fact = 'N'`
/// - `trans = 'N'`
///
/// # Errors
///
/// Returns [`Sb03OdError`] if the input mode is unsupported, the factor matrix
/// dimensions are incompatible, the Lyapunov equation cannot be solved, or the
/// resulting solution is not positive definite.
pub fn sb03od_factor(
    dico: char,
    fact: char,
    trans: char,
    a: &[Vec<f64>],
    b: &[Vec<f64>],
) -> Result<Sb03OdResult, Sb03OdError> {
    if !matches!(dico, 'C') {
        return Err(Sb03OdError::UnsupportedDico { dico });
    }
    if !matches!(fact, 'N') {
        return Err(Sb03OdError::UnsupportedFact { fact });
    }
    if !matches!(trans, 'N') {
        return Err(Sb03OdError::UnsupportedTrans { trans });
    }
    if b.is_empty() {
        return Err(Sb03OdError::EmptyFactor);
    }

    let order = a.len();
    if b.iter().any(|row| row.len() != order) {
        return Err(Sb03OdError::IncompatibleFactorMatrix {
            expected_columns: order,
            actual_columns: b.iter().map(Vec::len).max().unwrap_or(0),
        });
    }

    let rhs = negate_matrix(&multiply_transpose_by_self(b));
    let solved = sb03md_solve('C', 'X', 'N', 'N', a, &rhs)?;
    let u = cholesky_upper(&solved.x)?;

    Ok(Sb03OdResult {
        u,
        x: solved.x,
        scale: solved.scale,
    })
}

fn multiply_transpose_by_self(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let column_count = matrix.first().map_or(0, Vec::len);
    let mut product = vec![vec![0.0; column_count]; column_count];

    for (left_column, row) in product.iter_mut().enumerate() {
        for (right_column, value) in row.iter_mut().enumerate() {
            let mut sum = 0.0;
            for matrix_row in matrix {
                sum += matrix_row[left_column] * matrix_row[right_column];
            }
            *value = sum;
        }
    }

    product
}

fn negate_matrix(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    matrix
        .iter()
        .map(|row| row.iter().map(|value| -*value).collect())
        .collect()
}

fn cholesky_upper(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, Sb03OdError> {
    let order = matrix.len();
    let mut lower = vec![vec![0.0; order]; order];

    for row_index in 0..order {
        for column_index in 0..=row_index {
            let mut sum = matrix[row_index][column_index];
            for inner_index in 0..column_index {
                sum -= lower[row_index][inner_index] * lower[column_index][inner_index];
            }

            if row_index == column_index {
                if sum <= 0.0 {
                    return Err(Sb03OdError::NonPositiveDefinite);
                }
                lower[row_index][column_index] = sum.sqrt();
            } else {
                lower[row_index][column_index] = sum / lower[column_index][column_index];
            }
        }
    }

    Ok(transpose(&lower))
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

#[cfg(test)]
mod tests {
    use super::sb03od_factor;

    #[test]
    fn solves_scalar_continuous_factor_case() {
        let a = vec![vec![-2.0]];
        let b = vec![vec![2.0]];
        let result = sb03od_factor('C', 'N', 'N', &a, &b).expect("factorization should succeed");

        assert!((result.x[0][0] - 1.0).abs() < 1.0e-12);
        assert!((result.u[0][0] - 1.0).abs() < 1.0e-12);
    }
}
