//! Routine inventory generation from the upstream SLICOT example corpus.

use std::{collections::BTreeMap, path::Path};

use slicot_routines::{target_rust_module_for_stem, RoutineId, TargetRustModule};

use crate::{discover_example_cases, CatalogError};

/// One grouped routine entry derived from the upstream example manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutineInventoryEntry {
    pub routine_stem: String,
    pub example_names: Vec<String>,
    pub target_module: TargetRustModule,
    pub phase_one_routine: Option<RoutineId>,
    pub has_data_file: bool,
    pub has_result_file: bool,
}

/// Groups discovered examples by routine stem and attaches the target Rust
/// module family for the pure-Rust port.
///
/// # Errors
///
/// Returns [`CatalogError`] if the upstream example corpus cannot be read or a
/// discovered routine stem cannot be mapped to a known SLICOT chapter family.
pub fn discover_routine_inventory(
    root: impl AsRef<Path>,
) -> Result<Vec<RoutineInventoryEntry>, CatalogError> {
    let example_cases = discover_example_cases(root)?;
    let mut grouped = BTreeMap::<String, Vec<_>>::new();

    for case in example_cases {
        grouped
            .entry(case.routine_stem.clone())
            .or_default()
            .push(case);
    }

    let mut inventory = Vec::with_capacity(grouped.len());
    for (routine_stem, cases) in grouped {
        let target_module = target_rust_module_for_stem(&routine_stem).ok_or_else(|| {
            CatalogError::UnknownRoutineStem {
                routine_stem: routine_stem.clone(),
            }
        })?;
        let example_names = cases
            .iter()
            .map(|case| case.example_name.clone())
            .collect::<Vec<_>>();
        let has_data_file = cases.iter().all(|case| case.data_path.is_some());
        let has_result_file = cases.iter().all(|case| case.result_path.is_some());
        let phase_one_routine = RoutineId::from_stem(&routine_stem);

        inventory.push(RoutineInventoryEntry {
            routine_stem,
            example_names,
            target_module,
            phase_one_routine,
            has_data_file,
            has_result_file,
        });
    }

    Ok(inventory)
}
