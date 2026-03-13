//! Parsers for the upstream `TB05AD` example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use num_complex::Complex64;
use thiserror::Error;

/// Parsed `TB05AD` example inputs and expected outputs.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb05AdCase {
    pub input: Tb05AdInput,
    pub output: Tb05AdOutput,
}

/// Parsed `TB05AD` input data.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb05AdInput {
    pub n: usize,
    pub m: usize,
    pub p: usize,
    pub freq: Complex64,
    pub inita: char,
    pub baleig: char,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
}

/// Parsed `TB05AD` result data.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb05AdOutput {
    pub rcond: Option<f64>,
    pub eigenvalues: Vec<Complex64>,
    pub g: Vec<Vec<Complex64>>,
    pub hinvb: Vec<Vec<Complex64>>,
}

/// Errors produced while parsing the upstream `TB05AD` assets.
#[derive(Debug, Error)]
pub enum Tb05AdExampleError {
    #[error("failed to read TB05AD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing TB05AD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of TB05AD data while parsing {field}")]
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
    #[error("invalid TB05AD mode flag: {value}")]
    InvalidModeFlag { value: String },
    #[error("invalid TB05AD complex token: {token}")]
    InvalidComplexToken { token: String },
}

/// Loads the checked-in upstream `TB05AD` example from `root`.
///
/// # Errors
///
/// Returns [`Tb05AdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_tb05ad_case(root: impl AsRef<Path>) -> Result<Tb05AdCase, Tb05AdExampleError> {
    let root = root.as_ref();
    let input = parse_tb05ad_input_file(root.join("data/TB05AD.dat"))?;
    let output = parse_tb05ad_result_file(root.join("results/TB05AD.res"), &input)?;
    Ok(Tb05AdCase { input, output })
}

/// Parses the upstream `TB05AD.dat` file.
///
/// # Errors
///
/// Returns [`Tb05AdExampleError`] if the file cannot be read or the token
/// stream cannot be converted into the expected matrices.
pub fn parse_tb05ad_input_file(path: impl AsRef<Path>) -> Result<Tb05AdInput, Tb05AdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Tb05AdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let header = lines
        .next()
        .ok_or(Tb05AdExampleError::UnexpectedEnd { field: "header" })?;
    let (state_count, input_count, output_count, freq, inita, baleig) =
        parse_tb05ad_header(header)?;
    let body = lines.collect::<Vec<_>>().join(" ");
    let mut tokens = body.split_whitespace();

    let a = read_row_major_matrix(&mut tokens, state_count, state_count, "A")?;
    let b = read_row_major_matrix(&mut tokens, state_count, input_count, "B")?;
    let c = read_row_major_matrix(&mut tokens, output_count, state_count, "C")?;

    Ok(Tb05AdInput {
        n: state_count,
        m: input_count,
        p: output_count,
        freq,
        inita,
        baleig,
        a,
        b,
        c,
    })
}

