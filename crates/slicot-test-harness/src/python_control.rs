//! Compatibility targets derived from the `python-control` Slycot-backed tests.

use std::path::{Path, PathBuf};

use slicot_routines::RoutineId;

/// A `python-control` test target relevant to the phase-one Rust port.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PythonControlTarget {
    pub test_path: &'static str,
    pub routines: &'static [RoutineId],
    pub behavior: &'static str,
}

/// A resolved `python-control` test target with an absolute or relative path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedPythonControlTarget {
    pub test_path: PathBuf,
    pub routines: Vec<RoutineId>,
    pub behavior: &'static str,
    pub exists: bool,
}

const PHASE_ONE_PYTHON_CONTROL_TARGETS: [PythonControlTarget; 10] = [
    PythonControlTarget {
        test_path: "control/tests/convert_test.py",
        routines: &[RoutineId::Tb04Ad, RoutineId::Td04Ad],
        behavior: "State-space and transfer-function conversion round trips, MIMO conversion, and tf2ss edge cases.",
    },
    PythonControlTarget {
        test_path: "control/tests/freqresp_test.py",
        routines: &[RoutineId::Tb04Ad, RoutineId::Tb05Ad],
        behavior: "Frequency-response checks that indirectly depend on successful SS conversion and state-space evaluation.",
    },
    PythonControlTarget {
        test_path: "control/tests/statesp_test.py",
        routines: &[
            RoutineId::Ab08Nd,
            RoutineId::Ab13Dd,
            RoutineId::Tb01Pd,
            RoutineId::Tb05Ad,
            RoutineId::Td04Ad,
        ],
        behavior: "Transmission zeros, linfnorm, minimal realization, state-space evaluation, and tf2ss behavior.",
    },
    PythonControlTarget {
        test_path: "control/tests/minreal_test.py",
        routines: &[RoutineId::Tb01Pd],
        behavior: "State-space minimal realization preserves reduced dynamics and I/O behavior.",
    },
    PythonControlTarget {
        test_path: "control/tests/statefbk_test.py",
        routines: &[
            RoutineId::Sb01Bd,
            RoutineId::Sb02Md,
            RoutineId::Sb02Mt,
            RoutineId::Sb03Md,
            RoutineId::Sb03Od,
            RoutineId::Sg02Ad,
        ],
        behavior: "Pole placement, LQR wrappers, and Gramian computations for controllability/observability paths.",
    },
    PythonControlTarget {
        test_path: "control/tests/mateqn_test.py",
        routines: &[
            RoutineId::Sb02Md,
            RoutineId::Sb02Mt,
            RoutineId::Sb03Md,
            RoutineId::Sb04Md,
            RoutineId::Sb04Qd,
            RoutineId::Sg02Ad,
            RoutineId::Sg03Ad,
        ],
        behavior: "Continuous/discrete Lyapunov, Sylvester, CARE, DARE, and generalized equation residual checks.",
    },
    PythonControlTarget {
        test_path: "control/tests/modelsimp_test.py",
        routines: &[RoutineId::Ab09Ad, RoutineId::Ab09Md, RoutineId::Ab09Nd],
        behavior: "Balanced truncation and match-DC reduction, with partial coverage of unstable-system branches.",
    },
    PythonControlTarget {
        test_path: "control/tests/sysnorm_test.py",
        routines: &[RoutineId::Ab13Bd, RoutineId::Ab13Dd],
        behavior: "H2 and H-infinity norm checks for stable, unstable, and MIMO systems.",
    },
    PythonControlTarget {
        test_path: "control/tests/canonical_test.py",
        routines: &[RoutineId::Mb03Rd],
        behavior: "Block-diagonal Schur decomposition and modal-form regression cases.",
    },
    PythonControlTarget {
        test_path: "control/tests/robust_test.py",
        routines: &[RoutineId::Sb10Ad, RoutineId::Sb10Hd],
        behavior: "H-infinity and H2 synthesis regression checks against reference controller realizations.",
    },
];

/// Returns the phase-one `python-control` compatibility targets.
#[must_use]
pub const fn phase_one_python_control_targets() -> &'static [PythonControlTarget] {
    &PHASE_ONE_PYTHON_CONTROL_TARGETS
}

/// Resolves the phase-one `python-control` compatibility targets against a
/// repository root containing the `python-control` checkout.
#[must_use]
pub fn resolve_phase_one_python_control_targets(
    root: impl AsRef<Path>,
) -> Vec<ResolvedPythonControlTarget> {
    let root = root.as_ref();
    phase_one_python_control_targets()
        .iter()
        .map(|target| {
            let test_path = root.join(target.test_path);
            ResolvedPythonControlTarget {
                exists: test_path.exists(),
                test_path,
                routines: target.routines.to_vec(),
                behavior: target.behavior,
            }
        })
        .collect()
}
