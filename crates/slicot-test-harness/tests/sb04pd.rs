//! Parse tests for the upstream SB04PD (discrete Sylvester) example fixtures.

use std::path::{Path, PathBuf};

use slicot_test_harness::{
    load_sb04pd_case, parse_sb04pd_input_file, parse_sb04pd_result_file,
};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn sb04pd_dat_parses() {
    let path = examples_root().join("data/SB04PD.dat");
    let input = parse_sb04pd_input_file(path).expect("SB04PD.dat should parse");
    assert_eq!(input.n, 3);
    assert_eq!(input.m, 2);
    assert_eq!(input.a.len(), 3);
    assert_eq!(input.a[0].len(), 3);
    assert_eq!(input.b.len(), 2);
    assert_eq!(input.b[0].len(), 2);
    assert_eq!(input.c.len(), 3);
    assert_eq!(input.c[0].len(), 2);
}

#[test]
fn sb04pd_res_parses() {
    let path = examples_root().join("results/SB04PD.res");
    let output =
        parse_sb04pd_result_file(path, 3, 2).expect("SB04PD.res should parse");
    assert_eq!(output.x.len(), 3);
    assert_eq!(output.x[0].len(), 2);
}

#[test]
fn sb04pd_case_loads() {
    let case = load_sb04pd_case(examples_root()).expect("SB04PD case should load");
    assert_eq!(case.input.n, 3);
    assert_eq!(case.input.m, 2);
    assert_eq!(case.output.x.len(), 3);
}
