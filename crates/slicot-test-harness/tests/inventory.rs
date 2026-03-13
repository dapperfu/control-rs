use std::path::{Path, PathBuf};

use slicot_routines::{RoutineId, TargetRustModule};
use slicot_test_harness::discover_routine_inventory;

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../SLICOT-Reference/examples")
        .canonicalize()
        .expect("SLICOT examples directory should exist")
}

#[test]
fn inventories_the_upstream_routine_surface() {
    let inventory =
        discover_routine_inventory(examples_root()).expect("inventory discovery should succeed");

    assert!(
        inventory.len() > 200,
        "expected broad upstream routine coverage"
    );
}

#[test]
fn classifies_phase_one_routines_and_module_families() {
    let inventory =
        discover_routine_inventory(examples_root()).expect("inventory discovery should succeed");

    let tb04ad = inventory
        .iter()
        .find(|entry| entry.routine_stem == "TB04AD")
        .expect("TB04AD inventory entry should exist");
    assert_eq!(tb04ad.target_module, TargetRustModule::Transformation);
    assert_eq!(tb04ad.phase_one_routine, Some(RoutineId::Tb04Ad));
    assert_eq!(tb04ad.example_names, vec!["TTB04AD"]);
    assert!(tb04ad.has_data_file);
    assert!(tb04ad.has_result_file);

    let mb03rd = inventory
        .iter()
        .find(|entry| entry.routine_stem == "MB03RD")
        .expect("MB03RD inventory entry should exist");
    assert_eq!(mb03rd.target_module, TargetRustModule::Mathematics);
    assert_eq!(mb03rd.phase_one_routine, Some(RoutineId::Mb03Rd));
}

#[test]
fn preserves_multiple_example_variants_for_one_routine() {
    let inventory =
        discover_routine_inventory(examples_root()).expect("inventory discovery should succeed");

    let sb02rd = inventory
        .iter()
        .find(|entry| entry.routine_stem == "SB02RD")
        .expect("SB02RD inventory entry should exist");
    assert_eq!(sb02rd.example_names, vec!["TSB02RD"]);

    let sb02rd_variant = inventory
        .iter()
        .find(|entry| entry.routine_stem == "SB02RD_2")
        .expect("SB02RD_2 inventory entry should exist");
    assert_eq!(sb02rd_variant.example_names, vec!["TSB02RD_2"]);
}
