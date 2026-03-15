//! Parse tests for the upstream SG03BD (generalized Lyapunov Cholesky) example fixtures.

use std::path::{Path, PathBuf};

use slicot_test_harness::{
    load_sg03bd_case, parse_sg03bd_input_file, parse_sg03bd_result_file,
};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn sg03bd_dat_parses() {
    let path = examples_root().join("data/SG03BD.dat");
    let input = parse_sg03bd_input_file(path).expect("SG03BD.dat should parse");
    assert_eq!(input.n, 3);
    assert_eq!(input.m, 1);
    assert_eq!(input.dico, 'C');
    assert_eq!(input.a.len(), 3);
    assert_eq!(input.e.len(), 3);
    assert_eq!(input.b.len(), 1);
    assert_eq!(input.b[0].len(), 3);
}

#[test]
fn sg03bd_res_parses() {
    let path = examples_root().join("results/SG03BD.res");
    let output =
        parse_sg03bd_result_file(path, 3).expect("SG03BD.res should parse");
    assert_eq!(output.u.len(), 3);
    assert_eq!(output.u[0].len(), 3);
    assert!((output.scale - 1.0).abs() < 1e-12);
}

#[test]
fn sg03bd_case_loads() {
    let case = load_sg03bd_case(examples_root()).expect("SG03BD case should load");
    assert_eq!(case.input.n, 3);
    assert_eq!(case.output.u.len(), 3);
}
