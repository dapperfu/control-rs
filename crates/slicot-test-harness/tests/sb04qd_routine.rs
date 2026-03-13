use std::path::{Path, PathBuf};

use slicot_routines::sb04qd_solve;
use slicot_test_harness::load_sb04qd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb04qd_matches_upstream_solution_fixture() {
    let sb04qd = load_sb04qd_case(examples_root()).expect("SB04QD fixture should parse");
    let result = sb04qd_solve(&sb04qd.input.a, &sb04qd.input.b, &sb04qd.input.c)
        .expect("SB04QD routine should solve the fixture");

    for (actual_row, expected_row) in result.x.iter().zip(&sb04qd.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((actual - expected).abs() < 1.0e-8);
        }
    }
}
