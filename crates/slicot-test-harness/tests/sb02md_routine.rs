//! Placeholder for SB02MD (CARE) routine golden test.
//!
//! The CARE solver is not yet implemented; Newton iteration for the upstream
//! 2×2 example did not converge. A Hamiltonian or other method is required.
//! This test is ignored until the routine is ported.

use std::path::{Path, PathBuf};

use slicot_test_harness::load_sb02md_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
#[ignore = "CARE solver (SB02MD) not yet implemented; Newton diverged for 2×2 example"]
fn pure_rust_sb02md_matches_upstream_fixture() {
    let _case = load_sb02md_case(examples_root()).expect("SB02MD fixture should parse");
    // When sb02md_solve is implemented:
    // let result = sb02md_solve(...).expect("SB02MD should solve");
    // compare result.x to case.output.x, result.rcond to case.output.rcond
}
