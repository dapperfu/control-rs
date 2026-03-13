use std::path::{Path, PathBuf};

use slicot_test_harness::load_sb04qd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_sb04qd_input_fixture() {
    let sb04qd = load_sb04qd_case(examples_root()).expect("SB04QD fixture should parse");

    assert_eq!(sb04qd.input.n, 3);
    assert_eq!(sb04qd.input.m, 3);
    assert_eq!(sb04qd.input.a[0], vec![1.0, 2.0, 3.0]);
    assert_eq!(sb04qd.input.b[1], vec![2.0, 1.0, 2.0]);
}

#[test]
fn parses_sb04qd_output_fixture() {
    let sb04qd = load_sb04qd_case(examples_root()).expect("SB04QD fixture should parse");

    assert_eq!(sb04qd.output.x[0], vec![2.0, 3.0, 6.0]);
    assert_eq!(sb04qd.output.z[0], vec![0.8337, 0.5204, -0.1845]);
}
