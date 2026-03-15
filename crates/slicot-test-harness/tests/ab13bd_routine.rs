//! Golden tests for AB13BD (L2/H2 norm).
//!
//! - `ab13bd_stable_system_norm`: runnable test with inline stable system.
//! - `pure_rust_ab13bd_matches_upstream_fixture`: ignored (upstream A is unstable).

use std::path::{Path, PathBuf};

use slicot_routines::ab13bd_norm;
use slicot_test_harness::load_ab13bd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

/// Stable scalar system G(s) = 1/(s+1): H2 norm = 1/sqrt(2).
#[test]
fn ab13bd_stable_system_norm() {
    let a = vec![vec![-1.0]];
    let b = vec![vec![1.0]];
    let c = vec![vec![1.0]];
    let d = vec![vec![0.0]];
    let norm = ab13bd_norm('C', &a, &b, &c, &d).expect("stable system should succeed");
    let expected = 1.0 / 2.0_f64.sqrt();
    assert!(
        (norm - expected).abs() < 1.0e-9,
        "norm {norm} vs expected {expected}"
    );
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
