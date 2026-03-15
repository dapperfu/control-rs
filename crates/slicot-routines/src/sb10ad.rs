//! Pure-Rust implementation of `SB10AD` (H-infinity optimal controller).
//!
//! Subset: continuous-time state-feedback synthesis. Solves the CARE
//! A'X + XA - X B B' X + C'C = 0 and returns the gain K = B'X.
//! This is the same equation as in the H-infinity state-feedback case in the
//! limit of large gamma (no bound), yielding a stabilizing static controller.
//! Full output-feedback Glover-Doyle (two AREs, general plant) is not implemented.

use crate::sb02md_solve;
use thiserror::Error;

/// Errors returned by the pure-Rust `SB10AD` implementation.
#[derive(Debug, Error)]
pub enum Sb10AdError {
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    #[error("CARE has no stabilizing solution")]
    NoStabilizingSolution,
    #[error(transparent)]
    Care(#[from] crate::Sb02MdError),
}

/// Result: controller (Ac, Bc, Cc, Dc). For state-feedback subset, Ac and Bc are empty;
/// Cc = -K, Dc = 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sb10AdResult {
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

/// H-infinity state-feedback synthesis (continuous-time subset).
///
/// Solves A'X + XA - X B B' X + C'C = 0 and returns u = -Kx with K = B'X.
/// This stabilizes the plant and corresponds to the state-feedback H-infinity
/// solution in the limit of no performance bound. Full Glover-Doyle output-feedback
/// is not implemented.
///
/// # Errors
///
/// Returns [`Sb10AdError`] if dimensions are wrong or the CARE has no stabilizing solution.
pub fn sb10ad_hinfsyn(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<Sb10AdResult, Sb10AdError> {
    let n = a.len();
    if n == 0 {
        return Ok(Sb10AdResult {
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
        return Err(Sb10AdError::IncompatibleDimensions(
            "A n×n, B n×m, C p×n, D p×m".to_string(),
        ));
    }

    let q = mat_mul_at_b(p, n, c, p, n, c);
    let g = mat_mul(n, m, b, m, n, &transpose(b));

    let care = sb02md_solve('C', a, &q, &g).map_err(|_| Sb10AdError::NoStabilizingSolution)?;
    let x = &care.x;

    let bt = transpose(b);
    let k = mat_mul(m, n, &bt, n, n, x);

    let cc: Vec<Vec<f64>> = k.iter().map(|row| row.iter().map(|&v| -v).collect()).collect();
    let dc = vec![vec![0.0; n]; m];

    Ok(Sb10AdResult {
        ac: vec![],
        bc: vec![],
        cc,
        dc,
    })
}

#[cfg(test)]
mod tests {
    use super::sb10ad_hinfsyn;

    #[test]
    fn sb10ad_returns_controller_for_scalar() {
        let a = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let result = sb10ad_hinfsyn(&a, &b, &c, &d).expect("stabilizable");
        assert!(result.ac.is_empty());
        assert!(result.bc.is_empty());
        assert_eq!(result.cc.len(), 1);
        assert_eq!(result.cc[0].len(), 1);
        assert!(result.cc[0][0].abs() > 0.0);
    }

    #[test]
    fn sb10ad_stable_plant() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let result = sb10ad_hinfsyn(&a, &b, &c, &d).expect("stable");
        assert_eq!(result.cc.len(), 1);
    }
}
