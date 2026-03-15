use std::path::{Path, PathBuf};

use slicot_routines::tb04ad_transfer_matrix;
use slicot_test_harness::{load_tb04ad_case, TransferPolynomial};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn pure_rust_tb04ad_matches_upstream_transfer_polynomials_by_evaluation() {
    let tb04ad = load_tb04ad_case(examples_root()).expect("TB04AD fixture should parse");
    let result = tb04ad_transfer_matrix(&tb04ad.input.a, &tb04ad.input.b, &tb04ad.input.c, &tb04ad.input.d)
        .expect("TB04AD transfer matrix should evaluate");

    for expected in &tb04ad.output.transfer_polynomials {
        let actual = result
            .transfer_polynomials
            .iter()
            .find(|entry| entry.row == expected.row && entry.column == expected.column)
            .expect("every upstream element should be present");
        for point in [-0.5_f64, 0.5, 2.0, 4.0] {
            let actual_value = evaluate_rational(&actual.numerator, &actual.denominator, point);
            let expected_value = evaluate_expected(expected, point);
            assert!(
                (actual_value - expected_value).abs() < 1.0e-8,
                "entry ({}, {}) mismatch at s={point}: actual={actual_value}, expected={expected_value}",
                expected.row,
                expected.column,
            );
        }
    }
}

fn evaluate_expected(polynomial: &TransferPolynomial, point: f64) -> f64 {
    let degree = if polynomial.denominator.last().is_some_and(|value| value.abs() < 1.0e-12) {
        polynomial.denominator.len() - 2
    } else {
        polynomial.denominator.len() - 1
    };
    let numerator = evaluate_polynomial(&polynomial.numerator[..=degree], point);
    let denominator = evaluate_polynomial(&polynomial.denominator[..=degree], point);
    numerator / denominator
}

fn evaluate_rational(numerator: &[f64], denominator: &[f64], point: f64) -> f64 {
    evaluate_polynomial(numerator, point) / evaluate_polynomial(denominator, point)
}

fn evaluate_polynomial(coefficients: &[f64], point: f64) -> f64 {
    coefficients
        .iter()
        .fold(0.0, |accumulator, coefficient| accumulator * point + coefficient)
}
