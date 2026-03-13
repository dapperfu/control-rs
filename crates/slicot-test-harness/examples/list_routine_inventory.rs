use std::{path::PathBuf, process::ExitCode};

use slicot_test_harness::discover_routine_inventory;

fn main() -> ExitCode {
    let root = PathBuf::from("SLICOT-Reference/examples");
    match discover_routine_inventory(root) {
        Ok(entries) => {
            let with_results = entries.iter().filter(|entry| entry.has_result_file).count();
            println!("routines: {}", entries.len());
            println!("with golden results: {with_results}");
            for entry in entries.iter().take(10) {
                println!(
                    "{} {} {:?}",
                    entry.routine_stem,
                    entry.target_module.as_str(),
                    entry.phase_one_routine
                );
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
