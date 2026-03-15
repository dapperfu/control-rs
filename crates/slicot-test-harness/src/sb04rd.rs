//! Parsers for the upstream `SB04RD` (Sylvester equation) example assets.
//!
//! SB04RD uses a different equation form or scaling than SB04MD; no golden
//! routine test is provided. Parse and case-load tests only.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SB04RD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04RdCase {
    pub input: Sb04RdInput,
    pub output: Sb04RdOutput,
}

/// Parsed `SB04RD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04RdInput {
    pub n: usize,
    pub m: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
}

/// Parsed `SB04RD` output data (solution X only).
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04RdOutput {
    pub x: Vec<Vec<f64>>,
}

/// Errors produced while parsing the upstream `SB04RD` assets.
#[derive(Debug, Error)]
pub enum Sb04RdExampleError {
    #[error("failed to read SB04RD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SB04RD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SB04RD data while parsing {field}")]
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

/// Loads the checked-in upstream `SB04RD` example from `root`.
///
/// # Errors
///
/// Returns [`Sb04RdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sb04rd_case(root: impl AsRef<Path>) -> Result<Sb04RdCase, Sb04RdExampleError> {
    let root = root.as_ref();
    let input = parse_sb04rd_input_file(root.join("data/SB04RD.dat"))?;
    let output = parse_sb04rd_result_file(root.join("results/SB04RD.res"), input.n, input.m)?;
    Ok(Sb04RdCase { input, output })
}

/// Parses the upstream `SB04RD.dat` file.
///
/// # Errors
///
/// Returns [`Sb04RdExampleError`] if the file cannot be read or parsed.
pub fn parse_sb04rd_input_file(path: impl AsRef<Path>) -> Result<Sb04RdInput, Sb04RdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb04RdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sb04RdExampleError::UnexpectedEnd { field: "header" })?;
    let mut header_tokens = header.split_whitespace();
    let n = parse_next_usize(&mut header_tokens, "n")?;
    let m = parse_next_usize(&mut header_tokens, "m")?;
    for _ in 0..4 {
        let _ = header_tokens.next();
    }

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut tokens, n, n, "A")?;
    let b = read_row_major_matrix(&mut tokens, m, m, "B")?;
    let c = read_row_major_matrix(&mut tokens, n, m, "C")?;

    Ok(Sb04RdInput { n, m, a, b, c })
}

/// Parses the checked-in `SB04RD.res` file.
///
/// # Errors
///
/// Returns [`Sb04RdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sb04rd_result_file(
    path: impl AsRef<Path>,
    n: usize,
    _m: usize,
) -> Result<Sb04RdOutput, Sb04RdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb04RdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let x_index = find_line(&lines, "The solution matrix X is")?;
    let x = read_matrix_rows(&lines, x_index + 1, n, "solution matrix")?;

    Ok(Sb04RdOutput { x })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sb04RdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sb04RdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sb04RdExampleError> {
    tokens
        .next()
        .ok_or(Sb04RdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sb04RdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Sb04RdExampleError::ParseInt {
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
) -> Result<Vec<Vec<f64>>, Sb04RdExampleError> {
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
) -> Result<f64, Sb04RdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<f64>().map_err(|source| Sb04RdExampleError::ParseFloat {
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
) -> Result<Vec<Vec<f64>>, Sb04RdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sb04RdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sb04RdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Sb04RdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}
