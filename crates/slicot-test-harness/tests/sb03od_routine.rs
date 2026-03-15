use std::path::{Path, PathBuf};

use slicot_routines::sb03od_factor;
use slicot_test_harness::load_sb03od_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb03od_matches_upstream_factor_and_solution() {
    let sb03od = load_sb03od_case(examples_root()).expect("SB03OD fixture should parse");

    let result = sb03od_factor(
        sb03od.input.dico,
        sb03od.input.fact,
        sb03od.input.trans,
        &sb03od.input.a,
        &sb03od.input.b,
    )
    .expect("SB03OD factorization should succeed");

    for (actual_row, expected_row) in transpose(&result.u).iter().zip(&sb03od.output.u_transpose) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((actual - expected).abs() < 1.0e-10);
        }
    }

    for (actual_row, expected_row) in result.x.iter().zip(&sb03od.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((actual - expected).abs() < 1.0e-10);
        }
    }

    assert!((result.scale - sb03od.output.scale).abs() < 1.0e-12);
}

fn transpose(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
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
