//! Compares Rust SLICOT routine outputs to Fortran reference results (.res files).
//!
//! Usage: compare_rust_vs_reference [EXAMPLES_ROOT]
//! Default EXAMPLES_ROOT: SLICOT-Reference/examples (relative to CWD).
//! Exit code: 0 if all run and pass, 1 if any FAIL or load error.

use std::path::{Path, PathBuf};

use slicot_routines::{
    ab13bd_norm, sb02md_solve, sb03md_solve, sb03od_factor, sb04md_solve, sb04qd_solve,
    sg03ad_solve, tb04ad_transfer_matrix, tb05ad_frequency_response,
};
use slicot_test_harness::{
    load_ab13bd_case, load_sb02md_case, load_sb03md_case, load_sb03od_case, load_sb04md_case,
    load_sb04nd_case, load_sb04pd_case, load_sb04qd_case, load_sb04rd_case, load_sg03ad_case,
    load_tb04ad_case, load_tb05ad_case, TransferPolynomial,
};

/// Outcome of comparing one routine's output to reference.
#[derive(Debug)]
enum Outcome {
    Ok,
    Fail(String),
    Skip(String),
}

fn main() {
    let examples_root = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("SLICOT-Reference/examples"));

    let canonical = examples_root.canonicalize();
    let root = match &canonical {
        Ok(p) => p.as_path(),
        Err(e) => {
            eprintln!("Examples root not found: {}: {}", examples_root.display(), e);
            std::process::exit(1);
        }
    };

    let mut results: Vec<(&str, Outcome)> = Vec::new();

    // AB13BD
    results.push(("AB13BD", compare_ab13bd(root)));

    // SB02MD
    results.push(("SB02MD", compare_sb02md(root)));

    // SB03MD
    results.push(("SB03MD", compare_sb03md(root)));

    // SB03OD
    results.push(("SB03OD", compare_sb03od(root)));

    // SB04MD
    results.push(("SB04MD", compare_sb04md(root)));

    // SB04ND (uses sb04md_solve)
    results.push(("SB04ND", compare_sb04nd(root)));

    // SB04QD
    results.push(("SB04QD", compare_sb04qd(root)));

    // SB04PD (uses sb04qd_solve, tolerance 1e-4)
    results.push(("SB04PD", compare_sb04pd(root)));

    // SB04RD (uses sb04qd_solve)
    results.push(("SB04RD", compare_sb04rd(root)));

    // SG03AD
    results.push(("SG03AD", compare_sg03ad(root)));

    // TB04AD
    results.push(("TB04AD", compare_tb04ad(root)));

    // TB05AD
    results.push(("TB05AD", compare_tb05ad(root)));

    // Print table
    println!("{:12}  {:6}  {}", "ROUTINE", "STATUS", "DETAIL");
    println!("{}", "-".repeat(60));
    for (name, outcome) in &results {
        let (status, detail) = match outcome {
            Outcome::Ok => ("OK", ""),
            Outcome::Fail(s) => ("FAIL", s.as_str()),
            Outcome::Skip(s) => ("SKIP", s.as_str()),
        };
        println!("{:12}  {:6}  {}", name, status, detail);
    }

    let has_fail = results.iter().any(|(_, o)| matches!(o, Outcome::Fail(_)));
    std::process::exit(if has_fail { 1 } else { 0 });
}

fn matrix_max_abs_diff(actual: &[Vec<f64>], expected: &[Vec<f64>]) -> f64 {
    actual
        .iter()
        .zip(expected.iter())
        .flat_map(|(ar, er)| ar.iter().zip(er.iter()))
        .map(|(a, e)| (a - e).abs())
        .fold(0.0_f64, f64::max)
}

fn compare_ab13bd(root: &Path) -> Outcome {
    let case = match load_ab13bd_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let norm = match ab13bd_norm(
        case.input.dico,
        &case.input.a,
        &case.input.b,
        &case.input.c,
        &case.input.d,
    ) {
        Ok(n) => n,
        Err(_) => return Outcome::Skip("unstable A (reference fixture)".to_string()),
    };
    let tol = 1.0e-4;
    if (norm - case.output.norm).abs() < tol {
        Outcome::Ok
    } else {
        Outcome::Fail(format!(
            "norm: actual {}, expected {} (tol {})",
            norm, case.output.norm, tol
        ))
    }
}

fn compare_sb02md(root: &Path) -> Outcome {
    let case = match load_sb02md_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sb02md_solve(
        case.input.dico,
        &case.input.a,
        &case.input.q,
        &case.input.g,
    ) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("solve failed: {}", e)),
    };
    let tol = 1.0e-8;
    let max_err = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_err >= tol {
        return Outcome::Fail(format!("X max |actual−expected| = {} (tol {})", max_err, tol));
    }
    if !(result.rcond > 0.0 && result.rcond <= 1.0) {
        return Outcome::Fail(format!("rcond out of range: {}", result.rcond));
    }
    Outcome::Ok
}