/// Parses the checked-in `TB05AD.res` file.
///
/// # Errors
///
/// Returns [`Tb05AdExampleError`] if the result file cannot be read or if the
/// expected sections cannot be located and parsed.
pub fn parse_tb05ad_result_file(
    path: impl AsRef<Path>,
    input: &Tb05AdInput,
) -> Result<Tb05AdOutput, Tb05AdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Tb05AdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let rcond = find_line(&lines, "RCOND =")
        .ok()
        .map(|index| parse_f64_after_equals(lines[index], "rcond"))
        .transpose()?;

    let eigenvalues = if let Ok(index) =
        find_line(&lines, "Eigenvalues of the state transmission matrix A are")
    {
        read_complex_pairs(&lines, index + 1, input.n, "eigenvalues")?
    } else {
        Vec::new()
    };

    let g_index = find_line(&lines, "The frequency response matrix G(freq) is")?;
    let g = read_complex_matrix_rows(&lines, g_index + 1, input.p, "G(freq)")?;

    let hinvb_index = find_line(&lines, "H(inverse)*B is")?;
    let hinvb = read_complex_matrix_rows(&lines, hinvb_index + 1, input.n, "H(inverse)*B")?;

    Ok(Tb05AdOutput {
        rcond,
        eigenvalues,
        g,
        hinvb,
    })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Tb05AdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Tb05AdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Tb05AdExampleError> {
    tokens
        .next()
        .ok_or(Tb05AdExampleError::UnexpectedEnd { field })
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Tb05AdExampleError> {
    let token = next_token(tokens, field)?;
    token
        .parse::<usize>()
        .map_err(|source| Tb05AdExampleError::ParseInt {
            field,
            token: token.to_owned(),
            source,
        })
}

fn parse_complex_token(token: &str) -> Result<Complex64, Tb05AdExampleError> {
    let trimmed = token.trim();
    let Some(without_prefix) = trimmed.strip_prefix('(') else {
        return Err(Tb05AdExampleError::InvalidComplexToken {
            token: token.to_owned(),
        });
    };
    let Some(without_suffix) = without_prefix.strip_suffix(')') else {
        return Err(Tb05AdExampleError::InvalidComplexToken {
            token: token.to_owned(),
        });
    };
    let Some((real_text, imag_text)) = without_suffix.split_once(',') else {
        return Err(Tb05AdExampleError::InvalidComplexToken {
            token: token.to_owned(),
        });
    };
    let real =
        real_text
            .trim()
            .parse::<f64>()
            .map_err(|source| Tb05AdExampleError::ParseFloat {
                field: "complex real part",
                token: real_text.trim().to_owned(),
                source,
            })?;
    let imaginary =
        imag_text
            .trim()
            .parse::<f64>()
            .map_err(|source| Tb05AdExampleError::ParseFloat {
                field: "complex imaginary part",
                token: imag_text.trim().to_owned(),
                source,
            })?;
    Ok(Complex64::new(real, imaginary))
}

fn parse_tb05ad_header(
    header: &str,
) -> Result<(usize, usize, usize, Complex64, char, char), Tb05AdExampleError> {
    let Some((prefix, remainder)) = header.split_once('(') else {
        return Err(Tb05AdExampleError::InvalidComplexToken {
            token: header.to_owned(),
        });
    };
    let Some((complex_body, suffix)) = remainder.split_once(')') else {
        return Err(Tb05AdExampleError::InvalidComplexToken {
            token: header.to_owned(),
        });
    };

    let mut prefix_tokens = prefix.split_whitespace();
    let state_count = parse_next_usize(&mut prefix_tokens, "n")?;
    let input_count = parse_next_usize(&mut prefix_tokens, "m")?;
    let output_count = parse_next_usize(&mut prefix_tokens, "p")?;
    let freq = parse_complex_token(&format!("({complex_body})"))?;

    let mut suffix_tokens = suffix.split_whitespace();
    let inita = parse_mode_flag(next_token(&mut suffix_tokens, "inita")?)?;
    let baleig = parse_mode_flag(next_token(&mut suffix_tokens, "baleig")?)?;

    Ok((state_count, input_count, output_count, freq, inita, baleig))
}

fn parse_mode_flag(token: &str) -> Result<char, Tb05AdExampleError> {
    let mut chars = token.chars();
    match chars.next() {
        Some(value) if chars.next().is_none() => Ok(value),
        _ => Err(Tb05AdExampleError::InvalidModeFlag {
            value: token.to_owned(),
        }),
    }
}

fn read_row_major_matrix<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    rows: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Tb05AdExampleError> {
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
) -> Result<f64, Tb05AdExampleError> {
    let token = next_token(tokens, field)?;
    token
        .parse::<f64>()
        .map_err(|source| Tb05AdExampleError::ParseFloat {
            field,
            token: token.to_owned(),
            source,
        })
}

fn parse_f64_after_equals(line: &str, field: &'static str) -> Result<f64, Tb05AdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Tb05AdExampleError::MissingSection { section: field })?
        .trim();
    token
        .parse::<f64>()
        .map_err(|source| Tb05AdExampleError::ParseFloat {
            field,
            token: token.to_owned(),
            source,
        })
}

fn read_complex_pairs(
    lines: &[&str],
    start: usize,
    row_count: usize,
    field: &'static str,
) -> Result<Vec<Complex64>, Tb05AdExampleError> {
    let mut values = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Tb05AdExampleError::UnexpectedEnd { field })?;
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        if tokens.len() < 2 {
            return Err(Tb05AdExampleError::UnexpectedEnd { field });
        }
        let real = tokens[0]
            .parse::<f64>()
            .map_err(|source| Tb05AdExampleError::ParseFloat {
                field,
                token: tokens[0].to_owned(),
                source,
            })?;
        let imaginary_token = tokens[1].trim_end_matches("*j");
        let imaginary =
            imaginary_token
                .parse::<f64>()
                .map_err(|source| Tb05AdExampleError::ParseFloat {
                    field,
                    token: imaginary_token.to_owned(),
                    source,
                })?;
        values.push(Complex64::new(real, imaginary));
    }
    Ok(values)
}

fn read_complex_matrix_rows(
    lines: &[&str],
    start: usize,
    row_count: usize,
    field: &'static str,
) -> Result<Vec<Vec<Complex64>>, Tb05AdExampleError> {
    let mut values = Vec::with_capacity(row_count);
    for offset in 0..row_count {
        let line = lines
            .get(start + offset)
            .ok_or(Tb05AdExampleError::UnexpectedEnd { field })?;
        values.push(parse_complex_row(line, field)?);
    }
    Ok(values)
}

fn parse_complex_row(
    line: &str,
    field: &'static str,
) -> Result<Vec<Complex64>, Tb05AdExampleError> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut collecting = false;

    for character in line.chars() {
        if character == '(' {
            collecting = true;
            current.clear();
            current.push(character);
            continue;
        }
        if collecting {
            current.push(character);
            if character == ')' {
                values.push(parse_complex_token(&current)?);
                collecting = false;
            }
        }
    }

    if values.is_empty() {
        return Err(Tb05AdExampleError::MissingSection { section: field });
    }
    Ok(values)
}
