//! Benchmarks proving that const-generic law bounds generate no runtime branching.
#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, unused_features)]
//!
//! `ConditionCell<N>` and `Between01<NUM, DEN>` carry zero bits at runtime.
//! Different const parameter combinations (e.g. `ConditionCell<8>` vs
//! `ConditionCell<4>`) are different *types* but identical *machine code* —
//! the law is enforced by the compiler, not by a runtime branch.
//!
//! The goal of these benchmarks is to show that two differently-parameterised
//! const-generic law types produce identical (vanishingly small) timings.
//! If law bounds were runtime-checked, we would see parameter-dependent
//! latency differences.
//!
//! Run with:
//! ```text
//! cargo bench --bench law_bounds_bench --all-features
//! ```

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wasm4pm_compat::conformance::{FitnessConst, PrecisionConst};
use wasm4pm_compat::law::{Between01, ConditionCell};

// ── ConditionCell construction: 8 bits vs 4 bits ────────────────────────────

fn bench_condition_cell_8(c: &mut Criterion) {
    c.bench_function("ConditionCell::<8>::new", |b| {
        b.iter(|| black_box(ConditionCell::<8>::new()))
    });
}

fn bench_condition_cell_4(c: &mut Criterion) {
    c.bench_function("ConditionCell::<4>::new", |b| {
        b.iter(|| black_box(ConditionCell::<4>::new()))
    });
}

fn bench_condition_cell_1(c: &mut Criterion) {
    c.bench_function("ConditionCell::<1>::new", |b| {
        b.iter(|| black_box(ConditionCell::<1>::new()))
    });
}

// ── Between01 construction: different ratios ─────────────────────────────────

fn bench_between01_3_4(c: &mut Criterion) {
    c.bench_function("Between01::<3,4>::new (0.75)", |b| {
        b.iter(|| black_box(Between01::<3, 4>::new()))
    });
}

fn bench_between01_1_2(c: &mut Criterion) {
    c.bench_function("Between01::<1,2>::new (0.5)", |b| {
        b.iter(|| black_box(Between01::<1, 2>::new()))
    });
}

fn bench_between01_1_1(c: &mut Criterion) {
    c.bench_function("Between01::<1,1>::new (1.0)", |b| {
        b.iter(|| black_box(Between01::<1, 1>::new()))
    });
}

// ── Metric (FitnessConst / PrecisionConst): different params ─────────────────
//
// FitnessConst<NUM, DEN> = Metric<Fitness, NUM, DEN>
// Both different const params must produce identical runtime overhead (zero).

fn bench_fitness_3_4(c: &mut Criterion) {
    c.bench_function("FitnessConst::<3,4>::new", |b| {
        b.iter(|| black_box(FitnessConst::<3, 4>::new()))
    });
}

fn bench_fitness_1_2(c: &mut Criterion) {
    c.bench_function("FitnessConst::<1,2>::new", |b| {
        b.iter(|| black_box(FitnessConst::<1, 2>::new()))
    });
}

fn bench_precision_7_8(c: &mut Criterion) {
    c.bench_function("PrecisionConst::<7,8>::new", |b| {
        b.iter(|| black_box(PrecisionConst::<7, 8>::new()))
    });
}

criterion_group!(
    benches,
    bench_condition_cell_8,
    bench_condition_cell_4,
    bench_condition_cell_1,
    bench_between01_3_4,
    bench_between01_1_2,
    bench_between01_1_1,
    bench_fitness_3_4,
    bench_fitness_1_2,
    bench_precision_7_8,
);
criterion_main!(benches);
