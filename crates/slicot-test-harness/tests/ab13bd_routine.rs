//! Placeholder for AB13BD (L2/H2 norm) routine golden test.
//!
//! The L2/H2 norm routine is not yet implemented; the upstream example has
//! an unstable A matrix, so the simple observability-Gramian formula does not
//! apply. Full AB13BD would need handling for unstable systems.
//! This test is ignored until the routine is ported.

use std::path::{Path, PathBuf};

use slicot_test_harness::load_ab13bd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
#[ignore = "L2/H2 norm (AB13BD) not yet implemented; upstream example has unstable A"]
fn pure_rust_ab13bd_matches_upstream_fixture() {
    let _case = load_ab13bd_case(examples_root()).expect("AB13BD fixture should parse");
    // When ab13bd_norm (or equivalent) is implemented:
    // let result = ab13bd_norm(...).expect("AB13BD should compute norm");
    // assert!((result - case.output.norm).abs() < 1e-5);
}
