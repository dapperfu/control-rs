.PHONY: build clean format help lint run test test-coverage test-doc test-verbose

build:
	cargo build --workspace

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
		'build          Build all workspace crates' \
		'run            Print a summary of upstream SLICOT example coverage' \
		'test           Run unit, integration, and documentation tests' \
		'test-verbose   Run tests without output capture' \
		'test-doc       Run documentation tests' \
		'test-coverage  Generate a tarpaulin coverage report' \
		'lint           Run clippy with warnings denied' \
		'format         Format the workspace with rustfmt' \
		'clean          Remove cargo build artifacts'
