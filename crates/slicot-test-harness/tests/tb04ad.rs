use std::path::{Path, PathBuf};

use slicot_test_harness::load_tb04ad_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_tb04ad_input_fixture() {
    let tb04ad = load_tb04ad_case(examples_root()).expect("TB04AD fixture should parse");

    assert_eq!(tb04ad.input.n, 3);
    assert_eq!(tb04ad.input.m, 2);
    assert_eq!(tb04ad.input.p, 2);
    assert_eq!(tb04ad.input.rowcol, 'R');
    assert_eq!(tb04ad.input.a[0], vec![-1.0, 0.0, 0.0]);
    assert_eq!(tb04ad.input.b[0], vec![0.0, 1.0]);
    assert_eq!(tb04ad.input.b[2], vec![-1.0, 0.0]);
}

#[test]
fn parses_tb04ad_output_fixture() {
    let tb04ad = load_tb04ad_case(examples_root()).expect("TB04AD fixture should parse");

    assert_eq!(tb04ad.output.nr, 3);
    assert_eq!(tb04ad.output.controllability_index, 2);
    assert_eq!(tb04ad.output.diagonal_block_dimensions, vec![2, 1]);
    assert_eq!(tb04ad.output.denominator_degrees, vec![2, 3]);
    assert_eq!(tb04ad.output.transformed_a[0], vec![-2.5, -0.2887, -0.4082]);
    assert_eq!(tb04ad.output.transformed_b[1], vec![0.0, 1.2247]);
    assert_eq!(tb04ad.output.transformed_c[1], vec![0.0, 1.6330, 0.5774]);

    let first_element = &tb04ad.output.transfer_polynomials[0];
    assert_eq!(first_element.row, 1);
    assert_eq!(first_element.column, 1);
    assert_eq!(first_element.numerator, vec![1.0, 5.0, 7.0, 0.0]);
    assert_eq!(first_element.denominator, vec![1.0, 5.0, 6.0, 0.0]);
}