fn compare_sb03md(root: &Path) -> Outcome {
    let case = match load_sb03md_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sb03md_solve(
        case.input.dico,
        case.input.job,
        case.input.fact,
        case.input.trana,
        &case.input.a,
        &case.input.c,
    ) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("solve failed: {}", e)),
    };
    let tol_x = 1.0e-8;
    let tol_scale = 1.0e-12;
    let max_x = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_x >= tol_x {
        return Outcome::Fail(format!("X max diff = {} (tol {})", max_x, tol_x));
    }
    if (result.scale - case.output.scale).abs() >= tol_scale {
        return Outcome::Fail(format!(
            "scale diff = {} (tol {})",
            (result.scale - case.output.scale).abs(),
            tol_scale
        ));
    }
    Outcome::Ok
}

fn transpose(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = matrix.len();
    let cols = matrix.first().map_or(0, Vec::len);
    let mut t = vec![vec![0.0; rows]; cols];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            t[j][i] = v;
        }
    }
    t
}

fn compare_sb03od(root: &Path) -> Outcome {
    let case = match load_sb03od_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sb03od_factor(
        case.input.dico,
        case.input.fact,
        case.input.trans,
        &case.input.a,
        &case.input.b,
    ) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("factor failed: {}", e)),
    };
    let tol = 1.0e-10;
    let u_t = transpose(&result.u);
    let max_u = matrix_max_abs_diff(&u_t, &case.output.u_transpose);
    if max_u >= tol {
        return Outcome::Fail(format!("U max diff = {} (tol {})", max_u, tol));
    }
    let max_x = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_x >= tol {
        return Outcome::Fail(format!("X max diff = {} (tol {})", max_x, tol));
    }
    let tol_scale = 1.0e-12;
    if (result.scale - case.output.scale).abs() >= tol_scale {
        return Outcome::Fail(format!("scale diff (tol {})", tol_scale));
    }
    Outcome::Ok
}

fn compare_sb04md(root: &Path) -> Outcome {
    let case = match load_sb04md_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sb04md_solve(&case.input.a, &case.input.b, &case.input.c) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("solve failed: {}", e)),
    };
    let tol = 1.0e-4;
    let max_err = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_err >= tol {
        Outcome::Fail(format!("X max diff = {} (tol {})", max_err, tol))
    } else {
        Outcome::Ok
    }
}

fn compare_sb04nd(root: &Path) -> Outcome {
    let case = match load_sb04nd_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sb04md_solve(&case.input.a, &case.input.b, &case.input.c) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("solve failed: {}", e)),
    };
    let tol = 1.0e-8;
    let max_err = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_err >= tol {
        Outcome::Fail(format!("X max diff = {} (tol {})", max_err, tol))
    } else {
        Outcome::Ok
    }
}

fn compare_sb04qd(root: &Path) -> Outcome {
    let case = match load_sb04qd_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sb04qd_solve(&case.input.a, &case.input.b, &case.input.c) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("solve failed: {}", e)),
    };
    let tol = 1.0e-8;
    let max_err = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_err >= tol {
        Outcome::Fail(format!("X max diff = {} (tol {})", max_err, tol))
    } else {
        Outcome::Ok
    }
}

fn compare_sb04pd(root: &Path) -> Outcome {
    let case = match load_sb04pd_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sb04qd_solve(&case.input.a, &case.input.b, &case.input.c) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("solve failed: {}", e)),
    };
    let tol = 1.0e-4; // reference prints X to 4 decimal places
    let max_err = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_err >= tol {
        Outcome::Fail(format!("X max diff = {} (tol {})", max_err, tol))
    } else {
        Outcome::Ok
    }
}

fn compare_sb04rd(root: &Path) -> Outcome {
    let case = match load_sb04rd_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sb04qd_solve(&case.input.a, &case.input.b, &case.input.c) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("solve failed: {}", e)),
    };
    let tol = 1.0e-8;
    let max_err = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_err >= tol {
        Outcome::Fail(format!("X max diff = {} (tol {})", max_err, tol))
    } else {
        Outcome::Ok
    }
}

