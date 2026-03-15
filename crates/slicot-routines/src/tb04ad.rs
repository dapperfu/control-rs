//! Pure-Rust transfer-matrix subset of `TB04AD`.

use num_complex::Complex64;
use slicot_linalg::solve_complex_system;
use thiserror::Error;

use crate::Tb05AdError;

/// One transfer-matrix element represented as a rational polynomial quotient.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb04AdTransferPolynomial {
    pub row: usize,
    pub column: usize,
    pub numerator: Vec<f64>,
    pub denominator: Vec<f64>,
}

/// Output bundle for the pure-Rust `TB04AD` subset.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb04AdResult {
    pub order: usize,
    pub transfer_polynomials: Vec<Tb04AdTransferPolynomial>,
}

/// Errors returned by the pure-Rust `TB04AD` transfer-matrix implementation.
#[derive(Debug, Error)]
pub enum Tb04AdError {
    /// The state matrix must be square.
    #[error("expected square state matrix A, found {rows}x{columns}")]
    NonSquareStateMatrix { rows: usize, columns: usize },
    /// The input matrix dimensions are incompatible with `A`.
    #[error("expected B with {expected_rows} rows, found {actual_rows}")]
    IncompatibleInputMatrix {
        expected_rows: usize,
        actual_rows: usize,
    },
    /// The output matrix dimensions are incompatible with `A`.
    #[error("expected C with {expected_columns} columns, found {actual_columns}")]
    IncompatibleOutputMatrix {
        expected_columns: usize,
        actual_columns: usize,
    },
    /// The feedthrough matrix dimensions are incompatible with `B` and `C`.
    #[error("expected D with shape {expected_rows}x{expected_columns}, found {actual_rows}x{actual_columns}")]
    IncompatibleFeedthroughMatrix {
        expected_rows: usize,
        expected_columns: usize,
        actual_rows: usize,
        actual_columns: usize,
    },
    /// The interpolation solve failed.
    #[error(transparent)]
    Interpolation(#[from] Tb05AdError),
}

/// Computes the transfer-matrix polynomial quotient
/// `G(s) = C (sI - A)^-1 B + D`.
///
/// This subset returns one rational quotient per transfer-matrix element using
/// a shared characteristic denominator of `A`. It does not currently expose the
/// controllability-reduced realization reported by the full SLICOT `TB04AD`.
///
/// # Errors
///
/// Returns [`Tb04AdError`] if the input dimensions are incompatible or the
/// interpolation solves fail.
pub fn tb04ad_transfer_matrix(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<Tb04AdResult, Tb04AdError> {
    validate_dimensions(a, b, c, d)?;

    let order = a.len();
    let input_count = b.first().map_or(0, Vec::len);
    let output_count = c.len();
    let denominator = characteristic_polynomial(a);
    let sample_points = interpolation_points(&denominator, order + 1);
    let realization = RealizationRef { a, b, c, d };

    let mut transfer_polynomials = Vec::with_capacity(output_count * input_count);
    for output_index in 0..output_count {
        for input_index in 0..input_count {
            let numerator = interpolate_numerator(
                &realization,
                &denominator,
                &sample_points,
                output_index,
                input_index,
            )?;
            transfer_polynomials.push(Tb04AdTransferPolynomial {
                row: output_index + 1,
                column: input_index + 1,
                numerator,
                denominator: denominator.clone(),
            });
        }
    }

    Ok(Tb04AdResult {
        order,
        transfer_polynomials,
    })
}

fn validate_dimensions(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<(), Tb04AdError> {
    let order = a.len();
    if a.iter().any(|row| row.len() != order) {
        return Err(Tb04AdError::NonSquareStateMatrix {
            rows: order,
            columns: a.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    if b.len() != order {
        return Err(Tb04AdError::IncompatibleInputMatrix {
            expected_rows: order,
            actual_rows: b.len(),
        });
    }
    if c.iter().any(|row| row.len() != order) {
        return Err(Tb04AdError::IncompatibleOutputMatrix {
            expected_columns: order,
            actual_columns: c.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    let expected_rows = c.len();
    let expected_columns = b.first().map_or(0, Vec::len);
    if d.len() != expected_rows || d.iter().any(|row| row.len() != expected_columns) {
        return Err(Tb04AdError::IncompatibleFeedthroughMatrix {
            expected_rows,
            expected_columns,
            actual_rows: d.len(),
            actual_columns: d.iter().map(Vec::len).max().unwrap_or(0),
        });
    }
    Ok(())
}

fn characteristic_polynomial(matrix: &[Vec<f64>]) -> Vec<f64> {
    let order = matrix.len();
    let identity = identity_matrix(order);
    let mut previous = identity.clone();
    let mut coefficients = vec![1.0];

    for step in 1..=order {
        let product = multiply_real_matrices(matrix, &previous);
        let trace = (0..order).map(|index| product[index][index]).sum::<f64>();
        let step_as_u32 = u32::try_from(step).expect("matrix order should fit in u32");
        let coefficient = -trace / f64::from(step_as_u32);
        coefficients.push(coefficient);
        previous = add_scaled_identity(&product, coefficient);
    }

    coefficients
}

fn interpolation_points(denominator: &[f64], count: usize) -> Vec<f64> {
    let mut points = Vec::with_capacity(count);
    let mut candidate = 0.0_f64;
    while points.len() < count {
        if evaluate_polynomial(denominator, candidate).abs() > 1.0e-6 {
            points.push(candidate);
        }
        candidate += 1.0;
    }
    points
}

fn interpolate_numerator(
    realization: &RealizationRef<'_>,
    denominator: &[f64],
    sample_points: &[f64],
    output_index: usize,
    input_index: usize,
) -> Result<Vec<f64>, Tb04AdError> {
    let vandermonde = sample_points
        .iter()
        .map(|point| {
            (0..sample_points.len())
                .map(|power_index| {
                    let exponent = sample_points.len() - power_index - 1;
                    Complex64::new(real_power(*point, exponent), 0.0)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rhs = sample_points
        .iter()
        .map(|point| {
            let transfer =
                evaluate_transfer_entry(realization, *point, output_index, input_index)?;
            let numerator_value = transfer * evaluate_polynomial(denominator, *point);
            Ok(vec![Complex64::new(numerator_value, 0.0)])
        })
        .collect::<Result<Vec<_>, Tb04AdError>>()?;

    let coefficients = solve_complex_system(&vandermonde, &rhs)
        .map_err(|source| Tb04AdError::Interpolation(Tb05AdError::LinearSolve(source)))?;
    Ok(coefficients.into_iter().map(|value| value[0].re).collect())
}

struct RealizationRef<'matrix> {
    a: &'matrix [Vec<f64>],
    b: &'matrix [Vec<f64>],
    c: &'matrix [Vec<f64>],
    d: &'matrix [Vec<f64>],
}

fn evaluate_transfer_entry(
    realization: &RealizationRef<'_>,
    point: f64,
    output_index: usize,
    input_index: usize,
) -> Result<f64, Tb04AdError> {
    let order = realization.a.len();
    let mut system = vec![vec![Complex64::new(0.0, 0.0); order]; order];
    for (row_index, row) in system.iter_mut().enumerate() {
        for (column_index, value) in row.iter_mut().enumerate() {
            let diagonal = if row_index == column_index { point } else { 0.0 };
            *value = Complex64::new(diagonal - realization.a[row_index][column_index], 0.0);
        }
    }
    let rhs = (0..order)
        .map(|row_index| vec![Complex64::new(realization.b[row_index][input_index], 0.0)])
        .collect::<Vec<_>>();
    let state = solve_complex_system(&system, &rhs)
        .map_err(|source| Tb04AdError::Interpolation(Tb05AdError::LinearSolve(source)))?;
    let dynamic = realization.c[output_index]
        .iter()
        .zip(state)
        .map(|(output_value, state_value)| output_value * state_value[0].re)
        .sum::<f64>();
    Ok(dynamic + realization.d[output_index][input_index])
}

fn identity_matrix(order: usize) -> Vec<Vec<f64>> {
    (0..order)
        .map(|row_index| {
            (0..order)
                .map(|column_index| if row_index == column_index { 1.0 } else { 0.0 })
                .collect()
        })
        .collect()
}

fn add_scaled_identity(matrix: &[Vec<f64>], scale: f64) -> Vec<Vec<f64>> {
    matrix
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(column_index, value)| {
                    if row_index == column_index {
                        value + scale
                    } else {
                        *value
                    }
                })
                .collect()
        })
        .collect()
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

fn evaluate_polynomial(coefficients: &[f64], point: f64) -> f64 {
    coefficients.iter().fold(0.0, |accumulator, coefficient| accumulator * point + coefficient)
}

fn real_power(base: f64, exponent: usize) -> f64 {
    let exponent_as_i32 = i32::try_from(exponent).expect("polynomial degree should fit in i32");
    base.powi(exponent_as_i32)
}

#[cfg(test)]
mod tests {
    use super::{evaluate_polynomial, tb04ad_transfer_matrix};

    #[test]
    fn evaluates_scalar_transfer_function() {
        let a = vec![vec![-2.0]];
        let b = vec![vec![3.0]];
        let c = vec![vec![4.0]];
        let d = vec![vec![5.0]];

        let result = tb04ad_transfer_matrix(&a, &b, &c, &d).expect("transfer matrix should succeed");
        let polynomial = &result.transfer_polynomials[0];

        let actual = evaluate_polynomial(&polynomial.numerator, 1.0)
            / evaluate_polynomial(&polynomial.denominator, 1.0);
        let expected = 5.0 + (4.0 * 3.0) / 3.0;
        assert!((actual - expected).abs() < 1.0e-10);
    }
}
