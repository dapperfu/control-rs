//! Parse tests for the upstream TB01PD (minimal realization) example fixtures.

use std::path::{Path, PathBuf};

use slicot_test_harness::{
    load_tb01pd_case, parse_tb01pd_input_file, parse_tb01pd_result_file,
};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn tb01pd_dat_parses() {
    let path = examples_root().join("data/TB01PD.dat");
    let input = parse_tb01pd_input_file(path).expect("TB01PD.dat should parse");
    assert_eq!(input.n, 3);
    assert_eq!(input.m, 1);
    assert_eq!(input.p, 2);
    assert_eq!(input.a.len(), 3);
    assert_eq!(input.b.len(), 3);
    assert_eq!(input.c.len(), 2);
}

#[test]
fn tb01pd_res_parses() {
    let path = examples_root().join("results/TB01PD.res");
    let output = parse_tb01pd_result_file(path, 3, 1, 2).expect("TB01PD.res should parse");
    assert_eq!(output.order, 3);
    assert_eq!(output.a.len(), 3);
    assert_eq!(output.b.len(), 3);
    assert_eq!(output.c.len(), 2);
}

#[test]
fn tb01pd_case_loads() {
    let case = load_tb01pd_case(examples_root()).expect("TB01PD case should load");
    assert_eq!(case.input.n, 3);
    assert_eq!(case.output.order, 3);
}
