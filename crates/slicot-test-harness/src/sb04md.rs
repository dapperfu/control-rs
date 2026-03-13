//! Parsers for the upstream `SB04MD` example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SB04MD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04MdCase {
    pub input: Sb04MdInput,
    pub output: Sb04MdOutput,
}

/// Parsed `SB04MD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04MdInput {
    pub n: usize,
    pub m: usize,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
}

/// Parsed `SB04MD` output data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb04MdOutput {
    pub x: Vec<Vec<f64>>,
    pub z: Vec<Vec<f64>>,
}

/// Errors produced while parsing the upstream `SB04MD` assets.
#[derive(Debug, Error)]
pub enum Sb04MdExampleError {
    #[error("failed to read SB04MD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SB04MD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SB04MD data while parsing {field}")]
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

/// Loads the checked-in upstream `SB04MD` example from `root`.
///
/// # Errors
///
/// Returns [`Sb04MdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sb04md_case(root: impl AsRef<Path>) -> Result<Sb04MdCase, Sb04MdExampleError> {
    let root = root.as_ref();
    let input = parse_sb04md_input_file(root.join("data/SB04MD.dat"))?;
    let output = parse_sb04md_result_file(root.join("results/SB04MD.res"), input.n, input.m)?;
    Ok(Sb04MdCase { input, output })
}

/// Parses the upstream `SB04MD.dat` file.
///
/// # Errors
///
/// Returns [`Sb04MdExampleError`] if the file cannot be read or parsed.
pub fn parse_sb04md_input_file(path: impl AsRef<Path>) -> Result<Sb04MdInput, Sb04MdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb04MdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sb04MdExampleError::UnexpectedEnd { field: "header" })?;
    let mut header_tokens = header.split_whitespace();
    let left_order = parse_next_usize(&mut header_tokens, "n")?;
    let right_order = parse_next_usize(&mut header_tokens, "m")?;

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut tokens = body.split_whitespace();
    let left_matrix = read_row_major_matrix(&mut tokens, left_order, left_order, "A")?;
    let right_matrix = read_row_major_matrix(&mut tokens, right_order, right_order, "B")?;
    let rhs_matrix = read_row_major_matrix(&mut tokens, left_order, right_order, "C")?;

    Ok(Sb04MdInput {
        n: left_order,
        m: right_order,
        a: left_matrix,
        b: right_matrix,
        c: rhs_matrix,
    })
}

/// Parses the checked-in `SB04MD.res` file.
///
/// # Errors
///
/// Returns [`Sb04MdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sb04md_result_file(
    path: impl AsRef<Path>,
    n: usize,
    m: usize,
) -> Result<Sb04MdOutput, Sb04MdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb04MdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let x_index = find_line(&lines, "The solution matrix X is")?;
    let x = read_matrix_rows(&lines, x_index + 1, n, "solution matrix")?;

    let z_index = find_line(&lines, "The orthogonal matrix Z is")?;
    let z = read_matrix_rows(&lines, z_index + 1, m, "orthogonal matrix Z")?;

    Ok(Sb04MdOutput { x, z })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sb04MdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sb04MdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sb04MdExampleError> {
    tokens
        .next()
        .ok_or(Sb04MdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sb04MdExampleError> {
    let token = next_token(tokens, field)?;
    token
        .parse::<usize>()
        .map_err(|source| Sb04MdExampleError::ParseInt {
            field,
            token: token.to_owned(),
            source,
        })
}

fn parse_next_f64<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<f64, Sb04MdExampleError> {
    let token = next_token(tokens, field)?;
    token
        .parse::<f64>()
        .map_err(|source| Sb04MdExampleError::ParseFloat {
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
) -> Result<Vec<Vec<f64>>, Sb04MdExampleError> {
    let mut matrix = vec![vec![0.0; columns]; rows];
    for row in &mut matrix {
        for value in row {
            *value = parse_next_f64(tokens, field)?;
        }
    }
    Ok(matrix)
}

fn read_matrix_rows(
    lines: &[&str],
    start: usize,
    row_count: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Sb04MdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sb04MdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sb04MdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Sb04MdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}
