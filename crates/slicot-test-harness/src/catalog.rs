//! Discovery utilities for the upstream SLICOT example corpus.

use std::{
    fs,
    path::{Path, PathBuf},
};

use slicot_routines::RoutineId;

use crate::CatalogError;

/// A single upstream SLICOT example program and its associated golden files.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExampleCase {
    /// Upstream example program stem, including the leading `T`.
    pub example_name: String,
    /// Uppercase SLICOT routine stem, derived by dropping the leading `T`.
    pub routine_stem: String,
    /// Absolute or relative path to the upstream Fortran example source file.
    pub example_path: PathBuf,
    /// Path to the example input file, if one exists.
    pub data_path: Option<PathBuf>,
    /// Path to the expected result file, if one exists.
    pub result_path: Option<PathBuf>,
    /// Phase-one routine classification, when the routine is in the first port set.
    pub phase_one_routine: Option<RoutineId>,
}

impl ExampleCase {
    /// Returns `true` when the example has a checked-in golden result file.
    #[must_use]
    pub fn has_golden_result(&self) -> bool {
        self.result_path.is_some()
    }
}

/// Discovers all upstream SLICOT examples under `root`.
///
/// The discovery logic mirrors the upstream `examples/CMakeLists.txt` behavior:
/// source files named `T*.f` or `T*.f90` map to data and result files named
/// after the example stem with the leading `T` removed.
///
/// # Errors
///
/// Returns [`CatalogError`] if the directory cannot be enumerated or if an
/// example filename does not follow the upstream naming convention.
pub fn discover_example_cases(root: impl AsRef<Path>) -> Result<Vec<ExampleCase>, CatalogError> {
    let root = root.as_ref();
    let entries = fs::read_dir(root).map_err(|source| CatalogError::ReadDir {
        path: root.to_path_buf(),
        source,
    })?;

    let data_dir = root.join("data");
    let results_dir = root.join("results");
    let mut cases = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|source| CatalogError::ReadEntry {
            path: root.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if extension != "f" && extension != "f90" {
            continue;
        }

        let Some(raw_example_name) = path.file_stem().and_then(|value| value.to_str()) else {
            return Err(CatalogError::InvalidExampleName { path });
        };
        let example_name = raw_example_name.to_ascii_uppercase();
        let Some(routine_stem) = example_name.strip_prefix('T') else {
            return Err(CatalogError::InvalidExampleName { path });
        };

        let data_path = data_dir.join(format!("{routine_stem}.dat"));
        let result_path = results_dir.join(format!("{routine_stem}.res"));

        cases.push(ExampleCase {
            example_name: example_name.clone(),
            routine_stem: routine_stem.to_owned(),
            example_path: path.clone(),
            data_path: data_path.exists().then_some(data_path),
            result_path: result_path.exists().then_some(result_path),
            phase_one_routine: RoutineId::from_stem(routine_stem),
        });
    }

    cases.sort_by(|left, right| {
        left.routine_stem
            .cmp(&right.routine_stem)
            .then(left.example_name.cmp(&right.example_name))
    });

    Ok(cases)
}
