//! Pure-Rust implementation of `MB03RD` (block diagonalization of real Schur form).
//!
//! Reorders blocks of a real Schur matrix by eigenvalue; used by bdschur, modal_form.
//! Full implementation requires real Schur decomposition and block reordering;
//! this module is a documented stub until that is available.

use thiserror::Error;

/// Errors returned by the pure-Rust `MB03RD` implementation.
#[derive(Debug, Error)]
pub enum Mb03RdError {
    #[error("MB03RD is not yet implemented; requires real Schur and block reordering")]
    NotImplemented,
}

/// Result: block-diagonal form and transformation (not yet produced).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mb03RdResult {
    pub blocks: Vec<Vec<Vec<f64>>>,
}

/// Block diagonalizes real Schur form by reordering blocks.
///
/// # Errors
///
/// Currently always returns [`Mb03RdError::NotImplemented`].
pub fn mb03rd_block_diagonalize(
    _schur_a: &[Vec<f64>],
    _select: &[bool],
) -> Result<Mb03RdResult, Mb03RdError> {
    Err(Mb03RdError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::{mb03rd_block_diagonalize, Mb03RdError};

    #[test]
    fn mb03rd_returns_not_implemented() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 2.0]];
        let select = vec![true, true];
        let err = mb03rd_block_diagonalize(&a, &select).unwrap_err();
        assert!(matches!(err, Mb03RdError::NotImplemented));
    }
}
