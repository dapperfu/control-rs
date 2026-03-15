//! Real dense matrix utilities for SLICOT ports.
//!
//! These functions provide a public API for operations that are otherwise
//! duplicated as private helpers across routine modules.

/// Transposes a real matrix (rows × columns → columns × rows).
///
/// # Examples
///
/// ```
/// use slicot_linalg::transpose_real;
///
/// let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let at = transpose_real(&a);
/// assert_eq!(at, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
/// ```
#[must_use]
pub fn transpose_real(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
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

/// Returns the Frobenius norm of a real matrix (sqrt of sum of squares of entries).
///
/// # Examples
///
/// ```
/// use slicot_linalg::frobenius_norm_real;
///
/// let a = vec![vec![3.0, 4.0]];
/// assert!((frobenius_norm_real(&a) - 5.0).abs() < 1e-10);
/// ```
#[must_use]
pub fn frobenius_norm_real(matrix: &[Vec<f64>]) -> f64 {
    matrix
        .iter()
        .flat_map(|row| row.iter())
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt()
}

/// Returns the infinity norm of a real matrix (maximum absolute row sum).
///
/// # Examples
///
/// ```
/// use slicot_linalg::matrix_infinity_norm_real;
///
/// let a = vec![vec![1.0, -2.0], vec![3.0, 4.0]];
/// assert!((matrix_infinity_norm_real(&a) - 7.0).abs() < 1e-10);
/// ```
#[must_use]
pub fn matrix_infinity_norm_real(matrix: &[Vec<f64>]) -> f64 {
    matrix
        .iter()
        .map(|row| row.iter().map(|x| x.abs()).sum::<f64>())
        .fold(0.0_f64, f64::max)
}

/// Scales a real matrix by a scalar (in place conceptually; returns a new matrix).
///
/// # Examples
///
/// ```
/// use slicot_linalg::scale_real_matrix;
///
/// let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let b = scale_real_matrix(&a, 2.0);
/// assert_eq!(b, vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
/// ```
#[must_use]
pub fn scale_real_matrix(matrix: &[Vec<f64>], alpha: f64) -> Vec<Vec<f64>> {
    matrix
        .iter()
        .map(|row| row.iter().map(|x| alpha * x).collect())
        .collect()
}

/// Adds two real matrices of the same shape.
///
/// If the matrices have different dimensions, only the overlapping part is used (rows/columns beyond the smaller dimension are ignored).
///
/// # Examples
///
/// ```
/// use slicot_linalg::add_real_matrices;
///
/// let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
/// let b = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
/// let c = add_real_matrices(&a, &b);
/// assert_eq!(c, vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
/// ```
#[must_use]
pub fn add_real_matrices(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<f64>> {
    left.iter()
        .zip(right)
        .map(|(left_row, right_row)| {
            left_row
                .iter()
                .zip(right_row)
                .map(|(l, r)| l + r)
                .collect()
        })
        .collect()
}

/// Subtracts the second real matrix from the first (same shape).
///
/// # Examples
///
/// ```
/// use slicot_linalg::subtract_real_matrices;
///
/// let a = vec![vec![3.0, 3.0], vec![3.0, 3.0]];
/// let b = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
/// let c = subtract_real_matrices(&a, &b);
/// assert_eq!(c, vec![vec![2.0, 2.0], vec![2.0, 2.0]]);
/// ```
#[must_use]
pub fn subtract_real_matrices(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<f64>> {
    left.iter()
        .zip(right)
        .map(|(left_row, right_row)| {
            left_row
                .iter()
                .zip(right_row)
                .map(|(l, r)| l - r)
                .collect()
        })
        .collect()
}

/// Multiplies two real matrices (left * right).
///
/// # Examples
///
/// ```
/// use slicot_linalg::multiply_real_matrices;
///
/// let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let b = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
/// let c = multiply_real_matrices(&a, &b);
/// assert_eq!(c, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
/// ```
#[must_use]
pub fn multiply_real_matrices(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let row_count = left.len();
    let column_count = right.first().map_or(0, Vec::len);
    let inner = right.len();
    let mut out = vec![vec![0.0; column_count]; row_count];
    for (i, left_row) in left.iter().enumerate() {
        for j in 0..column_count {
            let mut s = 0.0;
            for k in 0..inner {
                s += left_row[k] * right[k][j];
            }
            out[i][j] = s;
        }
    }
    out
}

/// Builds an n×n real identity matrix.
///
/// # Examples
///
/// ```
/// use slicot_linalg::identity_real_matrix;
///
/// let i = identity_real_matrix(2);
/// assert_eq!(i, vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
/// ```
#[must_use]
pub fn identity_real_matrix(n: usize) -> Vec<Vec<f64>> {
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| if i == j { 1.0 } else { 0.0 })
                .collect()
        })
        .collect()
}

/// Builds an m×n real matrix of zeros.
///
/// # Examples
///
/// ```
/// use slicot_linalg::zero_real_matrix;
///
/// let z = zero_real_matrix(2, 3);
/// assert_eq!(z, vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]]);
/// ```
#[must_use]
pub fn zero_real_matrix(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    vec![vec![0.0; cols]; rows]
}

