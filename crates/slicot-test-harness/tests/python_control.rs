use std::path::{Path, PathBuf};

use slicot_routines::RoutineId;
use slicot_test_harness::resolve_phase_one_python_control_targets;

fn python_control_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../python-control")
        .canonicalize()
        .expect("python-control checkout should exist")
}

#[test]
fn all_phase_one_python_control_targets_exist() {
    let targets = resolve_phase_one_python_control_targets(python_control_root());
    assert!(!targets.is_empty());
    assert!(targets.iter().all(|target| target.exists));
}

#[test]
fn every_phase_one_routine_has_a_python_control_target() {
    let targets = resolve_phase_one_python_control_targets(python_control_root());
    let covered_routines = targets
        .iter()
        .flat_map(|target| target.routines.iter().copied())
        .collect::<Vec<_>>();

    for routine in [
        RoutineId::Ab08Nd,
        RoutineId::Ab09Ad,
        RoutineId::Ab09Md,
        RoutineId::Ab09Nd,
        RoutineId::Ab13Bd,
        RoutineId::Ab13Dd,
        RoutineId::Mb03Rd,
        RoutineId::Sb01Bd,
        RoutineId::Sb02Md,
        RoutineId::Sb02Mt,
        RoutineId::Sb03Md,
        RoutineId::Sb03Od,
        RoutineId::Sb04Md,
        RoutineId::Sb04Qd,
        RoutineId::Sb10Ad,
        RoutineId::Sb10Hd,
        RoutineId::Sg02Ad,
        RoutineId::Sg03Ad,
        RoutineId::Tb01Pd,
        RoutineId::Tb04Ad,
        RoutineId::Tb05Ad,
        RoutineId::Td04Ad,
    ] {
        assert!(covered_routines.contains(&routine));
    }
}
