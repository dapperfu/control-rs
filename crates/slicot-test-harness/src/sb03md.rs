//! Parsers for the upstream `SB03MD` example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `SB03MD` input and output fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03MdCase {
    pub input: Sb03MdInput,
    pub output: Sb03MdOutput,
}

/// Parsed `SB03MD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03MdInput {
    pub n: usize,
    pub dico: char,
    pub fact: char,
    pub job: char,
    pub trana: char,
    pub a: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
}

/// Parsed `SB03MD` output data.
#[derive(Clone, Debug, PartialEq)]
pub struct Sb03MdOutput {
    pub x: Vec<Vec<f64>>,
    pub scale: f64,
}

/// Errors produced while parsing the upstream `SB03MD` assets.
#[derive(Debug, Error)]
pub enum Sb03MdExampleError {
    #[error("failed to read SB03MD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing SB03MD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of SB03MD data while parsing {field}")]
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
    #[error("invalid SB03MD mode flag: {value}")]
    InvalidModeFlag { value: String },
}

/// Loads the checked-in upstream `SB03MD` example from `root`.
///
/// # Errors
///
/// Returns [`Sb03MdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_sb03md_case(root: impl AsRef<Path>) -> Result<Sb03MdCase, Sb03MdExampleError> {
    let root = root.as_ref();
    let input = parse_sb03md_input_file(root.join("data/SB03MD.dat"))?;
    let output = parse_sb03md_result_file(root.join("results/SB03MD.res"), input.n)?;
    Ok(Sb03MdCase { input, output })
}

/// Parses the upstream `SB03MD.dat` file.
///
/// # Errors
///
/// Returns [`Sb03MdExampleError`] if the file cannot be read or parsed.
pub fn parse_sb03md_input_file(path: impl AsRef<Path>) -> Result<Sb03MdInput, Sb03MdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb03MdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Sb03MdExampleError::UnexpectedEnd { field: "header" })?;
    let mut header_tokens = header.split_whitespace();

    let n = parse_next_usize(&mut header_tokens, "n")?;
    let dico = parse_mode_flag(next_token(&mut header_tokens, "dico")?)?;
    let fact = parse_mode_flag(next_token(&mut header_tokens, "fact")?)?;
    let job = parse_mode_flag(next_token(&mut header_tokens, "job")?)?;
    let trana = parse_mode_flag(next_token(&mut header_tokens, "trana")?)?;

    let body = lines.collect::<Vec<_>>().join(" ");
    let mut tokens = body.split_whitespace();
    let a = read_row_major_matrix(&mut tokens, n, n, "A")?;
    let c = read_row_major_matrix(&mut tokens, n, n, "C")?;

    Ok(Sb03MdInput {
        n,
        dico,
        fact,
        job,
        trana,
        a,
        c,
    })
}

/// Parses the checked-in `SB03MD.res` file.
///
/// # Errors
///
/// Returns [`Sb03MdExampleError`] if the result file cannot be read or parsed.
pub fn parse_sb03md_result_file(
    path: impl AsRef<Path>,
    order: usize,
) -> Result<Sb03MdOutput, Sb03MdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Sb03MdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let solution_index = find_line(&lines, "The solution matrix X is")?;
    let x = read_matrix_rows(&lines, solution_index + 1, order, "solution matrix")?;
    let scale_index = find_line(&lines, "Scaling factor =")?;
    let scale = parse_f64_after_equals(lines[scale_index], "scaling factor")?;

    Ok(Sb03MdOutput { x, scale })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Sb03MdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Sb03MdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Sb03MdExampleError> {
    tokens
        .next()
        .ok_or(Sb03MdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Sb03MdExampleError> {
    let token = next_token(tokens, field)?;
    token
        .parse::<usize>()
        .map_err(|source| Sb03MdExampleError::ParseInt {
            field,
            token: token.to_owned(),
            source,
        })
}

fn parse_next_f64<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<f64, Sb03MdExampleError> {
    let token = next_token(tokens, field)?;
    token
        .parse::<f64>()
        .map_err(|source| Sb03MdExampleError::ParseFloat {
            field,
            token: token.to_owned(),
            source,
        })
}

fn parse_mode_flag(token: &str) -> Result<char, Sb03MdExampleError> {
    let mut chars = token.chars();
    match chars.next() {
        Some(value) if chars.next().is_none() => Ok(value),
        _ => Err(Sb03MdExampleError::InvalidModeFlag {
            value: token.to_owned(),
        }),
    }
}

fn read_row_major_matrix<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    rows: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Sb03MdExampleError> {
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
) -> Result<Vec<Vec<f64>>, Sb03MdExampleError> {
    let mut matrix = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Sb03MdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_row(line, field)?);
    }
    Ok(matrix)
}

fn parse_f64_row(line: &str, field: &'static str) -> Result<Vec<f64>, Sb03MdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Sb03MdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}

fn parse_f64_after_equals(line: &str, field: &'static str) -> Result<f64, Sb03MdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Sb03MdExampleError::MissingSection { section: field })?
        .trim();
    token
        .parse::<f64>()
        .map_err(|source| Sb03MdExampleError::ParseFloat {
            field,
            token: token.to_owned(),
            source,
        })
}
