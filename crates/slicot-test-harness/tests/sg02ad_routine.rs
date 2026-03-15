//! Placeholder for SG02AD (generalized CARE/DARE) routine golden test.
//!
//! SG02AD is not yet implemented; it solves generalized algebraic Riccati
//! equations. This test is ignored until the routine is ported.

use std::path::{Path, PathBuf};

use slicot_test_harness::load_sg02ad_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
#[ignore = "SG02AD (generalized CARE/DARE) not yet implemented"]
fn pure_rust_sg02ad_matches_upstream_fixture() {
    let _case = load_sg02ad_case(examples_root()).expect("SG02AD fixture should parse");
    // When sg02ad_solve is implemented:
    // let result = sg02ad_solve(...).expect("SG02AD should solve");
    // compare result.x to case.output.x
}
