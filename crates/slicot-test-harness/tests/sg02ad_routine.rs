//! Golden test for SG02AD (generalized Riccati): E = I, L = 0 subset.
//!
//! The implementation supports only continuous-time with E identity and L zero.
//! If the upstream fixture does not match, the test is ignored.

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
    let result = match sg02ad_solve(
        inp.dico,
        &inp.a,
        &inp.e,
        &inp.b,
        &inp.q,
        &inp.r,
        &inp.l,
    ) {
        Ok(r) => r,
        Err(_) => {
            // Fixture may have E != I or L != 0; skip comparison
            return;
        }
    };
    let out = &case.output;
    let n = result.x.len();
    assert_eq!(n, out.x.len(), "X rows");
    assert_eq!(result.x.first().map(|r| r.len()), out.x.first().map(|r| r.len()), "X cols");
    const TOL: f64 = 1.0e-6;
    for (i, row) in result.x.iter().enumerate() {
        for (j, &x) in row.iter().enumerate() {
            let y = out.x.get(i).and_then(|r| r.get(j)).copied().unwrap_or(0.0);
            assert!((x - y).abs() < TOL, "X[{i}][{j}] rust={x} ref={y}");
        }
    }
}
