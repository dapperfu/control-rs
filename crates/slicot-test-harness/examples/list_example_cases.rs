use std::{env, path::PathBuf, process::ExitCode};

use slicot_test_harness::discover_example_cases;

fn main() -> ExitCode {
    let root = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("SLICOT-Reference/examples"));

    match discover_example_cases(&root) {
        Ok(cases) => {
            let golden_count = cases.iter().filter(|case| case.has_golden_result()).count();
            let phase_one_count = cases
                .iter()
                .filter(|case| case.phase_one_routine.is_some())
                .count();

            println!("examples: {}", cases.len());
            println!("with golden results: {golden_count}");
            println!("phase-one routines represented: {phase_one_count}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
