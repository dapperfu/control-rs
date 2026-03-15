//! Parse tests for the upstream SB03QD (continuous Lyapunov) example fixtures.

use std::path::{Path, PathBuf};

use slicot_test_harness::{
    load_sb03qd_case, parse_sb03qd_input_file, parse_sb03qd_result_file,
};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn sb03qd_dat_parses() {
    let path = examples_root().join("data/SB03QD.dat");
    let input = parse_sb03qd_input_file(path).expect("SB03QD.dat should parse");
    assert_eq!(input.n, 3);
    assert_eq!(input.a.len(), 3);
    assert_eq!(input.a[0].len(), 3);
    assert_eq!(input.c.len(), 3);
    assert_eq!(input.c[0].len(), 3);
}

#[test]
fn sb03qd_res_parses() {
    let path = examples_root().join("results/SB03QD.res");
    let output =
        parse_sb03qd_result_file(path, 3).expect("SB03QD.res should parse");
    assert_eq!(output.x.len(), 3);
    assert_eq!(output.x[0].len(), 3);
    assert!((output.scale - 1.0).abs() < 1e-12);
}

#[test]
fn sb03qd_case_loads() {
    let case = load_sb03qd_case(examples_root()).expect("SB03QD case should load");
    assert_eq!(case.input.n, 3);
    assert_eq!(case.output.x.len(), 3);
}
