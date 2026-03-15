use std::path::{Path, PathBuf};

use slicot_routines::sg03ad_solve;
use slicot_test_harness::load_sg03ad_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sg03ad_matches_upstream_example() {
    let sg03ad = load_sg03ad_case(examples_root()).expect("SG03AD fixture should parse");

    let result = sg03ad_solve(
        sg03ad.input.dico,
        sg03ad.input.job,
        sg03ad.input.fact,
        sg03ad.input.trans,
        &sg03ad.input.a,
        &sg03ad.input.e,
        &sg03ad.input.y,
    )
    .expect("SG03AD routine should solve the upstream example");

    for (actual_row, expected_row) in result.x.iter().zip(&sg03ad.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!(
                (actual - expected).abs() < 1.0e-10,
                "solution mismatch: actual={actual}, expected={expected}"
            );
        }
    }

    assert!((result.scale - sg03ad.output.scale).abs() < 1.0e-12);

    let sep = result.sep.expect("JOB='B' should produce a sep estimate");
    assert!((sep - sg03ad.output.sep).abs() < 5.0e-2);

    let ferr = result.ferr.expect("JOB='B' should produce a ferr estimate");
    assert!(ferr < 1.0e-12);
}
