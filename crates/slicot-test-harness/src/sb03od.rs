//! Parsers for the upstream `SB03OD` example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SB03OD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03OdCase {
    pub input: Sb03OdInput,
    pub output: Sb03OdOutput,
}

/// Parsed `SB03OD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03OdInput {
    pub n: usize,
    pub m: usize,
    pub dico: char,
    pub fact: char,
    pub trans: char,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
}

/// Parsed `SB03OD` output data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03OdOutput {
    pub u_transpose: Vec<Vec<f64>>,
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Errors produced while parsing the upstream `SB03OD` assets.
#[derive(Debug, Error)]
pub enum Sb03OdExampleError {
    #[error("failed to read SB03OD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SB03OD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SB03OD data while parsing {field}")]
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
    #[error("invalid SB03OD mode flag: {value}")]
    InvalidModeFlag { value: String },
}

/// Loads the checked-in upstream `SB03OD` example from `root`.
///
/// # Errors
///
/// Returns [`Sb03OdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sb03od_case(root: impl AsRef<Path>) -> Result<Sb03OdCase, Sb03OdExampleError> {
    let root = root.as_ref();
    let input = parse_sb03od_input_file(root.join("data/SB03OD.dat"))?;
    let output = parse_sb03od_result_file(root.join("results/SB03OD.res"), input.n)?;
    Ok(Sb03OdCase { input, output })
}

/// Parses the upstream `SB03OD.dat` file.
///
/// # Errors
///
/// Returns [`Sb03OdExampleError`] if the file cannot be read or parsed.
pub fn parse_sb03od_input_file(
    path: impl AsRef<Path>,
) -> Result<Sb03OdInput, Sb03OdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb03OdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sb03OdExampleError::UnexpectedEnd { field: "header" })?;
    let mut tokens = header.split_whitespace();
    let n = parse_next_usize(&mut tokens, "n")?;
    let m = parse_next_usize(&mut tokens, "m")?;
    let dico = parse_mode_flag(next_token(&mut tokens, "dico")?)?;
    let fact = parse_mode_flag(next_token(&mut tokens, "fact")?)?;
    let trans = parse_mode_flag(next_token(&mut tokens, "trans")?)?;

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut body_tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut body_tokens, n, n, "A")?;
    let b = if matches!(trans, 'N') {
        read_row_major_matrix(&mut body_tokens, m, n, "B")?
    } else {
        read_row_major_matrix(&mut body_tokens, n, m, "B")?
    };

    Ok(Sb03OdInput {
        n,
        m,
        dico,
        fact,
        trans,
        a,
        b,
    })
}

/// Parses the checked-in `SB03OD.res` file.
///
/// # Errors
///
/// Returns [`Sb03OdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sb03od_result_file(
    path: impl AsRef<Path>,
    order: usize,
) -> Result<Sb03OdOutput, Sb03OdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb03OdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let u_index = find_line(&lines, "The transpose of the Cholesky factor U is")?;
    let x_index = find_line(&lines, "The solution matrix X = op(U)'*op(U) is")?;
    let scale_index = find_line(&lines, "Scaling factor =")?;

    let u_transpose = read_lower_triangular_rows(&lines, u_index + 1, order, "U transpose")?;
    let x = read_matrix_rows(&lines, x_index + 1, order, "solution matrix")?;
    let scale = parse_f64_after_equals(lines[scale_index], "scale")?;

    Ok(Sb03OdOutput {
        u_transpose,
        x,
        scale,
    })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sb03OdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sb03OdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sb03OdExampleError> {
    tokens
        .next()
        .ok_or(Sb03OdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sb03OdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Sb03OdExampleError::ParseInt {
        field,
        token: token.to_owned(),
        source,
    })
}

fn parse_mode_flag(token: &str) -> Result<char, Sb03OdExampleError> {
    let mut chars = token.chars();
    match chars.next() {
        Some(value) if chars.next().is_none() => Ok(value),
        _ => Err(Sb03OdExampleError::InvalidModeFlag {
            value: token.to_owned(),
        }),
    }
}

fn read_row_major_matrix<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    rows: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Sb03OdExampleError> {
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
) -> Result<f64, Sb03OdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<f64>().map_err(|source| Sb03OdExampleError::ParseFloat {
        field,
        token: token.to_owned(),
        source,
    })
}

fn read_lower_triangular_rows(
    lines: &[&str],
    start: usize,
    order: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Sb03OdExampleError> {
    let mut matrix = vec![vec![0.0; order]; order];
    for (row_index, row) in matrix.iter_mut().enumerate() {
        let line = lines
            .get(start + row_index)
            .ok_or(Sb03OdExampleError::UnexpectedEnd { field })?;
        let values = parse_f64_row(line, field)?;
        for (column_index, value) in values.into_iter().enumerate() {
            row[column_index] = value;
        }
    }
    Ok(matrix)
}

fn read_matrix_rows(
    lines: &[&str],
    start: usize,
    row_count: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Sb03OdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sb03OdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sb03OdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Sb03OdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}

fn parse_f64_after_equals(line: &str, field: &'static str) -> Result<f64, Sb03OdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Sb03OdExampleError::MissingSection { section: field })?
        .trim();
    token.parse::<f64>().map_err(|source| Sb03OdExampleError::ParseFloat {
        field,
        token: token.to_owned(),
        source,
    })
}
