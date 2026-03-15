//! Golden test: continuous Sylvester via SB04MD matches SB04ND fixture.

use std::path::{Path, PathBuf};

use slicot_routines::sb04md_solve;
use slicot_test_harness::load_sb04nd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb04nd_via_sb04md_matches_upstream_fixture() {
    let case = load_sb04nd_case(examples_root()).expect("SB04ND fixture should parse");
    let result = sb04md_solve(&case.input.a, &case.input.b, &case.input.c)
        .expect("SB04MD should solve the SB04ND Sylvester fixture");

    for (actual_row, expected_row) in result.x.iter().zip(&case.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((actual - expected).abs() < 1.0e-8);
        }
    }
}