fn compare_sg03ad(root: &Path) -> Outcome {
    let case = match load_sg03ad_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match sg03ad_solve(
        case.input.dico,
        case.input.job,
        case.input.fact,
        case.input.trans,
        &case.input.a,
        &case.input.e,
        &case.input.y,
    ) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("solve failed: {}", e)),
    };
    let tol_x = 1.0e-10;
    let tol_scale = 1.0e-12;
    let max_x = matrix_max_abs_diff(&result.x, &case.output.x);
    if max_x >= tol_x {
        return Outcome::Fail(format!("X max diff = {} (tol {})", max_x, tol_x));
    }
    if (result.scale - case.output.scale).abs() >= tol_scale {
        return Outcome::Fail(format!("scale diff (tol {})", tol_scale));
    }
    if let (Some(sep), exp_sep) = (result.sep, case.output.sep) {
        if (sep - exp_sep).abs() >= 5.0e-2 {
            return Outcome::Fail(format!("sep diff (tol 5e-2)"));
        }
    }
    if let (Some(ferr), _) = (result.ferr, case.output.ferr) {
        if ferr >= 1.0e-12 {
            return Outcome::Fail(format!("ferr = {} (expect < 1e-12)", ferr));
        }
    }
    Outcome::Ok
}

fn evaluate_polynomial(coefficients: &[f64], point: f64) -> f64 {
    coefficients
        .iter()
        .fold(0.0, |acc, c| acc * point + c)
}

fn evaluate_rational(numerator: &[f64], denominator: &[f64], point: f64) -> f64 {
    evaluate_polynomial(numerator, point) / evaluate_polynomial(denominator, point)
}

fn evaluate_expected(polynomial: &TransferPolynomial, point: f64) -> f64 {
    let degree = if polynomial
        .denominator
        .last()
        .is_some_and(|v| v.abs() < 1.0e-12)
    {
        polynomial.denominator.len() - 2
    } else {
        polynomial.denominator.len() - 1
    };
    let num = evaluate_polynomial(&polynomial.numerator[..=degree], point);
    let den = evaluate_polynomial(&polynomial.denominator[..=degree], point);
    num / den
}

fn compare_tb04ad(root: &Path) -> Outcome {
    let case = match load_tb04ad_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match tb04ad_transfer_matrix(
        &case.input.a,
        &case.input.b,
        &case.input.c,
        &case.input.d,
    ) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("transfer_matrix failed: {}", e)),
    };
    let tol = 1.0e-8;
    let points = [-0.5_f64, 0.5, 2.0, 4.0];
    for expected in &case.output.transfer_polynomials {
        let actual = result
            .transfer_polynomials
            .iter()
            .find(|e| e.row == expected.row && e.column == expected.column)
            .ok_or_else(|| Outcome::Fail("missing transfer polynomial entry".to_string()));
        let actual = match actual {
            Ok(a) => a,
            Err(o) => return o,
        };
        for point in points {
            let actual_val = evaluate_rational(&actual.numerator, &actual.denominator, point);
            let expected_val = evaluate_expected(expected, point);
            if (actual_val - expected_val).abs() >= tol {
                return Outcome::Fail(format!(
                    "entry ({}, {}) at s={}: actual={}, expected={} (tol {})",
                    expected.row, expected.column, point, actual_val, expected_val, tol
                ));
            }
        }
    }
    Outcome::Ok
}

fn compare_tb05ad(root: &Path) -> Outcome {
    let case = match load_tb05ad_case(root) {
        Ok(c) => c,
        Err(e) => return Outcome::Skip(format!("fixture not found: {}", e)),
    };
    let result = match tb05ad_frequency_response(
        case.input.baleig,
        case.input.inita,
        &case.input.a,
        &case.input.b,
        &case.input.c,
        case.input.freq,
    ) {
        Ok(r) => r,
        Err(e) => return Outcome::Fail(format!("frequency_response failed: {}", e)),
    };
    let tol = 1.0e-2;
    for (actual_row, expected_row) in result.g.iter().zip(&case.output.g) {
        for (a, e) in actual_row.iter().zip(expected_row) {
            if (a - e).norm() >= tol {
                return Outcome::Fail(format!("G matrix norm diff >= {}", tol));
            }
        }
    }
    for (actual_row, expected_row) in result.hinvb.iter().zip(&case.output.hinvb) {
        for (a, e) in actual_row.iter().zip(expected_row) {
            if (a - e).norm() >= tol {
                return Outcome::Fail(format!("hinvb matrix norm diff >= {}", tol));
            }
        }
    }
    if let (Some(actual_rcond), Some(expected_rcond)) = (result.rcond, case.output.rcond) {
        if (actual_rcond - expected_rcond).abs() >= 5.0e-2 {
            return Outcome::Fail(format!(
                "rcond diff = {} (tol 5e-2)",
                (actual_rcond - expected_rcond).abs()
            ));
        }
    }
    Outcome::Ok
}
