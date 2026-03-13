use std::path::{Path, PathBuf};

use slicot_routines::tb05ad_frequency_response;
use slicot_test_harness::load_tb05ad_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_tb05ad_matches_upstream_fixture() {
    let tb05ad = load_tb05ad_case(examples_root()).expect("TB05AD fixture should parse");
    let result = tb05ad_frequency_response(
        tb05ad.input.baleig,
        tb05ad.input.inita,
        &tb05ad.input.a,
        &tb05ad.input.b,
        &tb05ad.input.c,
        tb05ad.input.freq,
    )
    .expect("TB05AD routine should evaluate");

    let expected_g = &tb05ad.output.g;
    let expected_hinvb = &tb05ad.output.hinvb;
    for (actual_row, expected_row) in result.g.iter().zip(expected_g) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((*actual - *expected).norm() < 1.0e-2);
        }
    }
    for (actual_row, expected_row) in result.hinvb.iter().zip(expected_hinvb) {
        for (actual, expected) in actual_row.iter().zip(expected_row) {
            assert!((*actual - *expected).norm() < 1.0e-2);
        }
    }

    let actual_rcond = result.rcond.expect("rcond should be computed");
    let expected_rcond = tb05ad.output.rcond.expect("fixture includes rcond");
    assert!((actual_rcond - expected_rcond).abs() < 5.0e-2);
}
