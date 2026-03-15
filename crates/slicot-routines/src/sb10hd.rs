//! Pure-Rust implementation of `SB10HD` (H2 optimal control synthesis).
//!
//! Subset: continuous-time H2 state-feedback synthesis. Solves the CARE
//! A'X + XA - X B B' X + C'C = 0 and returns the optimal gain K = B'X.
//! The controller is static: u = -Kx (Ac, Bc empty; Cc = -K, Dc = 0).

use crate::sb02md_solve;
use thiserror::Error;

/// Errors returned by the pure-Rust `SB10HD` implementation.
#[derive(Debug, Error)]
pub enum Sb10HdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("CARE has no stabilizing solution (system not stabilizable or Q not suitable)")]
    NoStabilizingSolution,
    #[error(transparent)]
    Care(#[from] crate::Sb02MdError),
}

/// Result: controller (Ac, Bc, Cc, Dc). For state-feedback subset, Ac and Bc are empty;
/// Cc = -K (gain), Dc = 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sb10HdResult {
    pub ac: Vec<Vec<f64>>,
    pub bc: Vec<Vec<f64>>,
    pub cc: Vec<Vec<f64>>,
    pub dc: Vec<Vec<f64>>,
}

fn mat_mul_at_b(
    ar: usize,
    ac: usize,
    a: &[Vec<f64>],
    _br: usize,
    bc: usize,
    b: &[Vec<f64>],
) -> Vec<Vec<f64>> {
    let mut out = vec![vec![0.0; bc]; ac];
    for i in 0..ac {
        for j in 0..bc {
            let mut s = 0.0;
            for k in 0..ar {
                s += a[k][i] * b[k][j];
            }
            out[i][j] = s;
        }
    }
    out
}

fn mat_mul(ar: usize, ac: usize, a: &[Vec<f64>], _br: usize, bc: usize, b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut out = vec![vec![0.0; bc]; ar];
    for i in 0..ar {
        for j in 0..bc {
            let mut s = 0.0;
            for k in 0..ac {
                s += a[i][k] * b[k][j];
            }
            out[i][j] = s;
        }
    }
    out
}

/// H2 optimal state-feedback synthesis (continuous-time subset).
///
/// Solves CARE A'X + XA - X B B' X + C'C = 0 and sets K = B'X. The controller
/// is u = -Kx (static). Returns Ac = [], Bc = [], Cc = -K, Dc = 0.
///
/// Only continuous-time and the standard (A, B, C, D) form are supported.
///
/// # Errors
///
/// Returns [`Sb10HdError`] if dimensions are wrong or the CARE has no stabilizing solution.
pub fn sb10hd_h2syn(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<Sb10HdResult, Sb10HdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Sb10HdResult {
            ac: vec![],
            bc: vec![],
            cc: vec![],
            dc: d.to_vec(),
        });
    }
    let m = b.first().map_or(0, Vec::len);
    let p = c.len();
    if a.iter().any(|row| row.len() != n)
        || b.len() != n
        || c.iter().any(|row| row.len() != n)
        || d.len() != p
        || d.iter().any(|row| row.len() != m)
    {
        return Err(Sb10HdError::IncompatibleDimensions(
            "A n×n, B n×m, C p×n, D p×m".to_string(),
        ));
    }

    let q = mat_mul_at_b(p, n, c, p, n, c);
    let g = mat_mul(n, m, b, m, n, &transpose(b));

    let care = sb02md_solve('C', a, &q, &g).map_err(|_| Sb10HdError::NoStabilizingSolution)?;
    let x = &care.x;

    let bt = transpose(b);
    let k = mat_mul(m, n, &bt, n, n, x);

    let cc: Vec<Vec<f64>> = k.iter().map(|row| row.iter().map(|&v| -v).collect()).collect();
    let dc = vec![vec![0.0; n]; m];

    Ok(Sb10HdResult {
        ac: vec![],
        bc: vec![],
        cc,
        dc,
    })
}

fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let r = m.len();
    let c = m.first().map_or(0, Vec::len);
    let mut t = vec![vec![0.0; r]; c];
    for i in 0..r {
        for j in 0..c {
            t[j][i] = m[i][j];
        }
    }
    t
}

#[cfg(test)]
mod tests {
    use super::sb10hd_h2syn;

    #[test]
    fn sb10hd_stabilizes_scalar() {
        let a = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let result = sb10hd_h2syn(&a, &b, &c, &d).expect("stabilizable");
        assert!(result.ac.is_empty());
        assert!(result.bc.is_empty());
        assert_eq!(result.cc.len(), 1);
        assert_eq!(result.cc[0].len(), 1);
        assert!(result.cc[0][0].abs() > 0.0);
    }

    #[test]
    fn sb10hd_already_stable() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let result = sb10hd_h2syn(&a, &b, &c, &d).expect("stable");
        assert_eq!(result.cc.len(), 1);
    }
}
