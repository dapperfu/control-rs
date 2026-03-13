use std::path::{Path, PathBuf};

use slicot_test_harness::load_sb03md_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_sb03md_input_fixture() {
    let sb03md = load_sb03md_case(examples_root()).expect("SB03MD fixture should parse");

    assert_eq!(sb03md.input.n, 3);
    assert_eq!(sb03md.input.dico, 'D');
    assert_eq!(sb03md.input.fact, 'N');
    assert_eq!(sb03md.input.job, 'X');
    assert_eq!(sb03md.input.trana, 'N');
    assert_eq!(sb03md.input.a[0], vec![3.0, 1.0, 1.0]);
}

#[test]
fn parses_sb03md_output_fixture() {
    let sb03md = load_sb03md_case(examples_root()).expect("SB03MD fixture should parse");

    assert_eq!(sb03md.output.x[0], vec![2.0, 1.0, 1.0]);
    assert_eq!(sb03md.output.x[2], vec![1.0, 0.0, 4.0]);
    assert_eq!(sb03md.output.scale, 1.0);
}
