//! Consumer-facing compatibility metadata for the first Rust port wave.

use crate::RoutineId;

/// `python-control` usage metadata for one SLICOT routine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PythonControlUsage {
    pub routine: RoutineId,
    pub module_paths: &'static [&'static str],
    pub public_apis: &'static [&'static str],
    pub fallback_behavior: &'static str,
}

const PHASE_ONE_COMPATIBILITY: [PythonControlUsage; 23] = [
    PythonControlUsage {
        routine: RoutineId::Ab08Nd,
        module_paths: &["control/statesp.py"],
        public_apis: &["StateSpace.zeros"],
        fallback_behavior: "Falls back to SciPy generalized eig/QZ for square input/output systems only.",
    },
    PythonControlUsage {
        routine: RoutineId::Ab09Ad,
        module_paths: &["control/modelsimp.py"],
        public_apis: &["balanced_reduction", "balred"],
        fallback_behavior: "No SciPy fallback in balanced_reduction for this stable-system branch.",
    },
    PythonControlUsage {
        routine: RoutineId::Ab09Md,
        module_paths: &["control/modelsimp.py"],
        public_apis: &["balanced_reduction", "balred"],
        fallback_behavior: "No SciPy fallback in balanced_reduction for this unstable-system branch.",
    },
    PythonControlUsage {
        routine: RoutineId::Ab09Nd,
        module_paths: &["control/modelsimp.py"],
        public_apis: &["balanced_reduction", "balred"],
        fallback_behavior: "No SciPy fallback in balanced_reduction for method='matchdc'.",
    },
    PythonControlUsage {
        routine: RoutineId::Ab13Bd,
        module_paths: &["control/sysnorm.py"],
        public_apis: &["system_norm", "norm"],
        fallback_behavior: "Falls back to Lyapunov-based SciPy/Python code when method resolves to SciPy.",
    },
    PythonControlUsage {
        routine: RoutineId::Ab13Dd,
        module_paths: &["control/statesp.py", "control/sysnorm.py"],
        public_apis: &["linfnorm", "system_norm", "norm"],
        fallback_behavior: "Used for L-infinity norm paths; system_norm has a non-Slycot fallback path.",
    },
    PythonControlUsage {
        routine: RoutineId::Ab13Md,
        module_paths: &[],
        public_apis: &[],
        fallback_behavior: "Not directly used by the inspected python-control modules.",
    },
    PythonControlUsage {
        routine: RoutineId::Mb03Rd,
        module_paths: &["control/canonical.py"],
        public_apis: &["bdschur", "modal_form", "canonical_form"],
        fallback_behavior: "SciPy performs the initial Schur factorization, but MB03RD is still required for block diagonalization.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb01Bd,
        module_paths: &["control/statefbk.py"],
        public_apis: &["place_varga"],
        fallback_behavior: "No fallback inside place_varga; users must choose alternate placement APIs.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb02Md,
        module_paths: &["control/mateqn.py"],
        public_apis: &["care", "lqr"],
        fallback_behavior: "Standard CARE path falls back to scipy.linalg.solve_continuous_are when requested.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb02Mt,
        module_paths: &["control/mateqn.py"],
        public_apis: &["care", "lqr"],
        fallback_behavior: "Used as a preprocessing step for the Slycot CARE path only.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb03Md,
        module_paths: &["control/mateqn.py", "control/statefbk.py"],
        public_apis: &["lyap", "dlyap", "gram", "hankel_singular_values", "hsvd"],
        fallback_behavior: "Standard Lyapunov problems fall back to SciPy when method resolves to SciPy.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb03Od,
        module_paths: &["control/statefbk.py"],
        public_apis: &["gram"],
        fallback_behavior: "No fallback for Cholesky-factor Gramian variants.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb04Md,
        module_paths: &["control/mateqn.py"],
        public_apis: &["lyap"],
        fallback_behavior: "Sylvester equation path falls back to scipy.linalg.solve_sylvester.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb04Qd,
        module_paths: &["control/mateqn.py"],
        public_apis: &["dlyap"],
        fallback_behavior: "No SciPy fallback for the discrete Sylvester variant.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb10Ad,
        module_paths: &["control/robust.py"],
        public_apis: &["hinfsyn", "mixsyn"],
        fallback_behavior: "No fallback; the routine is required for the synthesis path.",
    },
    PythonControlUsage {
        routine: RoutineId::Sb10Hd,
        module_paths: &["control/robust.py"],
        public_apis: &["h2syn"],
        fallback_behavior: "No fallback; the routine is required for the synthesis path.",
    },
    PythonControlUsage {
        routine: RoutineId::Sg02Ad,
        module_paths: &["control/mateqn.py"],
        public_apis: &["care", "dare", "lqr", "dlqr"],
        fallback_behavior: "SciPy covers supported generalized ARE/DARE cases when method resolves away from Slycot.",
    },
    PythonControlUsage {
        routine: RoutineId::Sg03Ad,
        module_paths: &["control/mateqn.py"],
        public_apis: &["lyap", "dlyap"],
        fallback_behavior: "No fallback for generalized Lyapunov variants.",
    },
    PythonControlUsage {
        routine: RoutineId::Tb01Pd,
        module_paths: &["control/statesp.py", "control/modelsimp.py"],
        public_apis: &["StateSpace.minreal", "minimal_realization", "minreal"],
        fallback_behavior: "No fallback in StateSpace.minreal; the API raises if Slycot is unavailable.",
    },
    PythonControlUsage {
        routine: RoutineId::Tb04Ad,
        module_paths: &["control/xferfcn.py", "control/tests/slycot_convert_test.py"],
        public_apis: &["ss2tf", "tf"],
        fallback_behavior: "Library code falls back to scipy.signal.ss2tf when Slycot is unavailable.",
    },
    PythonControlUsage {
        routine: RoutineId::Tb05Ad,
        module_paths: &["control/statesp.py"],
        public_apis: &["StateSpace.slycot_laub", "StateSpace.horner", "StateSpace.__call__"],
        fallback_behavior: "StateSpace.horner catches failures and falls back to direct linear solves in Python/SciPy.",
    },
    PythonControlUsage {
        routine: RoutineId::Td04Ad,
        module_paths: &["control/statesp.py", "control/tests/slycot_convert_test.py"],
        public_apis: &["tf2ss", "ss"],
        fallback_behavior: "Falls back to SciPy only for SISO or static cases; general MIMO conversion still requires Slycot.",
    },
];

/// Returns the compatibility metadata for the phase-one routines.
#[must_use]
pub const fn phase_one_compatibility() -> &'static [PythonControlUsage] {
    &PHASE_ONE_COMPATIBILITY
}

#[cfg(test)]
mod tests {
    use super::phase_one_compatibility;
    use crate::PHASE_ONE_ROUTINES;

    #[test]
    fn every_phase_one_routine_has_compatibility_metadata() {
        let compatibility = phase_one_compatibility();
        assert_eq!(compatibility.len(), PHASE_ONE_ROUTINES.len());

        for routine in PHASE_ONE_ROUTINES {
            assert!(compatibility.iter().any(|entry| entry.routine == routine));
        }
    }
}
