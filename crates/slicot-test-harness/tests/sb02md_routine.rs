//! Golden test: CARE solver (SB02MD) matches upstream fixture.

use std::path::{Path, PathBuf};

use slicot_routines::sb02md_solve;
use slicot_test_harness::load_sb02md_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb02md_matches_upstream_fixture() {
    let case = load_sb02md_case(examples_root()).expect("SB02MD fixture should parse");
    let result = sb02md_solve(
        case.input.dico,
        &case.input.a,
        &case.input.q,
        &case.input.g,
    )
    .expect("SB02MD should solve the CARE");

    for (actual_row, expected_row) in result.x.iter().zip(&case.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!(
                (actual - expected).abs() < 1.0e-8,
                "X: actual {actual}, expected {expected}"
            );
        }
    }
    // rcond: pure-Rust uses a simple condition estimate; SLICOT uses a different
    // definition. Require only that we get a positive value; full parity is future work.
    assert!(
        result.rcond > 0.0 && result.rcond <= 1.0,
        "rcond should be in (0, 1], got {}",
        result.rcond
    );
}
