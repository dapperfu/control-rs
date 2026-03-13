#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! Pure-Rust linear algebra building blocks for SLICOT ports.

pub use faer::{Mat, MatRef};

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
    use super::default_tolerance;

    #[test]
    fn default_tolerance_is_positive() {
        assert!(default_tolerance() > 0.0);
    }
}
