//! Parse tests for the upstream SB04ND (Sylvester) example fixtures.

use std::path::{Path, PathBuf};

use slicot_test_harness::{
    load_sb04nd_case, parse_sb04nd_input_file, parse_sb04nd_result_file,
};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn sb04nd_dat_parses() {
    let path = examples_root().join("data/SB04ND.dat");
    let input = parse_sb04nd_input_file(path).expect("SB04ND.dat should parse");
    assert_eq!(input.n, 5);
    assert_eq!(input.m, 3);
    assert_eq!(input.a.len(), 5);
    assert_eq!(input.a[0].len(), 5);
    assert_eq!(input.b.len(), 3);
    assert_eq!(input.b[0].len(), 3);
    assert_eq!(input.c.len(), 5);
    assert_eq!(input.c[0].len(), 3);
}

#[test]
fn sb04nd_res_parses() {
    let path = examples_root().join("results/SB04ND.res");
    let output =
        parse_sb04nd_result_file(path, 5, 3).expect("SB04ND.res should parse");
    assert_eq!(output.x.len(), 5);
    assert_eq!(output.x[0].len(), 3);
}

#[test]
fn sb04nd_case_loads() {
    let case = load_sb04nd_case(examples_root()).expect("SB04ND case should load");
    assert_eq!(case.input.n, 5);
    assert_eq!(case.input.m, 3);
    assert_eq!(case.output.x.len(), 5);
}
