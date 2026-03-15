.PHONY: bench build clean compare format help lint run test test-coverage test-doc test-verbose

bench:
	cargo bench -p slicot-bench

build:
	cargo build --workspace

compare:
	cargo run -p slicot-bench --bin compare_rust_vs_reference -- SLICOT-Reference/examples

run:
	cargo run --package slicot-test-harness --example list_example_cases -- SLICOT-Reference/examples

test:
	cargo test --workspace

test-verbose:
	cargo test --workspace -- --nocapture

test-doc:
	cargo test --workspace --doc

test-coverage:
	cargo tarpaulin --workspace --out Html --output-dir coverage

lint:
	cargo clippy --workspace --all-targets -- -D warnings

format:
	cargo fmt --all

clean:
	cargo clean

help:
	@printf '%s\n' \
		'bench          Run Criterion benchmarks for Rust SLICOT routines' \
		'build          Build all workspace crates' \
		'compare        Compare Rust routine outputs to Fortran reference (.res)' \
		'run            Print a summary of upstream SLICOT example coverage' \
		'test           Run unit, integration, and documentation tests' \
		'test-verbose   Run tests without output capture' \
		'test-doc       Run documentation tests' \
		'test-coverage  Generate a tarpaulin coverage report' \
		'lint           Run clippy with warnings denied' \
		'format         Format the workspace with rustfmt' \
		'clean          Remove cargo build artifacts'
