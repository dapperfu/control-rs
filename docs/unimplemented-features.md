# Unimplemented Features

This document lists unimplemented features in the control-rs project: plan TODOs, routine subset limitations, and full-library gaps.

## Table 1: Plan TODOs (unimplemented)

| TODO ID | Content | Source |
|---------|---------|--------|
| inventory-routines | Inventory all user-callable SLICOT routines and map each to upstream example and target Rust module | pure-rust-slicot-port.md |
| build-upstream-harness | Parser/runner for examples/data/*.dat and results/*.res → Rust golden tests | pure-rust-slicot-port.md |
| build-compat-harness | Compatibility harness for python-control Slycot-backed tests | pure-rust-slicot-port.md |
| port-phase-one | Port phase-one subset with one-to-one tests (plan status still "pending" though routines are ported) | pure-rust-slicot-port.md |
| expand-full-library | Port remaining SLICOT chapters with upstream example parity | pure-rust-slicot-port.md |

## Table 2: Routine subset / partial implementation gaps

| Routine | Implemented | Unimplemented / limitation |
|---------|-------------|----------------------------|
| SB01BD | Single-input pole placement (Ackermann) | Multi-input (m>1); returns `MultiInputNotImplemented` |
| SB02MD | Continuous CARE | Discrete (DICO='D'); returns `UnsupportedDico` |
| SB03MD | Continuous/discrete Lyapunov, JOB='X', FACT='N', TRANA='N'/'T' | Other JOB/FACT/TRANA; documented as subset |
| SB03OD | Continuous Cholesky factor, FACT='N', TRANS='N' | Discrete; other FACT; other TRANS |
| SB04MD / SB04QD | Sylvester solvers | Documented as subset; no explicit unimplemented error variants |
| SG02AD | Continuous CARE, E=I, L=0 | Discrete; general E; non-zero L |
| SG03AD | Continuous generalized Lyapunov, FACT='N', JOB='X' | Discrete; other FACT/JOB |
| TB04AD | Transfer matrix evaluation (one rational per element) | Full TB04AD API if different |
| TD04AD | ROWCOL='R' (row form) only | ROWCOL='C' (column form); returns error |
| AB08ND | Invariant zeros when D square and invertible | D singular or non-square; requires QZ (returns `DNotInvertible`) |
| AB13DD | Standard state-space (no descriptor E) | Descriptor systems (E ≠ I) |
| SB10AD | State-feedback H-infinity (CARE-based) | Full Glover-Doyle output-feedback (two AREs, general plant) |
| SB10HD | State-feedback H2 (CARE-based) | Full H2 output-feedback synthesis |

## Phase 3 / full library

Full SLICOT expansion (remaining user-callable routines by chapter) is not yet enumerated in this repo; see Phase 3 in [plans/pure-rust-slicot-port.md](plans/pure-rust-slicot-port.md).
