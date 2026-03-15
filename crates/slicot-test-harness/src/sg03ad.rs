//! Parsers for the upstream `SG03AD` example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SG03AD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03AdCase {
    pub input: Sg03AdInput,
    pub output: Sg03AdOutput,
}

/// Parsed `SG03AD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03AdInput {
    pub n: usize,
    pub job: char,
    pub dico: char,
    pub fact: char,
    pub trans: char,
    pub uplo: char,
    pub a: Vec<Vec<f64>>,
    pub e: Vec<Vec<f64>>,
    pub y: Vec<Vec<f64>>,
}

/// Parsed `SG03AD` output data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03AdOutput {
    pub sep: f64,
    pub ferr: f64,
    pub scale: f64,
    pub x: Vec<Vec<f64>>,
}

/// Errors produced while parsing the upstream `SG03AD` assets.
#[derive(Debug, Error)]
pub enum Sg03AdExampleError {
    #[error("failed to read SG03AD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SG03AD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SG03AD data while parsing {field}")]
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
    #[error("invalid SG03AD mode flag: {value}")]
    InvalidModeFlag { value: String },
}

/// Loads the checked-in upstream `SG03AD` example from `root`.
///
/// # Errors
///
/// Returns [`Sg03AdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sg03ad_case(root: impl AsRef<Path>) -> Result<Sg03AdCase, Sg03AdExampleError> {
    let root = root.as_ref();
    let input = parse_sg03ad_input_file(root.join("data/SG03AD.dat"))?;
    let output = parse_sg03ad_result_file(root.join("results/SG03AD.res"), input.n)?;
    Ok(Sg03AdCase { input, output })
}

/// Parses the upstream `SG03AD.dat` file.
///
/// # Errors
///
/// Returns [`Sg03AdExampleError`] if the file cannot be read or parsed.
pub fn parse_sg03ad_input_file(
    path: impl AsRef<Path>,
) -> Result<Sg03AdInput, Sg03AdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sg03AdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sg03AdExampleError::UnexpectedEnd { field: "header" })?;
    let mut header_tokens = header.split_whitespace();
    let order = parse_next_usize(&mut header_tokens, "n")?;
    let job = parse_mode_flag(next_token(&mut header_tokens, "job")?)?;
    let dico = parse_mode_flag(next_token(&mut header_tokens, "dico")?)?;
    let fact = parse_mode_flag(next_token(&mut header_tokens, "fact")?)?;
    let trans = parse_mode_flag(next_token(&mut header_tokens, "trans")?)?;
    let uplo = parse_mode_flag(next_token(&mut header_tokens, "uplo")?)?;

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut tokens, order, order, "A")?;
    let e = read_row_major_matrix(&mut tokens, order, order, "E")?;
    let raw_y = read_row_major_matrix(&mut tokens, order, order, "Y")?;
    let y = symmetrize_triangular_matrix(&raw_y, uplo);

    Ok(Sg03AdInput {
        n: order,
        job,
        dico,
        fact,
        trans,
        uplo,
        a,
        e,
        y,
    })
}

/// Parses the checked-in `SG03AD.res` file.
///
/// # Errors
///
/// Returns [`Sg03AdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sg03ad_result_file(
    path: impl AsRef<Path>,
    order: usize,
) -> Result<Sg03AdOutput, Sg03AdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sg03AdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let sep_index = find_line(&lines, "SEP =")?;
    let ferr_index = find_line(&lines, "FERR =")?;
    let scale_index = find_line(&lines, "SCALE =")?;
    let x_index = find_line(&lines, "The solution matrix X is")?;

    let sep = parse_scientific_after_equals(lines[sep_index], "sep")?;
    let ferr = parse_scientific_after_equals(lines[ferr_index], "ferr")?;
    let scale = parse_scientific_after_equals(lines[scale_index], "scale")?;
    let x = read_matrix_rows(&lines, x_index + 1, order, "solution matrix")?;

    Ok(Sg03AdOutput { sep, ferr, scale, x })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sg03AdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sg03AdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sg03AdExampleError> {
    tokens
        .next()
        .ok_or(Sg03AdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sg03AdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Sg03AdExampleError::ParseInt {
        field,
        token: token.to_owned(),
        source,
    })
}

fn parse_next_f64<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<f64, Sg03AdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<f64>().map_err(|source| Sg03AdExampleError::ParseFloat {
        field,
        token: token.to_owned(),
        source,
    })
}

fn parse_mode_flag(token: &str) -> Result<char, Sg03AdExampleError> {
    let mut chars = token.chars();
    match chars.next() {
        Some(value) if chars.next().is_none() => Ok(value),
        _ => Err(Sg03AdExampleError::InvalidModeFlag {
            value: token.to_owned(),
        }),
    }
}

fn read_row_major_matrix<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    rows: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Sg03AdExampleError> {
    let mut matrix = vec![vec![0.0; columns]; rows];
    for row in &mut matrix {
        for value in row {
            *value = parse_next_f64(tokens, field)?;
        }
    }
    Ok(matrix)
}

fn symmetrize_triangular_matrix(matrix: &[Vec<f64>], uplo: char) -> Vec<Vec<f64>> {
    let order = matrix.len();
    let mut symmetric = vec![vec![0.0; order]; order];

    for row_index in 0..order {
        for column_index in 0..order {
            symmetric[row_index][column_index] = if matches!(uplo, 'U') {
                if row_index <= column_index {
                    matrix[row_index][column_index]
                } else {
                    matrix[column_index][row_index]
                }
            } else if row_index >= column_index {
                matrix[row_index][column_index]
            } else {
                matrix[column_index][row_index]
            };
        }
    }

    symmetric
}

fn parse_scientific_after_equals(line: &str, field: &'static str) -> Result<f64, Sg03AdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Sg03AdExampleError::MissingSection { section: field })?
        .trim()
        .replace('D', "E");
    token.parse::<f64>().map_err(|source| Sg03AdExampleError::ParseFloat {
        field,
        token,
        source,
    })
}

fn read_matrix_rows(
    lines: &[&str],
    start: usize,
    row_count: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Sg03AdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sg03AdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sg03AdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Sg03AdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}
