//! Parsers for the upstream `AB13BD` example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `AB13BD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Ab13BdCase {
    pub input: Ab13BdInput,
    pub output: Ab13BdOutput,
}

/// Parsed `AB13BD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Ab13BdInput {
    pub n: usize,
    pub m: usize,
    pub p: usize,
    pub tol: f64,
    pub dico: char,
    pub jobn: char,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
}

/// Parsed `AB13BD` output data.
#[derive(Clone, Debug, PartialEq)]
pub struct Ab13BdOutput {
    pub norm: f64,
}

/// Errors produced while parsing the upstream `AB13BD` assets.
#[derive(Debug, Error)]
pub enum Ab13BdExampleError {
    #[error("failed to read AB13BD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing AB13BD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of AB13BD data while parsing {field}")]
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
    #[error("invalid AB13BD mode flag: {value}")]
    InvalidModeFlag { value: String },
}

/// Loads the checked-in upstream `AB13BD` example from `root`.
///
/// # Errors
///
/// Returns [`Ab13BdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_ab13bd_case(root: impl AsRef<Path>) -> Result<Ab13BdCase, Ab13BdExampleError> {
    let root = root.as_ref();
    let input = parse_ab13bd_input_file(root.join("data/AB13BD.dat"))?;
    let output = parse_ab13bd_result_file(root.join("results/AB13BD.res"))?;
    Ok(Ab13BdCase { input, output })
}

/// Parses the upstream `AB13BD.dat` file.
///
/// # Errors
///
/// Returns [`Ab13BdExampleError`] if the file cannot be read or parsed.
pub fn parse_ab13bd_input_file(
    path: impl AsRef<Path>,
) -> Result<Ab13BdInput, Ab13BdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Ab13BdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Ab13BdExampleError::UnexpectedEnd { field: "header" })?;
    let mut tokens = header.split_whitespace();
    let n = parse_next_usize(&mut tokens, "n")?;
    let m = parse_next_usize(&mut tokens, "m")?;
    let p = parse_next_usize(&mut tokens, "p")?;
    let tol = parse_next_f64(&mut tokens, "tol")?;
    let dico = parse_mode_flag(next_token(&mut tokens, "dico")?)?;
    let jobn = parse_mode_flag(next_token(&mut tokens, "jobn")?)?;

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut body_tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut body_tokens, n, n, "A")?;
    let b = read_row_major_matrix(&mut body_tokens, n, m, "B")?;
    let c = read_row_major_matrix(&mut body_tokens, p, n, "C")?;
    let d = read_row_major_matrix(&mut body_tokens, p, m, "D")?;

    Ok(Ab13BdInput {
        n,
        m,
        p,
        tol,
        dico,
        jobn,
        a,
        b,
        c,
        d,
    })
}

/// Parses the checked-in `AB13BD.res` file.
///
/// # Errors
///
/// Returns [`Ab13BdExampleError`] if the result file cannot be read or parsed.
pub fn parse_ab13bd_result_file(path: impl AsRef<Path>) -> Result<Ab13BdOutput, Ab13BdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Ab13BdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let norm_line = lines
        .iter()
        .find(|line| line.contains("L2-norm") || line.contains("H2-norm"))
        .ok_or(Ab13BdExampleError::MissingSection { section: "norm" })?;
    let norm = parse_norm_from_line(norm_line)?;

    Ok(Ab13BdOutput { norm })
}

fn parse_norm_from_line(line: &str) -> Result<f64, Ab13BdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Ab13BdExampleError::MissingSection { section: "norm value" })?
        .trim()
        .replace('D', "E");
    token.parse::<f64>().map_err(|source| Ab13BdExampleError::ParseFloat {
        field: "norm",
        token,
        source,
    })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Ab13BdExampleError> {
    tokens
        .next()
        .ok_or(Ab13BdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Ab13BdExampleError> {
    let token = next_token(tokens, field)?;
    token.parse::<usize>().map_err(|source| Ab13BdExampleError::ParseInt {
        field,
        token: token.to_owned(),
        source,
    })
}

fn parse_next_f64<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<f64, Ab13BdExampleError> {
    let token = next_token(tokens, field)?;
    let normalized = token.replace('D', "E").replace('d', "e");
    normalized.parse::<f64>().map_err(|source| Ab13BdExampleError::ParseFloat {
        field,
        token: token.to_owned(),
        source,
    })
}

fn parse_mode_flag(token: &str) -> Result<char, Ab13BdExampleError> {
    let mut chars = token.chars();
    match chars.next() {
        Some(value) if chars.next().is_none() => Ok(value),
        _ => Err(Ab13BdExampleError::InvalidModeFlag {
            value: token.to_owned(),
        }),
    }
}

fn read_row_major_matrix<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    rows: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Ab13BdExampleError> {
    let mut matrix = vec![vec![0.0; columns]; rows];
    for row in &mut matrix {
        for value in row {
            *value = parse_next_f64(tokens, field)?;
        }
    }
    Ok(matrix)
}
