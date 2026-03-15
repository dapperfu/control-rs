//! Golden test: SB03QD (continuous Lyapunov) via SB03MD.

use std::path::{Path, PathBuf};

use slicot_routines::sb03md_solve;
use slicot_test_harness::load_sb03qd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb03qd_via_sb03md_matches_upstream_fixture() {
    let case = load_sb03qd_case(examples_root()).expect("SB03QD fixture should parse");
    let result = sb03md_solve(
        'C',
        'X',
        'N',
        'N',
        &case.input.a,
        &case.input.c,
    )
    .expect("SB03MD should solve continuous Lyapunov");

    const TOL: f64 = 1.0e-4;
    for (i, (actual_row, expected_row)) in result.x.iter().zip(&case.output.x).enumerate() {
        for (j, (actual, expected)) in actual_row.iter().zip(expected_row).enumerate() {
            assert!(
                (actual - expected).abs() < TOL,
                "X[{i}][{j}] rust={actual} ref={expected}"
            );
        }
    }
    assert!(
        (result.scale - case.output.scale).abs() < 1.0e-10,
        "scale"
    );
}