/// Returns the trace of a square real matrix (sum of diagonal elements).
///
/// # Examples
///
/// ```
/// use slicot_linalg::trace_real;
///
/// let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// assert!((trace_real(&a) - 5.0).abs() < 1e-10);
/// ```
#[must_use]
pub fn trace_real(matrix: &[Vec<f64>]) -> f64 {
    matrix
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.get(i).copied())
        .sum()
}

/// Extracts the diagonal of a square real matrix as a vector.
///
/// # Examples
///
/// ```
/// use slicot_linalg::diagonal_real;
///
/// let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// assert_eq!(diagonal_real(&a), vec![1.0, 4.0]);
/// ```
#[must_use]
pub fn diagonal_real(matrix: &[Vec<f64>]) -> Vec<f64> {
    matrix
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.get(i).copied())
        .collect()
}

/// Returns the maximum absolute value of any element in the matrix.
///
/// # Examples
///
/// ```
/// use slicot_linalg::matrix_max_abs_real;
///
/// let a = vec![vec![1.0, -3.0], vec![2.0, 0.5]];
/// assert!((matrix_max_abs_real(&a) - 3.0).abs() < 1e-10);
/// ```
#[must_use]
pub fn matrix_max_abs_real(matrix: &[Vec<f64>]) -> f64 {
    matrix
        .iter()
        .flat_map(|row| row.iter())
        .map(|x| x.abs())
        .fold(0.0_f64, f64::max)
}

/// Computes the outer product of two real vectors: u * v^T (column * row).
///
/// Returns a matrix of shape (u.len() × v.len()).
///
/// # Examples
///
/// ```
/// use slicot_linalg::outer_product_real;
///
/// let u = vec![1.0, 2.0];
/// let v = vec![3.0, 4.0];
/// let m = outer_product_real(&u, &v);
/// assert_eq!(m, vec![vec![3.0, 4.0], vec![6.0, 8.0]]);
/// ```
#[must_use]
pub fn outer_product_real(u: &[f64], v: &[f64]) -> Vec<Vec<f64>> {
    u.iter()
        .map(|ui| v.iter().map(|vj| ui * vj).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        add_real_matrices, diagonal_real, frobenius_norm_real, identity_real_matrix,
        matrix_infinity_norm_real, matrix_max_abs_real, multiply_real_matrices, outer_product_real,
        scale_real_matrix, subtract_real_matrices, trace_real, transpose_real, zero_real_matrix,
    };

    #[test]
    fn transpose_real_rectangular() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let at = transpose_real(&a);
        assert_eq!(at.len(), 3);
        assert_eq!(at[0], vec![1.0, 4.0]);
        assert_eq!(at[1], vec![2.0, 5.0]);
        assert_eq!(at[2], vec![3.0, 6.0]);
    }

    #[test]
    fn frobenius_norm_real_unit() {
        let i = identity_real_matrix(3);
        assert!((frobenius_norm_real(&i) - 3.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn matrix_infinity_norm_real_single_row() {
        let a = vec![vec![1.0, -2.0, 3.0]];
        assert!((matrix_infinity_norm_real(&a) - 6.0).abs() < 1e-10);
    }

    #[test]
    fn scale_real_matrix_zero() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let z = scale_real_matrix(&a, 0.0);
        assert_eq!(z, vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
    }

    #[test]
    fn add_real_matrices_identity_plus_identity() {
        let i = identity_real_matrix(2);
        let two_i = add_real_matrices(&i, &i);
        assert_eq!(two_i, vec![vec![2.0, 0.0], vec![0.0, 2.0]]);
    }

    #[test]
    fn subtract_real_matrices_identity() {
        let a = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let i = identity_real_matrix(2);
        let diff = subtract_real_matrices(&a, &i);
        assert_eq!(diff, vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    }

    #[test]
    fn multiply_real_matrices_square() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        let c = multiply_real_matrices(&a, &b);
        assert_eq!(c, vec![vec![2.0, 1.0], vec![4.0, 3.0]]);
    }

    #[test]
    fn identity_real_matrix_1x1() {
        let i = identity_real_matrix(1);
        assert_eq!(i, vec![vec![1.0]]);
    }

    #[test]
    fn zero_real_matrix_shape() {
        let z = zero_real_matrix(2, 3);
        assert_eq!(z.len(), 2);
        assert_eq!(z[0].len(), 3);
        assert!(z.iter().flat_map(|r| r.iter()).all(|&x| x == 0.0));
    }

    #[test]
    fn trace_real_1x1() {
        let a = vec![vec![7.0]];
        assert!((trace_real(&a) - 7.0).abs() < 1e-10);
    }

    #[test]
    fn diagonal_real_2x2() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        assert_eq!(diagonal_real(&a), vec![1.0, 4.0]);
    }

    #[test]
    fn matrix_max_abs_real_negative() {
        let a = vec![vec![1.0, -5.0], vec![2.0, 3.0]];
        assert!((matrix_max_abs_real(&a) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn outer_product_real_2x2() {
        let u = vec![1.0, 2.0];
        let v = vec![3.0, 4.0];
        let m = outer_product_real(&u, &v);
        assert_eq!(m, vec![vec![3.0, 4.0], vec![6.0, 8.0]]);
    }
}
