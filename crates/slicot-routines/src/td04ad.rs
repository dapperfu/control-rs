//! Pure-Rust implementation of `TD04AD` subset: transfer matrix to minimal state-space (tf2ss).
//!
//! Given T(s) as row or column polynomial vectors over denominators, builds an
//! observable companion realization and reduces it with TB01PD.

use crate::tb01pd_minreal;
use thiserror::Error;

/// Errors returned by the pure-Rust `TD04AD` implementation.
#[derive(Debug, Error)]
pub enum Td04AdError {
    #[error("incompatible dimensions or invalid input: {0}")]
    InvalidInput(String),
    #[error("leading denominator coefficient near zero")]
    SingularDenominator,
    #[error(transparent)]
    Minreal(#[from] crate::Tb01PdError),
}

/// Result: minimal (A, B, C, D) and order.
#[derive(Clone, Debug, PartialEq)]
pub struct Td04AdResult {
    pub order: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
}

/// Builds minimal state-space realization from row-based transfer matrix.
///
/// ROWCOL = 'R': INDEX has P entries (denominator degree per output), DCOEFF is P×kdcoef,
/// UCOEFF is P×M×kdcoef. kdcoef = max(INDEX) + 1.
///
/// # Errors
///
/// Returns [`Td04AdError`] if data is invalid or a leading denominator coefficient is zero.
pub fn td04ad_tf2ss(
    rowcol: char,
    m: usize,
    p: usize,
    index: &[usize],
    dcoeff: &[Vec<f64>],
    ucoeff: &[Vec<Vec<f64>>],
    _tol: f64,
) -> Result<Td04AdResult, Td04AdError> {
    if rowcol != 'R' {
        return Err(Td04AdError::InvalidInput(
            "only ROWCOL='R' (rows) is implemented".to_string(),
        ));
    }
    let porm = p;
    if index.len() != porm || dcoeff.len() != porm {
        return Err(Td04AdError::InvalidInput("INDEX/DCOEFF length".to_string()));
    }
    let kdcoef = index.iter().max().map_or(0, |&d| d + 1);
    if kdcoef == 0 {
        return Err(Td04AdError::InvalidInput("empty index".to_string()));
    }
    for (_i, row) in dcoeff.iter().enumerate() {
        if row.len() < kdcoef {
            return Err(Td04AdError::InvalidInput("DCOEFF row length".to_string()));
        }
        if row[0].abs() < 1.0e-14 {
            return Err(Td04AdError::SingularDenominator);
        }
    }
    if ucoeff.len() != p {
        return Err(Td04AdError::InvalidInput("UCOEFF rows".to_string()));
    }
    for i in 0..p {
        if ucoeff[i].len() != m {
            return Err(Td04AdError::InvalidInput("UCOEFF cols".to_string()));
        }
        for j in 0..m {
            if ucoeff[i][j].len() < kdcoef {
                return Err(Td04AdError::InvalidInput("UCOEFF depth".to_string()));
            }
        }
    }

    let n_total: usize = index.iter().sum();
    if n_total == 0 {
        let d: Vec<Vec<f64>> = (0..p)
            .map(|i| (0..m).map(|j| ucoeff[i][j][kdcoef - 1]).collect())
            .collect();
        return Ok(Td04AdResult {
            order: 0,
            a: vec![],
            b: vec![],
            c: vec![],
            d,
        });
    }

    let mut a = vec![vec![0.0; n_total]; n_total];
    let mut b = vec![vec![0.0; m]; n_total];
    let mut c = vec![vec![0.0; n_total]; p];
    let mut d = vec![vec![0.0; m]; p];

    let mut row_start = 0_usize;
    for i in 0..p {
        let ni = index[i];
        let lead = dcoeff[i][0];
        for r in 1..ni {
            a[row_start + r][row_start + r - 1] = 1.0;
        }
        for col in 0..ni {
            a[row_start + ni - 1][row_start + col] =
                -dcoeff[i][kdcoef - 1 - col] / lead;
        }
        for j in 0..m {
            b[row_start + ni - 1][j] = ucoeff[i][j][kdcoef - 1] / lead;
        }
        c[i][row_start + ni - 1] = 1.0;
        for j in 0..m {
            d[i][j] = ucoeff[i][j][0] / lead;
        }
        row_start += ni;
    }

    let minr = tb01pd_minreal(&a, &b, &c)?;
    Ok(Td04AdResult {
        order: minr.order,
        a: minr.a,
        b: minr.b,
        c: minr.c,
        d,
    })
}

#[cfg(test)]
mod tests {
    use super::td04ad_tf2ss;

    #[test]
    fn td04ad_siso_strictly_proper() {
        let index = vec![1];
        let dcoeff = vec![vec![1.0, 1.0]];
        let ucoeff = vec![vec![vec![0.0, 1.0]]];
        let result = td04ad_tf2ss('R', 1, 1, &index, &dcoeff, &ucoeff, 0.0).expect("tf2ss");
        assert_eq!(result.order, 1);
        assert_eq!(result.a.len(), 1);
        assert!((result.a[0][0] + 1.0).abs() < 1e-10);
        assert_eq!(result.d[0][0], 0.0);
    }
}
