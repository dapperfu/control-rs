//! Golden test for TB01PD (minimal realization): compare order and dimensions.
//!
//! Element-wise comparison of (Am, Bm, Cm) is not performed because our SVD-based
//! basis may differ from SLICOT's; only order and matrix shapes are asserted.

use std::path::{Path, PathBuf};

use slicot_routines::tb01pd_minreal;
use slicot_test_harness::load_tb01pd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_tb01pd_matches_upstream_fixture() {
    let case = load_tb01pd_case(examples_root()).expect("TB01PD fixture should parse");
    let inp = &case.input;
    let result = tb01pd_minreal(&inp.a, &inp.b, &inp.c).expect("TB01PD should compute minimal realization");

    let out = &case.output;
    assert_eq!(result.order, out.order, "minimal order");
    assert_eq!(result.a.len(), out.a.len(), "Am rows");
    assert_eq!(result.a.first().map(|r| r.len()), out.a.first().map(|r| r.len()), "Am cols");
    assert_eq!(result.b.len(), out.b.len(), "Bm rows");
    assert_eq!(result.b.first().map(|r| r.len()), out.b.first().map(|r| r.len()), "Bm cols");
    assert_eq!(result.c.len(), out.c.len(), "Cm rows");
    assert_eq!(result.c.first().map(|r| r.len()), out.c.first().map(|r| r.len()), "Cm cols");
}
