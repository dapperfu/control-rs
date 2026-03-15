# slicot-bench

Benchmark and correctness comparison for the pure-Rust SLICOT routines against the Fortran reference.

The **reference** is the upstream SLICOT-Reference result corpus: `.res` files under `SLICOT-Reference/examples/results/` produced by the official Fortran example programs. This crate does not compile or run Fortran; it compares Rust outputs to those parsed reference results.

## Correctness comparison

Run the comparison binary to check that each implemented Rust routine matches the reference (within routine-specific tolerances):

```bash
cargo run -p slicot-bench --bin compare_rust_vs_reference -- [EXAMPLES_ROOT]
```

- **EXAMPLES_ROOT** (optional): path to the SLICOT examples directory (contains `data/` and `results/`). Default: `SLICOT-Reference/examples` relative to the current working directory.
- **Exit code**: 0 if all run and pass; 1 if any routine fails the comparison.
- **Output**: A table of routine name, status (OK / FAIL / SKIP), and detail. SKIP means the fixture could not be loaded (e.g. missing SLICOT-Reference). AB13BD may report FAIL with "unstable A or Lyapunov failed" because the upstream AB13BD example uses an unstable A matrix and the current Rust implementation supports only stable systems.

## Performance benchmarks

Time the Rust routines on the same fixtures:

```bash
cargo bench -p slicot-bench
```

Benchmarks are skipped for a routine if its fixture is not found (e.g. SLICOT-Reference not present). Results are written to `target/criterion/`.

## Makefile

From the project root:

- `make compare` — run the correctness comparison.
- `make bench` — run Criterion benchmarks.
