use std::path::{Path, PathBuf};

use num_complex::Complex64;
use slicot_test_harness::load_tb05ad_case;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn parses_tb05ad_input_fixture() {
    let tb05ad = load_tb05ad_case(examples_root()).expect("TB05AD fixture should parse");

    assert_eq!(tb05ad.input.n, 3);
    assert_eq!(tb05ad.input.m, 1);
    assert_eq!(tb05ad.input.p, 2);
    assert_eq!(tb05ad.input.freq, Complex64::new(0.0, 0.5));
    assert_eq!(tb05ad.input.inita, 'G');
    assert_eq!(tb05ad.input.baleig, 'A');
    assert_eq!(tb05ad.input.a[0], vec![1.0, 2.0, 0.0]);
}

#[test]
fn parses_tb05ad_output_fixture() {
    let tb05ad = load_tb05ad_case(examples_root()).expect("TB05AD fixture should parse");

    assert_eq!(tb05ad.output.rcond, Some(0.22));
    assert_eq!(
        tb05ad.output.eigenvalues,
        vec![
            Complex64::new(3.0, 0.0),
            Complex64::new(-3.0, 0.0),
            Complex64::new(1.0, 0.0)
        ]
    );
    assert_eq!(tb05ad.output.g[0][0], Complex64::new(0.69, 0.35));
    assert_eq!(tb05ad.output.g[1][0], Complex64::new(-0.80, -0.40));
    assert_eq!(tb05ad.output.hinvb[2][0], Complex64::new(-0.80, -0.40));
}
