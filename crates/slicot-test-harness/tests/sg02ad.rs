//! Parse tests for the upstream SG02AD (generalized Riccati) example fixtures.

use std::path::{Path, PathBuf};

use slicot_test_harness::{
    load_sg02ad_case, parse_sg02ad_input_file, parse_sg02ad_result_file,
};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn sg02ad_dat_parses() {
    let path = examples_root().join("data/SG02AD.dat");
    let input = parse_sg02ad_input_file(path).expect("SG02AD.dat should parse");
    assert_eq!(input.n, 2);
    assert_eq!(input.m, 1);
    assert_eq!(input.dico, 'C');
    assert_eq!(input.a.len(), 2);
    assert_eq!(input.e.len(), 2);
    assert_eq!(input.b.len(), 2);
    assert_eq!(input.q.len(), 2);
    assert_eq!(input.r.len(), 1);
    assert_eq!(input.l.len(), 2);
}

#[test]
fn sg02ad_res_parses() {
    let path = examples_root().join("results/SG02AD.res");
    let output =
        parse_sg02ad_result_file(path, 2).expect("SG02AD.res should parse");
    assert_eq!(output.x.len(), 2);
    assert_eq!(output.x[0].len(), 2);
}

#[test]
fn sg02ad_case_loads() {
    let case = load_sg02ad_case(examples_root()).expect("SG02AD case should load");
    assert_eq!(case.input.n, 2);
    assert_eq!(case.input.m, 1);
    assert_eq!(case.output.x.len(), 2);
}
