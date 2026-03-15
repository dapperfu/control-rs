//! Golden test: discrete Sylvester X + A X B = C via SB04QD matches SB04RD fixture.

use std::path::{Path, PathBuf};

use slicot_routines::sb04qd_solve;
use slicot_test_harness::load_sb04rd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb04rd_via_sb04qd_matches_upstream_fixture() {
    let case = load_sb04rd_case(examples_root()).expect("SB04RD fixture should parse");
    let result = sb04qd_solve(&case.input.a, &case.input.b, &case.input.c)
        .expect("SB04QD (X + A X B = C) should solve the SB04RD fixture");

    for (actual_row, expected_row) in result.x.iter().zip(&case.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((actual - expected).abs() < 1.0e-8);
        }
    }
}
