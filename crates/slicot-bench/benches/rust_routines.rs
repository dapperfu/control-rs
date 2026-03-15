//! Benchmarks for pure-Rust SLICOT routine ports.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slicot_routines::sb02md_solve;

/// Small CARE: 2×2 system, single solve.
fn sb02md_small(c: &mut Criterion) {
    let a = vec![vec![0.0, 1.0], vec![0.0, -1.0]];
    let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let g = vec![vec![1.0, 0.0], vec![0.0, 0.0]];
    c.bench_function("sb02md_care_2x2", |b| {
        b.iter(|| {
            sb02md_solve(black_box('C'), &a, &q, &g).expect("CARE should succeed")
        });
    });
}

criterion_group!(benches, sb02md_small);
criterion_main!(benches);
