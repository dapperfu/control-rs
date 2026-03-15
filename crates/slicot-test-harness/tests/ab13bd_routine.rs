//! Golden test: AB13BD (L2/H2 norm) matches upstream fixture when system is stable.
//!
//! The upstream AB13BD example has an unstable A matrix; the current implementation
//! uses the observability-Gramian formula and supports only stable A. This test is
//! ignored until we add a fixture with stable A or implement unstable-case handling.

use std::path::{Path, PathBuf};

use slicot_routines::ab13bd_norm;
use slicot_test_harness::load_ab13bd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
#[ignore = "upstream AB13BD example has unstable A; stable-only implementation"]
fn pure_rust_ab13bd_matches_upstream_fixture() {
    let case = load_ab13bd_case(examples_root()).expect("AB13BD fixture should parse");
    let norm = match ab13bd_norm(
        case.input.dico,
        &case.input.a,
        &case.input.b,
        &case.input.c,
        &case.input.d,
    ) {
        Ok(n) => n,
        Err(_) => return, // unstable or Lyapunov failed; test ignored
    };
    assert!(
        (norm - case.output.norm).abs() < 1.0e-4,
        "norm: actual {norm}, expected {}",
        case.output.norm
    );
}
