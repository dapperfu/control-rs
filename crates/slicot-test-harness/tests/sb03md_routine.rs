use std::path::{Path, PathBuf};

use slicot_routines::sb03md_solve;
use slicot_test_harness::load_sb03md_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb03md_matches_upstream_fixture() {
    let sb03md = load_sb03md_case(examples_root()).expect("SB03MD fixture should parse");
    let result = sb03md_solve(
        sb03md.input.dico,
        sb03md.input.job,
        sb03md.input.fact,
        sb03md.input.trana,
        &sb03md.input.a,
        &sb03md.input.c,
    )
    .expect("SB03MD routine should solve the fixture");

    for (actual_row, expected_row) in result.x.iter().zip(&sb03md.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((actual - expected).abs() < 1.0e-8);
        }
    }
    assert!((result.scale - sb03md.output.scale).abs() < 1.0e-12);
}
