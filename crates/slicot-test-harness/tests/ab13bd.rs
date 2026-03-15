use std::path::{Path, PathBuf};

use slicot_test_harness::load_ab13bd_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_ab13bd_input_fixture() {
    let ab13bd = load_ab13bd_case(examples_root()).expect("AB13BD fixture should parse");

    assert_eq!(ab13bd.input.n, 7);
    assert_eq!(ab13bd.input.m, 2);
    assert_eq!(ab13bd.input.p, 3);
    assert_eq!(ab13bd.input.dico, 'C');
    assert_eq!(ab13bd.input.jobn, 'L');
    assert!((ab13bd.input.a[0][0] - (-0.04165)).abs() < 1.0e-6);
    assert_eq!(ab13bd.input.b.len(), 7);
    assert_eq!(ab13bd.input.c.len(), 3);
}

#[test]
fn parses_ab13bd_output_fixture() {
    let ab13bd = load_ab13bd_case(examples_root()).expect("AB13BD fixture should parse");

    assert!((ab13bd.output.norm - 7.939_48).abs() < 1.0e-5);
}

#[test]
fn ab13bd_case_loads() {
    let case = load_ab13bd_case(examples_root()).expect("AB13BD case should load");
    assert_eq!(case.input.n, 7);
    assert!(case.output.norm > 0.0);
}
