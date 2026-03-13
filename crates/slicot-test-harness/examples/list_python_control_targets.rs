use std::{path::PathBuf, process::ExitCode};

use slicot_test_harness::resolve_phase_one_python_control_targets;

fn main() -> ExitCode {
    let root = PathBuf::from("python-control");
    let targets = resolve_phase_one_python_control_targets(root);

    for target in targets {
        println!(
            "{} exists={} routines={:?}",
            target.test_path.display(),
            target.exists,
            target.routines
        );
    }

    ExitCode::SUCCESS
}
