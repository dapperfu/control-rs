use std::path::{Path, PathBuf};

use slicot_test_harness::load_sg03ad_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_sg03ad_input_fixture() {
    let sg03ad = load_sg03ad_case(examples_root()).expect("SG03AD fixture should parse");

    assert_eq!(sg03ad.input.n, 3);
    assert_eq!(sg03ad.input.job, 'B');
    assert_eq!(sg03ad.input.dico, 'C');
    assert_eq!(sg03ad.input.fact, 'N');
    assert_eq!(sg03ad.input.trans, 'N');
    assert_eq!(sg03ad.input.uplo, 'U');
    assert_eq!(sg03ad.input.y[1][0], -73.0);
}

#[test]
fn parses_sg03ad_output_fixture() {
    let sg03ad = load_sg03ad_case(examples_root()).expect("SG03AD fixture should parse");

    assert!((sg03ad.output.sep - 0.29).abs() < 1.0e-12);
    assert!(sg03ad.output.ferr < 1.0e-12);
    assert_eq!(sg03ad.output.x[0], vec![-2.0, -1.0, 0.0]);
}
