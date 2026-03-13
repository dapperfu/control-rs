#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Pure-Rust linear algebra building blocks for SLICOT ports.

mod complex;

pub use complex::{
    matrix_one_norm, multiply_real_by_complex, solve_complex_system, ComplexMatrixError,
};
pub use faer::{Mat, MatRef};
pub use num_complex::Complex64;

/// Floating-point scalar type used by the initial SLICOT ports.
pub type Real = f64;

/// Returns the default scalar tolerance used by the initial ports.
///
/// # Examples
///
/// ```
/// use slicot_linalg::default_tolerance;
///
/// assert!(default_tolerance().is_sign_positive());
/// ```
#[must_use]
pub fn default_tolerance() -> Real {
    f64::EPSILON.sqrt()
}

#[cfg(test)]
mod tests {
    use super::{default_tolerance, solve_complex_system, Complex64};

    #[test]
    fn default_tolerance_is_positive() {
        assert!(default_tolerance() > 0.0);
    }

    #[test]
    fn solves_small_complex_system() {
        let matrix = vec![
            vec![Complex64::new(2.0, 0.0), Complex64::new(1.0, 0.0)],
            vec![Complex64::new(1.0, -1.0), Complex64::new(3.0, 0.0)],
        ];
        let rhs = vec![
            vec![Complex64::new(1.0, 0.0)],
            vec![Complex64::new(2.0, 1.0)],
        ];

        let solution = solve_complex_system(&matrix, &rhs).expect("system should be solvable");
        let first = solution[0][0];
        let second = solution[1][0];

        assert!((first - Complex64::new(0.153_846_153_846, -0.230_769_230_769)).norm() < 1.0e-10);
        assert!((second - Complex64::new(0.692_307_692_308, 0.461_538_461_538)).norm() < 1.0e-10);
    }
}
