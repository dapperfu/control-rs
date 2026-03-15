//! Parsers for the upstream `SB03TD` (continuous Lyapunov) example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SB03TD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03TdCase {
    pub input: Sb03TdInput,
    pub output: Sb03TdOutput,
}

/// Parsed `SB03TD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03TdInput {
    pub n: usize,
    pub a: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
}

/// Parsed `SB03TD` output data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03TdOutput {
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Errors produced while parsing the upstream `SB03TD` assets.
#[derive(Debug, Error)]
pub enum Sb03TdExampleError {
    #[error("failed to read SB03TD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SB03TD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SB03TD data while parsing {field}")]
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

/// Loads the checked-in upstream `SB03TD` example from `root`.
///
/// # Errors
///
/// Returns [`Sb03TdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sb03td_case(root: impl AsRef<Path>) -> Result<Sb03TdCase, Sb03TdExampleError> {
    let root = root.as_ref();
    let input = parse_sb03td_input_file(root.join("data/SB03TD.dat"))?;
    let output = parse_sb03td_result_file(root.join("results/SB03TD.res"), input.n)?;
    Ok(Sb03TdCase { input, output })
}

/// Parses the upstream `SB03TD.dat` file.
///
/// # Errors
///
/// Returns [`Sb03TdExampleError`] if the file cannot be read or parsed.
pub fn parse_sb03td_input_file(path: impl AsRef<Path>) -> Result<Sb03TdInput, Sb03TdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb03TdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sb03TdExampleError::UnexpectedEnd { field: "header" })?;
    let mut header_tokens = header.split_whitespace();

    let order = parse_next_usize(&mut header_tokens, "n")?;
    for _ in 0..5 {
        let _ = header_tokens.next();
    }

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut tokens, order, order, "A")?;
    let c = read_row_major_matrix(&mut tokens, order, order, "C")?;

    Ok(Sb03TdInput { n: order, a, c })
}

/// Parses the checked-in `SB03TD.res` file.
///
/// # Errors
///
/// Returns [`Sb03TdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sb03td_result_file(
    path: impl AsRef<Path>,
    order: usize,
) -> Result<Sb03TdOutput, Sb03TdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb03TdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let solution_index = find_line(&lines, "The solution matrix X is")?;
    let x = read_matrix_rows(&lines, solution_index + 1, order, "solution matrix")?;
    let scale_index = find_line(&lines, "Scaling factor =")?;
    let scale = parse_f64_after_equals(lines[scale_index], "scaling factor")?;

    Ok(Sb03TdOutput { x, scale })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sb03TdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sb03TdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sb03TdExampleError> {
    tokens
        .next()
        .ok_or(Sb03TdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sb03TdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Sb03TdExampleError::ParseInt {
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
) -> Result<Vec<Vec<f64>>, Sb03TdExampleError> {
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
) -> Result<f64, Sb03TdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<f64>().map_err(|source| Sb03TdExampleError::ParseFloat {
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
) -> Result<Vec<Vec<f64>>, Sb03TdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sb03TdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sb03TdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Sb03TdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}

fn parse_f64_after_equals(line: &str, field: &'static str) -> Result<f64, Sb03TdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Sb03TdExampleError::MissingSection { section: field })?
        .trim();
    token.parse::<f64>().map_err(|source| Sb03TdExampleError::ParseFloat {
        field,
        token: token.to_owned(),
        source,
    })
}
