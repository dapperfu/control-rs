//! Golden test: discrete Lyapunov via SB03MD with DICO='D' matches SB03SD fixture.

use std::path::{Path, PathBuf};

use slicot_routines::sb03md_solve;
use slicot_test_harness::load_sb03sd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb03sd_via_sb03md_matches_upstream_fixture() {
    let case = load_sb03sd_case(examples_root()).expect("SB03SD fixture should parse");
    let result = sb03md_solve(
        'D',
        'X',
        'N',
        'N',
        &case.input.a,
        &case.input.c,
    )
    .expect("SB03MD discrete Lyapunov should solve the SB03SD fixture");

    for (actual_row, expected_row) in result.x.iter().zip(&case.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((actual - expected).abs() < 1.0e-8);
        }
    }
    assert!((result.scale - case.output.scale).abs() < 1.0e-12);
}
