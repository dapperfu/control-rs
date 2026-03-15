//! Golden test for SG02AD (generalized Riccati): E = I, L = 0 subset.
//!
//! The implementation supports continuous-time with E identity and L zero.
//! The upstream fixture (SG02AD example) uses E = I and L = 0; the test must pass.

use std::path::{Path, PathBuf};

use slicot_routines::sg02ad_solve;
use slicot_test_harness::load_sg02ad_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_sg02ad_matches_upstream_fixture() {
    let case = load_sg02ad_case(examples_root()).expect("SG02AD fixture should parse");
    let inp = &case.input;
    let result = sg02ad_solve(
        inp.dico,
        &inp.a,
        &inp.e,
        &inp.b,
        &inp.q,
        &inp.r,
        &inp.l,
    )
    .expect("SG02AD solve must succeed for upstream fixture (E=I, L=0)");
    let out = &case.output;
    let n = result.x.len();
    assert_eq!(n, out.x.len(), "X rows");
    assert_eq!(result.x.first().map(|r| r.len()), out.x.first().map(|r| r.len()), "X cols");
    // Reference file uses 4 decimal places; allow one unit in the last place.
    const TOL: f64 = 1.0e-4;
    for (i, row) in result.x.iter().enumerate() {
        for (j, &x) in row.iter().enumerate() {
            let y = out.x.get(i).and_then(|r| r.get(j)).copied().unwrap_or(0.0);
            assert!((x - y).abs() < TOL, "X[{i}][{j}] rust={x} ref={y}");
        }
    }
}
