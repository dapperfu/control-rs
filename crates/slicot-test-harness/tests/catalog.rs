use std::path::{Path, PathBuf};

use slicot_routines::RoutineId;
use slicot_test_harness::discover_example_cases;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn discovers_large_upstream_catalog() {
    let cases = discover_example_cases(examples_root()).expect("catalog discovery should succeed");

    assert!(
        cases.len() > 200,
        "expected the full upstream example corpus"
    );
    assert!(cases.iter().all(|case| case.example_name.starts_with('T')));
}

#[test]
fn maps_phase_one_routines() {
    let cases = discover_example_cases(examples_root()).expect("catalog discovery should succeed");

    let tb04ad = cases
        .iter()
        .find(|case| case.routine_stem == "TB04AD")
        .expect("TB04AD example should exist");
    assert_eq!(tb04ad.phase_one_routine, Some(RoutineId::Tb04Ad));
    assert!(tb04ad.data_path.is_some());
    assert!(tb04ad.has_golden_result());

    let sg03ad = cases
        .iter()
        .find(|case| case.routine_stem == "SG03AD")
        .expect("SG03AD example should exist");
    assert_eq!(sg03ad.phase_one_routine, Some(RoutineId::Sg03Ad));
}

#[test]
fn preserves_suffixes_in_example_names() {
    let cases = discover_example_cases(examples_root()).expect("catalog discovery should succeed");

    let sb02rd_second = cases
        .iter()
        .find(|case| case.example_name == "TSB02RD_2")
        .expect("TSB02RD_2 example should exist");
    assert_eq!(sb02rd_second.routine_stem, "SB02RD_2");
    assert_eq!(sb02rd_second.phase_one_routine, None);
}
