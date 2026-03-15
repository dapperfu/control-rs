use std::path::{Path, PathBuf};

use slicot_test_harness::load_sb02md_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_sb02md_input_fixture() {
    let sb02md = load_sb02md_case(examples_root()).expect("SB02MD fixture should parse");

    assert_eq!(sb02md.input.n, 2);
    assert_eq!(sb02md.input.dico, 'C');
    assert_eq!(sb02md.input.a[0], vec![0.0, 1.0]);
    assert_eq!(sb02md.input.a[1], vec![0.0, 0.0]);
    assert_eq!(sb02md.input.q[0], vec![1.0, 0.0]);
    assert_eq!(sb02md.input.q[1], vec![0.0, 2.0]);
    assert_eq!(sb02md.input.g[0], vec![0.0, 0.0]);
    assert_eq!(sb02md.input.g[1], vec![0.0, 1.0]);
}

#[test]
fn parses_sb02md_output_fixture() {
    let sb02md = load_sb02md_case(examples_root()).expect("SB02MD fixture should parse");

    assert!((sb02md.output.rcond - 0.31).abs() < 1.0e-6);
    assert_eq!(sb02md.output.x[0], vec![2.0, 1.0]);
    assert_eq!(sb02md.output.x[1], vec![1.0, 2.0]);
}

#[test]
fn sb02md_case_loads() {
    let case = load_sb02md_case(examples_root()).expect("SB02MD case should load");
    assert_eq!(case.input.n, 2);
    assert_eq!(case.output.x.len(), 2);
}
