//! Parse tests for the upstream SB03UD (discrete Lyapunov) example fixtures.

use std::path::{Path, PathBuf};

use slicot_test_harness::{
    load_sb03ud_case, parse_sb03ud_input_file, parse_sb03ud_result_file,
};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn sb03ud_dat_parses() {
    let path = examples_root().join("data/SB03UD.dat");
    let input = parse_sb03ud_input_file(path).expect("SB03UD.dat should parse");
    assert_eq!(input.n, 3);
    assert_eq!(input.a.len(), 3);
    assert_eq!(input.a[0].len(), 3);
    assert_eq!(input.c.len(), 3);
    assert_eq!(input.c[0].len(), 3);
}

#[test]
fn sb03ud_res_parses() {
    let path = examples_root().join("results/SB03UD.res");
    let output =
        parse_sb03ud_result_file(path, 3).expect("SB03UD.res should parse");
    assert_eq!(output.x.len(), 3);
    assert_eq!(output.x[0].len(), 3);
    assert!((output.scale - 1.0).abs() < 1e-12);
}

#[test]
fn sb03ud_case_loads() {
    let case = load_sb03ud_case(examples_root()).expect("SB03UD case should load");
    assert_eq!(case.input.n, 3);
    assert_eq!(case.output.x.len(), 3);
}
