//! Parsers for the upstream `TB04AD` example assets.

use std::{
    fs,
    num::{ParseFloatError, ParseIntError},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Parsed `TB04AD` example inputs and expected outputs.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb04AdCase {
    pub input: Tb04AdInput,
    pub output: Tb04AdOutput,
}

/// Parsed `TB04AD` example program input.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb04AdInput {
    pub n: usize,
    pub m: usize,
    pub p: usize,
    pub tol1: f64,
    pub tol2: f64,
    pub rowcol: char,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub d: Vec<Vec<f64>>,
}

/// Parsed polynomial coefficient data for one transfer-matrix element.
#[derive(Clone, Debug, PartialEq)]
pub struct TransferPolynomial {
    pub row: usize,
    pub column: usize,
    pub numerator: Vec<f64>,
    pub denominator: Vec<f64>,
}

/// Parsed `TB04AD` example output.
#[derive(Clone, Debug, PartialEq)]
pub struct Tb04AdOutput {
    pub nr: usize,
    pub transformed_a: Vec<Vec<f64>>,
    pub transformed_b: Vec<Vec<f64>>,
    pub transformed_c: Vec<Vec<f64>>,
    pub controllability_index: usize,
    pub diagonal_block_dimensions: Vec<usize>,
    pub denominator_degrees: Vec<usize>,
    pub transfer_polynomials: Vec<TransferPolynomial>,
}

/// Errors produced while parsing the upstream `TB04AD` assets.
#[derive(Debug, Error)]
pub enum Tb04AdExampleError {
    #[error("failed to read TB04AD asset {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("missing TB04AD section: {section}")]
    MissingSection { section: &'static str },
    #[error("unexpected end of TB04AD data while parsing {field}")]
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
    #[error("invalid TB04AD row/column selector: {value}")]
    InvalidRowCol { value: String },
    #[error("invalid TB04AD transfer element header: {line}")]
    InvalidElementHeader { line: String },
}

/// Loads the checked-in upstream `TB04AD` example from `root`.
///
/// # Errors
///
/// Returns [`Tb04AdExampleError`] if the example input or result files cannot
/// be read or parsed.
pub fn load_tb04ad_case(root: impl AsRef<Path>) -> Result<Tb04AdCase, Tb04AdExampleError> {
    let root = root.as_ref();
    let input = parse_tb04ad_input_file(root.join("data/TB04AD.dat"))?;
    let output = parse_tb04ad_result_file(root.join("results/TB04AD.res"), &input)?;
    Ok(Tb04AdCase { input, output })
}

/// Parses the upstream `TB04AD.dat` input file.
///
/// # Errors
///
/// Returns [`Tb04AdExampleError`] if the file cannot be read or the token
/// stream cannot be converted into the expected matrices.
pub fn parse_tb04ad_input_file(path: impl AsRef<Path>) -> Result<Tb04AdInput, Tb04AdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Tb04AdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut lines = contents.lines();
    let _ = lines.next();
    let body = lines.collect::<Vec<_>>().join(" ");
    let mut tokens = body.split_whitespace();

    let state_count = parse_next_usize(&mut tokens, "n")?;
    let input_count = parse_next_usize(&mut tokens, "m")?;
    let output_count = parse_next_usize(&mut tokens, "p")?;
    let tol1 = parse_next_f64(&mut tokens, "tol1")?;
    let tol2 = parse_next_f64(&mut tokens, "tol2")?;
    let rowcol_token = next_token(&mut tokens, "rowcol")?;
    let rowcol = parse_rowcol(rowcol_token)?;

    let a = read_row_major_matrix(&mut tokens, state_count, state_count, "A")?;
    let b = read_column_major_matrix(&mut tokens, state_count, input_count, "B")?;
    let c = read_row_major_matrix(&mut tokens, output_count, state_count, "C")?;
    let d = read_row_major_matrix(&mut tokens, output_count, input_count, "D")?;

