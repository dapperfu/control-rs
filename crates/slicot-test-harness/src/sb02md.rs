//! Parsers for the upstream `SB02MD` example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SB02MD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb02MdCase {
    pub input: Sb02MdInput,
    pub output: Sb02MdOutput,
}

/// Parsed `SB02MD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb02MdInput {
    pub n: usize,
    pub dico: char,
    pub a: Vec<Vec<f64>>,
    pub q: Vec<Vec<f64>>,
    pub g: Vec<Vec<f64>>,
}

/// Parsed `SB02MD` output data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb02MdOutput {
    pub rcond: f64,
    pub x: Vec<Vec<f64>>,
}

/// Errors produced while parsing the upstream `SB02MD` assets.
#[derive(Debug, Error)]
pub enum Sb02MdExampleError {
    #[error("failed to read SB02MD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SB02MD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SB02MD data while parsing {field}")]
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
    #[error("invalid SB02MD mode flag: {value}")]
    InvalidModeFlag { value: String },
}

/// Loads the checked-in upstream `SB02MD` example from `root`.
///
/// # Errors
///
/// Returns [`Sb02MdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sb02md_case(root: impl AsRef<Path>) -> Result<Sb02MdCase, Sb02MdExampleError> {
    let root = root.as_ref();
    let input = parse_sb02md_input_file(root.join("data/SB02MD.dat"))?;
    let output = parse_sb02md_result_file(root.join("results/SB02MD.res"), input.n)?;
    Ok(Sb02MdCase { input, output })
}

/// Parses the upstream `SB02MD.dat` file.
///
/// # Errors
///
/// Returns [`Sb02MdExampleError`] if the file cannot be read or parsed.
pub fn parse_sb02md_input_file(
    path: impl AsRef<Path>,
) -> Result<Sb02MdInput, Sb02MdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb02MdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sb02MdExampleError::UnexpectedEnd { field: "header" })?;
    let mut tokens = header.split_whitespace();
    let n = parse_next_usize(&mut tokens, "n")?;
    let dico = parse_mode_flag(next_token(&mut tokens, "dico")?)?;

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut body_tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut body_tokens, n, n, "A")?;
    let q = read_row_major_matrix(&mut body_tokens, n, n, "Q")?;
    let g = read_row_major_matrix(&mut body_tokens, n, n, "G")?;

    Ok(Sb02MdInput { n, dico, a, q, g })
}

/// Parses the checked-in `SB02MD.res` file.
///
/// # Errors
///
/// Returns [`Sb02MdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sb02md_result_file(
    path: impl AsRef<Path>,
    order: usize,
) -> Result<Sb02MdOutput, Sb02MdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb02MdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let rcond_index = find_line(&lines, "RCOND =")?;
    let x_index = find_line(&lines, "The solution matrix X is")?;

    let rcond = parse_f64_after_equals(lines[rcond_index], "rcond")?;
    let x = read_matrix_rows(&lines, x_index + 1, order, "solution matrix")?;

    Ok(Sb02MdOutput { rcond, x })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sb02MdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sb02MdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sb02MdExampleError> {
    tokens
        .next()
        .ok_or(Sb02MdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sb02MdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Sb02MdExampleError::ParseInt {
        field,
        token: token.to_owned(),
        source,
    })
}

fn parse_mode_flag(token: &str) -> Result<char, Sb02MdExampleError> {
    let mut chars = token.chars();
    match chars.next() {
        Some(value) if chars.next().is_none() => Ok(value),
        _ => Err(Sb02MdExampleError::InvalidModeFlag {
            value: token.to_owned(),
        }),
    }
}

fn read_row_major_matrix<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    rows: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Sb02MdExampleError> {
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
) -> Result<f64, Sb02MdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<f64>().map_err(|source| Sb02MdExampleError::ParseFloat {
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
) -> Result<Vec<Vec<f64>>, Sb02MdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sb02MdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sb02MdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Sb02MdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}

fn parse_f64_after_equals(line: &str, field: &'static str) -> Result<f64, Sb02MdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Sb02MdExampleError::MissingSection { section: field })?
        .trim();
    token.parse::<f64>().map_err(|source| Sb02MdExampleError::ParseFloat {
        field,
        token: token.to_owned(),
        source,
    })
}
