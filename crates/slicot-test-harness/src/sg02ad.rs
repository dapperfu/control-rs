//! Parsers for the upstream `SG02AD` (generalized algebraic Riccati equation) example assets.
//!
//! SG02AD solves continuous or discrete generalized CARE/DARE. The routine subset
//! E = I, L = 0 (continuous) is implemented in `slicot-routines`; golden test runs when fixture matches.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SG02AD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg02AdCase {
    pub input: Sg02AdInput,
    pub output: Sg02AdOutput,
}

/// Parsed `SG02AD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg02AdInput {
    pub n: usize,
    pub m: usize,
    pub dico: char,
    pub a: Vec<Vec<f64>>,
    pub e: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub q: Vec<Vec<f64>>,
    pub r: Vec<Vec<f64>>,
    pub l: Vec<Vec<f64>>,
}

/// Parsed `SG02AD` output data (solution X only).
#[derive(Clone, Debug, PartialEq)]
pub struct Sg02AdOutput {
    pub x: Vec<Vec<f64>>,
}

/// Errors produced while parsing the upstream `SG02AD` assets.
#[derive(Debug, Error)]
pub enum Sg02AdExampleError {
    #[error("failed to read SG02AD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SG02AD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SG02AD data while parsing {field}")]
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
    #[error("invalid SG02AD mode flag: {value}")]
    InvalidModeFlag { value: String },
}

/// Loads the checked-in upstream `SG02AD` example from `root`.
///
/// # Errors
///
/// Returns [`Sg02AdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sg02ad_case(root: impl AsRef<Path>) -> Result<Sg02AdCase, Sg02AdExampleError> {
    let root = root.as_ref();
    let input = parse_sg02ad_input_file(root.join("data/SG02AD.dat"))?;
    let output = parse_sg02ad_result_file(root.join("results/SG02AD.res"), input.n)?;
    Ok(Sg02AdCase { input, output })
}

/// Parses the upstream `SG02AD.dat` file.
///
/// # Errors
///
/// Returns [`Sg02AdExampleError`] if the file cannot be read or parsed.
pub fn parse_sg02ad_input_file(
    path: impl AsRef<Path>,
) -> Result<Sg02AdInput, Sg02AdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sg02AdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sg02AdExampleError::UnexpectedEnd { field: "header" })?;
    let mut tokens = header.split_whitespace();
    let n = parse_next_usize(&mut tokens, "n")?;
    let m = parse_next_usize(&mut tokens, "m")?;
    let dico = parse_dico_from_header(header)?;

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut body_tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut body_tokens, n, n, "A")?;
    let e = read_row_major_matrix(&mut body_tokens, n, n, "E")?;
    let b = read_row_major_matrix(&mut body_tokens, n, m, "B")?;
    let q = read_row_major_matrix(&mut body_tokens, n, n, "Q")?;
    let r = read_row_major_matrix(&mut body_tokens, m, m, "R")?;
    let l = read_row_major_matrix(&mut body_tokens, n, m, "L")?;

    Ok(Sg02AdInput {
        n,
        m,
        dico,
        a,
        e,
        b,
        q,
        r,
        l,
    })
}

fn parse_dico_from_header(header: &str) -> Result<char, Sg02AdExampleError> {
    let tokens: Vec<&str> = header.split_whitespace().collect();
    if tokens.len() < 5 {
        return Err(Sg02AdExampleError::UnexpectedEnd { field: "dico" });
    }
    let dico_token = tokens[4];
    let mut ch = dico_token.chars();
    match ch.next() {
        Some(c) if ch.next().is_none() => Ok(c),
        _ => Err(Sg02AdExampleError::InvalidModeFlag {
            value: dico_token.to_owned(),
        }),
    }
}

/// Parses the checked-in `SG02AD.res` file.
///
/// # Errors
///
/// Returns [`Sg02AdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sg02ad_result_file(
    path: impl AsRef<Path>,
    n: usize,
) -> Result<Sg02AdOutput, Sg02AdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sg02AdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let x_index = find_line(&lines, "The solution matrix X is")?;
    let x = read_matrix_rows(&lines, x_index + 1, n, "solution matrix")?;

    Ok(Sg02AdOutput { x })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sg02AdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sg02AdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sg02AdExampleError> {
    tokens
        .next()
        .ok_or(Sg02AdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sg02AdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Sg02AdExampleError::ParseInt {
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
) -> Result<Vec<Vec<f64>>, Sg02AdExampleError> {
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
) -> Result<f64, Sg02AdExampleError> {
    let token = next_token(tokens, field)?;
    let normalized = token.replace('D', "E").replace('d', "e");
    normalized.parse::<f64>().map_err(|source| Sg02AdExampleError::ParseFloat {
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
) -> Result<Vec<Vec<f64>>, Sg02AdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sg02AdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sg02AdExampleError> {
    line.split_whitespace()
        .map(|token| {
            let normalized = token.replace('D', "E").replace('d', "e");
            normalized.parse::<f64>().map_err(|source| Sg02AdExampleError::ParseFloat {
                field,
                token: token.to_owned(),
                source,
            })
        })
        .collect()
}
