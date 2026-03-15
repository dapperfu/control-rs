//! Pure-Rust implementation of the `AB13BD` L2/H2 norm subset for continuous- and
//! discrete-time state-space systems.
//!
//! For stable A: continuous uses observability Gramian A' P + P A = -C' C;
//! discrete uses A' P A - P = -C' C. Then norm^2 = trace(B' P B) + trace(D' D).

use crate::sb03md_solve;
use thiserror::Error;

/// Errors returned by the pure-Rust `AB13BD` implementation.
#[derive(Debug, Error)]
pub enum Ab13BdError {
    /// Matrix dimension mismatch.
    #[error("incompatible dimensions: {0}")]
    IncompatibleDimensions(String),
    /// The system matrix A has eigenvalues in the closed right half-plane;
    /// the observability Gramian does not exist.
    #[error("system is not stable: {0}")]
    NotStable(String),
    /// Lyapunov solve failed (e.g. singular).
    #[error(transparent)]
    Lyapunov(#[from] crate::Sb03MdError),
}

/// Computes the H2 (continuous-time) or L2 norm of the transfer function
/// G(s) = C(sI - A)^{-1} B + D or G(z) = C(zI - A)^{-1} B + D.
///
/// For continuous-time (dico = 'C'): solves A' P + P A = -C' C for the
/// observability Gramian P. For discrete-time (dico = 'D'): solves
/// A' P A - P = -C' C. Then norm^2 = trace(B' P B) + trace(D' D).
/// Requires A to be stable (continuous: open left half-plane; discrete: inside unit circle).
///
/// # Errors
///
/// Returns [`Ab13BdError`] if dimensions are wrong or A is not stable.
pub fn ab13bd_norm(
    dico: char,
    a: &[Vec<f64>],
    b: &[Vec<f64>],
    c: &[Vec<f64>],
    d: &[Vec<f64>],
) -> Result<f64, Ab13BdError> {
    let n = a.len();
    if n == 0 {
        return Ok(0.0);
    }
    if a.iter().any(|row| row.len() != n) {
        return Err(Ab13BdError::IncompatibleDimensions(
            "A must be square".to_string(),
        ));
    }
    let m = b.first().map_or(0, Vec::len);
    let p = c.len();
    if b.len() != n || c.iter().any(|row| row.len() != n) || d.len() != p
        || d.iter().any(|row| row.len() != m)
    {
        return Err(Ab13BdError::IncompatibleDimensions(
            "B, C, D dimensions must match n, m, p".to_string(),
        ));
    }

    if dico != 'C' && dico != 'D' {
        return Err(Ab13BdError::IncompatibleDimensions(
            "dico must be 'C' (continuous) or 'D' (discrete)".to_string(),
        ));
    }

    // Observability Gramian: continuous A' P + P A = -C' C; discrete A' P A - P = -C' C
    let ctc = matmul_at_b(p, n, c, p, n, c);
    let neg_ctc = ctc.iter().map(|row| row.iter().map(|&x| -x).collect()).collect::<Vec<_>>();
    let lyap_result = sb03md_solve(dico, 'X', 'N', 'N', a, &neg_ctc)
        .map_err(Ab13BdError::Lyapunov)?;
    let gramian_p = lyap_result.x;

    // norm^2 = trace(B' P B) + trace(D' D)
    let pb = matmul(n, n, &gramian_p, n, m, b);
    let bt_pb = matmul_at_b(n, m, b, n, m, &pb);
    let trace_bpb = (0..m).map(|i| bt_pb[i][i]).sum::<f64>();
    let trace_dtd = (0..p)
        .map(|i| (0..m).map(|j| d[i][j] * d[i][j]).sum::<f64>())
        .sum::<f64>();
    let norm_sq = trace_bpb + trace_dtd;
    if norm_sq < 0.0 {
        return Err(Ab13BdError::NotStable(
            "norm^2 < 0 (numerical or unstable A)".to_string(),
        ));
    }
    Ok(norm_sq.sqrt())
}

/// C := A' * B (A is rows_a×cols_a, B is rows_a×cols_b, result is cols_a×cols_b)
fn matmul_at_b(
    rows_a: usize,
    cols_a: usize,
    a: &[Vec<f64>],
    _rows_b: usize,
    cols_b: usize,
    b: &[Vec<f64>],
) -> Vec<Vec<f64>> {
    let mut out = vec![vec![0.0; cols_b]; cols_a];
    for i in 0..cols_a {
        for j in 0..cols_b {
            let mut s = 0.0;
            for k in 0..rows_a {
                s += a[k][i] * b[k][j];
            }
            out[i][j] = s;
        }
    }
    out
}

/// C := A * B (A is rows_a×cols_a, B is cols_a×cols_b)
fn matmul(
    rows_a: usize,
    cols_a: usize,
    a: &[Vec<f64>],
    _rows_b: usize,
    cols_b: usize,
    b: &[Vec<f64>],
) -> Vec<Vec<f64>> {
    let mut out = vec![vec![0.0; cols_b]; rows_a];
    for i in 0..rows_a {
        for j in 0..cols_b {
            let mut s = 0.0;
            for k in 0..cols_a {
                s += a[i][k] * b[k][j];
            }
            out[i][j] = s;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::ab13bd_norm;

    #[test]
    fn h2_norm_stable_scalar_system() {
        // G(s) = 1/(s+1): A=-1, B=1, C=1, D=0. H2 norm = 1/sqrt(2)
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let norm = ab13bd_norm('C', &a, &b, &c, &d).expect("stable system");
        let expected = 1.0 / 2.0_f64.sqrt();
        assert!((norm - expected).abs() < 1.0e-10);
    }

    #[test]
    fn l2_norm_stable_discrete_scalar_system() {
        // G(z) = 1/(z - 0.5): A=0.5, B=1, C=1, D=0. A' P A - P = -1 => 0.25 P - P = -1 => P = 4/3, norm^2 = 4/3
        let a = vec![vec![0.5]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let d = vec![vec![0.0]];
        let norm = ab13bd_norm('D', &a, &b, &c, &d).expect("stable discrete system");
        let expected = (4.0 / 3.0_f64).sqrt();
        assert!((norm - expected).abs() < 1.0e-10);
    }
}
