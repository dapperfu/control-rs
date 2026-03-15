//! Parsers for the upstream `TB01PD` (minimal realization) example assets.
//!
//! TB01PD computes a minimal state-space realization. No routine port yet;
//! harness provides parse tests and an ignored golden routine test.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `TB01PD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb01PdCase {
    pub input: Tb01PdInput,
    pub output: Tb01PdOutput,
}

/// Parsed `TB01PD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb01PdInput {
    pub n: usize,
    pub m: usize,
    pub p: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
}

/// Parsed `TB01PD` output data (minimal realization).
#[derive(Clone, Debug, PartialEq)]
pub struct Tb01PdOutput {
    pub order: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
}

/// Errors produced while parsing the upstream `TB01PD` assets.
#[derive(Debug, Error)]
pub enum Tb01PdExampleError {
    #[error("failed to read TB01PD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing TB01PD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of TB01PD data while parsing {field}")]
    UnexpectedEnd { field: &'static str },
    #[error("failed to parse integer for {field} from token {token}: {source}")]
    ParseInt {
        field: &'static str,
        token: String,
        #[source]
        source: ParseIntError,
    },
    #[error("failed to parse float for {field} from token {token}: {source}")]
    ParseFloat {
        field: &'static str,
        token: String,
        #[source]
        source: ParseFloatError,
    },
}

/// Loads the checked-in upstream `TB01PD` example from `root`.
///
/// # Errors
///
/// Returns [`Tb01PdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_tb01pd_case(root: impl AsRef<Path>) -> Result<Tb01PdCase, Tb01PdExampleError> {
    let root = root.as_ref();
    let input = parse_tb01pd_input_file(root.join("data/TB01PD.dat"))?;
    let output = parse_tb01pd_result_file(
        root.join("results/TB01PD.res"),
        input.n,
        input.m,
        input.p,
    )?;
    Ok(Tb01PdCase { input, output })
}

/// Parses the upstream `TB01PD.dat` file.
///
/// # Errors
///
/// Returns [`Tb01PdExampleError`] if the file cannot be read or parsed.
pub fn parse_tb01pd_input_file(
    path: impl AsRef<Path>,
) -> Result<Tb01PdInput, Tb01PdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Tb01PdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Tb01PdExampleError::UnexpectedEnd { field: "header" })?;
    let mut tokens = header.split_whitespace();
    let n = parse_next_usize(&mut tokens, "n")?;
    let m = parse_next_usize(&mut tokens, "m")?;
    let p = parse_next_usize(&mut tokens, "p")?;
    // Skip TOL and mode flags (2 tokens)
    let _ = tokens.next();
    let _ = tokens.next();

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut body_tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut body_tokens, n, n, "A")?;
    let b = read_row_major_matrix(&mut body_tokens, n, m, "B")?;
    let c = read_row_major_matrix(&mut body_tokens, p, n, "C")?;

    Ok(Tb01PdInput { n, m, p, a, b, c })
}

/// Parses the checked-in `TB01PD.res` file.
///
/// # Errors
///
/// Returns [`Tb01PdExampleError`] if the result file cannot be read or parsed.
pub fn parse_tb01pd_result_file(
    path: impl AsRef<Path>,
    _n: usize,
    _m: usize,
    p: usize,
) -> Result<Tb01PdOutput, Tb01PdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Tb01PdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let order = parse_order_from_res(&lines)?;
    let a_index = find_line(&lines, "The transformed state dynamics matrix")?;
    let b_index = find_line(&lines, "The transformed input/state matrix")?;
    let c_index = find_line(&lines, "The transformed state/output matrix")?;

    let a = read_matrix_rows(&lines, a_index + 1, order, "A")?;
    let b = read_matrix_rows(&lines, b_index + 1, order, "B")?;
    let c = read_matrix_rows(&lines, c_index + 1, p, "C")?;

    Ok(Tb01PdOutput {
        order,
        a,
        b,
        c,
    })
}

fn parse_order_from_res(lines: &[&str]) -> Result<usize, Tb01PdExampleError> {
    let line = lines
        .iter()
        .find(|l| l.contains("order of the minimal realization"))
        .ok_or(Tb01PdExampleError::MissingSection {
            section: "order of the minimal realization",
        })?;
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Tb01PdExampleError::MissingSection { section: "order value" })?
        .trim();
    token.parse::<usize>().map_err(|source| Tb01PdExampleError::ParseInt {
        field: "order",
        token: token.to_owned(),
        source,
    })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Tb01PdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Tb01PdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Tb01PdExampleError> {
    tokens
        .next()
        .ok_or(Tb01PdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Tb01PdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Tb01PdExampleError::ParseInt {
        field,
        token: token.to_owned(),
        source,
    })
}

fn read_row_major_matrix<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    rows: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Tb01PdExampleError> {
    let mut matrix = vec![vec![0.0; columns]; rows];
    for row in &mut matrix {
        for value in row {
            *value = parse_next_f64(tokens, field)?;
        }
    }
    Ok(matrix)
}

fn parse_next_f64<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<f64, Tb01PdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<f64>().map_err(|source| Tb01PdExampleError::ParseFloat {
        field,
        token: token.to_owned(),
        source,
    })
}

fn read_matrix_rows(
    lines: &[&str],
    start: usize,
    row_count: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Tb01PdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Tb01PdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Tb01PdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Tb01PdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}
