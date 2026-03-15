//! Parsers for the upstream `SG03BD` (generalized Lyapunov Cholesky factor) example assets.
//!
//! SG03BD computes the Cholesky factor U of the solution X of the generalized
//! Lyapunov equation A' X E + E' X A = -scale² B' B. The fixture can be validated
//! by solving via SG03AD with Y = -B'B then taking the upper Cholesky factor of X.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SG03BD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03BdCase {
    pub input: Sg03BdInput,
    pub output: Sg03BdOutput,
}

/// Parsed `SG03BD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03BdInput {
    pub n: usize,
    pub m: usize,
    pub dico: char,
    pub fact: char,
    pub trans: char,
    pub a: Vec<Vec<f64>>,
    pub e: Vec<Vec<f64>>,
    /// B is M×N (rows × columns).
    pub b: Vec<Vec<f64>>,
}

/// Parsed `SG03BD` output data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sg03BdOutput {
    pub scale: f64,
    /// Upper Cholesky factor U such that X = U' U.
    pub u: Vec<Vec<f64>>,
}

/// Errors produced while parsing the upstream `SG03BD` assets.
#[derive(Debug, Error)]
pub enum Sg03BdExampleError {
    #[error("failed to read SG03BD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SG03BD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SG03BD data while parsing {field}")]
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

/// Loads the checked-in upstream `SG03BD` example from `root`.
///
/// # Errors
///
/// Returns [`Sg03BdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sg03bd_case(root: impl AsRef<Path>) -> Result<Sg03BdCase, Sg03BdExampleError> {
    let root = root.as_ref();
    let input = parse_sg03bd_input_file(root.join("data/SG03BD.dat"))?;
    let output = parse_sg03bd_result_file(root.join("results/SG03BD.res"), input.n)?;
    Ok(Sg03BdCase { input, output })
}

/// Parses the upstream `SG03BD.dat` file.
///
/// # Errors
///
/// Returns [`Sg03BdExampleError`] if the file cannot be read or parsed.
pub fn parse_sg03bd_input_file(path: impl AsRef<Path>) -> Result<Sg03BdInput, Sg03BdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sg03BdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sg03BdExampleError::UnexpectedEnd { field: "header" })?;
    let mut header_tokens = header.split_whitespace();
    let n = parse_next_usize(&mut header_tokens, "n")?;
    let m = parse_next_usize(&mut header_tokens, "m")?;
    let dico = next_token(&mut header_tokens, "dico")?.chars().next().unwrap_or('C');
    let fact = next_token(&mut header_tokens, "fact")?.chars().next().unwrap_or('N');
    let trans = next_token(&mut header_tokens, "trans")?.chars().next().unwrap_or('N');

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut tokens, n, n, "A")?;
    let e = read_row_major_matrix(&mut tokens, n, n, "E")?;
    let b = read_row_major_matrix(&mut tokens, m, n, "B")?;

    Ok(Sg03BdInput {
        n,
        m,
        dico,
        fact,
        trans,
        a,
        e,
        b,
    })
}

/// Parses the checked-in `SG03BD.res` file.
///
/// # Errors
///
/// Returns [`Sg03BdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sg03bd_result_file(
    path: impl AsRef<Path>,
    order: usize,
) -> Result<Sg03BdOutput, Sg03BdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sg03BdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let scale_index = find_line(&lines, "SCALE =")?;
    let scale = parse_f64_after_equals(lines[scale_index], "SCALE")?;
    let u_index = find_line(&lines, "The Cholesky factor U of the solution matrix is")?;
    let u = read_matrix_rows(&lines, u_index + 1, order, "Cholesky factor U")?;

    Ok(Sg03BdOutput { scale, u })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sg03BdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sg03BdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sg03BdExampleError> {
    tokens
        .next()
        .ok_or(Sg03BdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sg03BdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Sg03BdExampleError::ParseInt {
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
) -> Result<Vec<Vec<f64>>, Sg03BdExampleError> {
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
) -> Result<f64, Sg03BdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<f64>().map_err(|source| Sg03BdExampleError::ParseFloat {
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
) -> Result<Vec<Vec<f64>>, Sg03BdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sg03BdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sg03BdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Sg03BdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}

fn parse_f64_after_equals(line: &str, field: &'static str) -> Result<f64, Sg03BdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Sg03BdExampleError::MissingSection { section: field })?
        .trim();
    token.parse::<f64>().map_err(|source| Sg03BdExampleError::ParseFloat {
        field,
        token: token.to_owned(),
        source,
    })
}