    Ok(Tb04AdInput {
        n: state_count,
        m: input_count,
        p: output_count,
        tol1,
        tol2,
        rowcol,
        a,
        b,
        c,
        d,
    })
}

/// Parses the checked-in `TB04AD.res` file.
///
/// # Errors
///
/// Returns [`Tb04AdExampleError`] if the result file cannot be read or if the
/// expected sections cannot be located and parsed.
pub fn parse_tb04ad_result_file(
    path: impl AsRef<Path>,
    input: &Tb04AdInput,
) -> Result<Tb04AdOutput, Tb04AdExampleError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| Tb04AdExampleError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines = contents.lines().collect::<Vec<_>>();

    let order_index = find_line(
        &lines,
        "The order of the transformed state-space representation",
    )?;
    let nr = parse_usize_after_equals(lines[order_index], "nr")?;

    let a_index = find_line(&lines, "The transformed state dynamics matrix A is")?;
    let transformed_a = read_matrix_rows(&lines, a_index + 1, nr, "transformed A")?;

    let b_index = find_line(&lines, "The transformed input/state matrix B is")?;
    let transformed_b = read_matrix_rows(&lines, b_index + 1, nr, "transformed B")?;

    let c_index = find_line(&lines, "The transformed state/output matrix C is")?;
    let transformed_c = read_matrix_rows(&lines, c_index + 1, input.p, "transformed C")?;

    let controllability_index_line = find_line(
        &lines,
        "The controllability index of the transformed state-space representation",
    )?;
    let controllability_index =
        parse_usize_after_equals(lines[controllability_index_line], "controllability index")?;

    let diagonal_blocks_line = find_line(
        &lines,
        "The dimensions of the diagonal blocks of the transformed A are",
    )?;
    let diagonal_block_dimensions =
        parse_usize_list(lines[diagonal_blocks_line + 1], "diagonal block dimensions")?;

    let denominator_degrees_line =
        find_line(&lines, "The degrees of the denominator polynomials are")?;
    let porm = if input.rowcol == 'R' {
        input.p
    } else {
        input.m
    };
    let denominator_degrees =
        parse_usize_list(lines[denominator_degrees_line + 1], "denominator degrees")?;
    if denominator_degrees.len() != porm {
        return Err(Tb04AdExampleError::MissingSection {
            section: "denominator degree count mismatch",
        });
    }

    let mut transfer_polynomials = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if !line.trim_start().starts_with("element (") {
            continue;
        }

        let (row, column, numerator) = parse_element_header(line)?;
        let denominator_line = lines
            .get(index + 2)
            .ok_or(Tb04AdExampleError::UnexpectedEnd {
                field: "transfer denominator",
            })?;
        let denominator = parse_f64_list(denominator_line, "transfer denominator")?;
        transfer_polynomials.push(TransferPolynomial {
            row,
            column,
            numerator,
            denominator,
        });
    }

    Ok(Tb04AdOutput {
        nr,
        transformed_a,
        transformed_b,
        transformed_c,
        controllability_index,
        diagonal_block_dimensions,
        denominator_degrees,
        transfer_polynomials,
    })
}

fn find_line(lines: &[&str], needle: &'static str) -> Result<usize, Tb04AdExampleError> {
    lines
        .iter()
        .position(|line| line.contains(needle))
        .ok_or(Tb04AdExampleError::MissingSection { section: needle })
}

fn next_token<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<&'input str, Tb04AdExampleError> {
    tokens
        .next()
        .ok_or(Tb04AdExampleError::UnexpectedEnd { field })
}

fn parse_rowcol(token: &str) -> Result<char, Tb04AdExampleError> {
    let mut chars = token.chars();
    match chars.next() {
        Some(value @ ('R' | 'C')) if chars.next().is_none() => Ok(value),
        _ => Err(Tb04AdExampleError::InvalidRowCol {
            value: token.to_owned(),
        }),
    }
}

fn parse_next_usize<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<usize, Tb04AdExampleError> {
    let token = next_token(tokens, field)?;
    token
        .parse::<usize>()
        .map_err(|source| Tb04AdExampleError::ParseInt {
            field,
            token: token.to_owned(),
            source,
        })
}

