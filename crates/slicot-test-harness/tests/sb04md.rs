use std::path::{Path, PathBuf};

use slicot_test_harness::load_sb04md_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_sb04md_input_fixture() {
    let sb04md = load_sb04md_case(examples_root()).expect("SB04MD fixture should parse");

    assert_eq!(sb04md.input.n, 3);
    assert_eq!(sb04md.input.m, 2);
    assert_eq!(sb04md.input.a[0], vec![2.0, 1.0, 3.0]);
    assert_eq!(sb04md.input.b[1], vec![1.0, 6.0]);
}

#[test]
fn parses_sb04md_output_fixture() {
    let sb04md = load_sb04md_case(examples_root()).expect("SB04MD fixture should parse");

    assert_eq!(sb04md.output.x[0], vec![-2.7685, 0.5498]);
    assert_eq!(sb04md.output.z[0], vec![-0.9732, -0.2298]);
}
