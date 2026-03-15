use std::path::{Path, PathBuf};

use slicot_test_harness::load_sb03od_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_sb03od_input_fixture() {
    let sb03od = load_sb03od_case(examples_root()).expect("SB03OD fixture should parse");

    assert_eq!(sb03od.input.n, 4);
    assert_eq!(sb03od.input.m, 5);
    assert_eq!(sb03od.input.dico, 'C');
    assert_eq!(sb03od.input.fact, 'N');
    assert_eq!(sb03od.input.trans, 'N');
    assert_eq!(sb03od.input.a[0], vec![-1.0, 37.0, -12.0, -12.0]);
}

#[test]
fn parses_sb03od_output_fixture() {
    let sb03od = load_sb03od_case(examples_root()).expect("SB03OD fixture should parse");

    assert_eq!(sb03od.output.u_transpose[0], vec![1.0, 0.0, 0.0, 0.0]);
    assert_eq!(sb03od.output.u_transpose[3], vec![-1.0, 1.0, -2.0, 1.0]);
    assert_eq!(sb03od.output.x[1], vec![3.0, 10.0, 5.0, -2.0]);
    assert!((sb03od.output.scale - 1.0).abs() < 1.0e-12);
}