fn parse_next_f64<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    field: &'static str,
) -> Result<f64, Tb04AdExampleError> {
    let token = next_token(tokens, field)?;
    token
        .parse::<f64>()
        .map_err(|source| Tb04AdExampleError::ParseFloat {
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
) -> Result<Vec<Vec<f64>>, Tb04AdExampleError> {
    let mut matrix = vec![vec![0.0; columns]; rows];
    for row in &mut matrix {
        for value in row {
            *value = parse_next_f64(tokens, field)?;
        }
    }
    Ok(matrix)
}

fn read_column_major_matrix<'input>(
    tokens: &mut impl Iterator<Item = &'input str>,
    rows: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Tb04AdExampleError> {
    let mut matrix = vec![vec![0.0; columns]; rows];
    for column in 0..columns {
        for row in matrix.iter_mut().take(rows) {
            row[column] = parse_next_f64(tokens, field)?;
        }
    }
    Ok(matrix)
}

fn read_matrix_rows(
    lines: &[&str],
    start: usize,
    rows: usize,
    field: &'static str,
) -> Result<Vec<Vec<f64>>, Tb04AdExampleError> {
    let mut matrix = Vec::with_capacity(rows);
    for offset in 0..rows {
        let line = lines
            .get(start + offset)
            .ok_or(Tb04AdExampleError::UnexpectedEnd { field })?;
        matrix.push(parse_f64_list(line, field)?);
    }
    Ok(matrix)
}

fn parse_usize_after_equals(line: &str, field: &'static str) -> Result<usize, Tb04AdExampleError> {
    let token = line
        .split('=')
        .nth(1)
        .ok_or(Tb04AdExampleError::MissingSection { section: field })?
        .trim();
    token
        .parse::<usize>()
        .map_err(|source| Tb04AdExampleError::ParseInt {
            field,
            token: token.to_owned(),
            source,
        })
}

fn parse_usize_list(line: &str, field: &'static str) -> Result<Vec<usize>, Tb04AdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<usize>()
                .map_err(|source| Tb04AdExampleError::ParseInt {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}

fn parse_f64_list(line: &str, field: &'static str) -> Result<Vec<f64>, Tb04AdExampleError> {
    line.split_whitespace()
        .map(|token| {
            token
                .parse::<f64>()
                .map_err(|source| Tb04AdExampleError::ParseFloat {
                    field,
                    token: token.to_owned(),
                    source,
                })
        })
        .collect()
}

fn parse_element_header(line: &str) -> Result<(usize, usize, Vec<f64>), Tb04AdExampleError> {
    let Some((prefix, numerator_text)) = line.split_once("is") else {
        return Err(Tb04AdExampleError::InvalidElementHeader {
            line: line.to_owned(),
        });
    };

    let prefix = prefix.trim();
    let prefix =
        prefix
            .strip_prefix("element (")
            .ok_or(Tb04AdExampleError::InvalidElementHeader {
                line: line.to_owned(),
            })?;
    let prefix = prefix
        .strip_suffix(')')
        .ok_or(Tb04AdExampleError::InvalidElementHeader {
            line: line.to_owned(),
        })?;

    let Some((row_text, column_text)) = prefix.split_once(',') else {
        return Err(Tb04AdExampleError::InvalidElementHeader {
            line: line.to_owned(),
        });
    };

    let row = row_text
        .trim()
        .parse::<usize>()
        .map_err(|source| Tb04AdExampleError::ParseInt {
            field: "transfer row",
            token: row_text.trim().to_owned(),
            source,
        })?;
    let column =
        column_text
            .trim()
            .parse::<usize>()
            .map_err(|source| Tb04AdExampleError::ParseInt {
                field: "transfer column",
                token: column_text.trim().to_owned(),
                source,
            })?;
    let numerator = parse_f64_list(numerator_text, "transfer numerator")?;

    Ok((row, column, numerator))
}
