//! Placeholder for TB01PD (minimal realization) routine golden test.
//!
//! TB01PD is not yet implemented; it computes a minimal state-space realization.
//! This test is ignored until the routine is ported.

use std::path::{Path, PathBuf};

use slicot_test_harness::load_tb01pd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
#[ignore = "TB01PD (minimal realization) not yet implemented"]
fn pure_rust_tb01pd_matches_upstream_fixture() {
    let _case = load_tb01pd_case(examples_root()).expect("TB01PD fixture should parse");
    // When tb01pd_minreal is implemented:
    // let result = tb01pd_minreal(...).expect("TB01PD should compute minimal realization");
    // compare result.a, result.b, result.c to case.output
}
