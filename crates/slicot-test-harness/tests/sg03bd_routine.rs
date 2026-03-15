//! Golden test: generalized Lyapunov Cholesky via SG03AD + Cholesky matches SG03BD fixture.
//!
//! SG03BD solves A' X E + E' X A = -scale² B' B and returns U with X = U' U.
//! We form Y = -B' B, solve with sg03ad_solve, then take upper Cholesky of X.

use std::path::{Path, PathBuf};

use slicot_routines::sg03ad_solve;
use slicot_test_harness::load_sg03bd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

/// Forms Y = -B' B (B is m×n, so B'B is n×n). B is stored as rows.
fn neg_btb(b: &[Vec<f64>], n: usize) -> Vec<Vec<f64>> {
    let mut y = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for row in b {
                sum += row[i] * row[j];
            }
            y[i][j] = -sum;
        }
    }
    y
}

/// Upper Cholesky U such that X = U' U (X symmetric positive definite).
fn cholesky_upper(x: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = x.len();
    let mut l = vec![vec![0.0; n]; n];
    for row in 0..n {
        for col in 0..=row {
            let mut sum = x[row][col];
            for k in 0..col {
                sum -= l[row][k] * l[col][k];
            }
            if row == col {
                l[row][col] = sum.sqrt();
            } else {
                l[row][col] = sum / l[col][col];
            }
        }
    }
    transpose(&l)
}

fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = m.len();
    let cols = m.first().map_or(0, Vec::len);
    let mut t = vec![vec![0.0; rows]; cols];
    for (i, row) in m.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            t[j][i] = v;
        }
    }
    t
}

#[test]
fn pure_rust_sg03bd_via_sg03ad_and_cholesky_matches_upstream_fixture() {
    let case = load_sg03bd_case(examples_root()).expect("SG03BD fixture should parse");
    let y = neg_btb(&case.input.b, case.input.n);
    let result = sg03ad_solve(
        case.input.dico,
        'X',
        case.input.fact,
        'N',
        &case.input.a,
        &case.input.e,
        &y,
    )
    .expect("SG03AD should solve the generalized Lyapunov equation");
    let u = cholesky_upper(&result.x);

    for (actual_row, expected_row) in u.iter().zip(&case.output.u) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!(
                (actual - expected).abs() < 1.0e-4,
                "U mismatch: actual {actual} vs expected {expected}"
            );
        }
    }
    assert!((result.scale - case.output.scale).abs() < 1.0e-12);
}
