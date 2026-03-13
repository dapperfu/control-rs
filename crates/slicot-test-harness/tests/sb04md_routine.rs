use std::path::{Path, PathBuf};

use slicot_routines::sb04md_solve;
use slicot_test_harness::load_sb04md_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sb04md_matches_upstream_solution_fixture() {
    let sb04md = load_sb04md_case(examples_root()).expect("SB04MD fixture should parse");
    let result = sb04md_solve(&sb04md.input.a, &sb04md.input.b, &sb04md.input.c)
        .expect("SB04MD routine should solve the fixture");

    for (actual_row, expected_row) in result.x.iter().zip(&sb04md.output.x) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((actual - expected).abs() < 1.0e-4);
        }
    }
}
