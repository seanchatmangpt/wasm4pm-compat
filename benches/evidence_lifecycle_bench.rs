//! Benchmarks proving that `Evidence<T, State, W>` state transitions are
//! genuinely zero-cost — each `into_*` call is a struct-field move with no
//! extra branching or allocation.
//!
//! The lifecycle chain benchmarked is:
//!
//! ```text
//! Evidence::raw(T)          <- Raw
//!   .into_parsed()          <- Parsed
//!   (Admission::new(T)      <- the only path to Admitted)
//!   .into_evidence()        <- Admitted
//!   .into_projected()       <- Projected
//!   .into_receipted()       <- Receipted
//! ```
//!
//! The baseline is a raw `u64` identity function -- the benchmark latency
//! should be indistinguishable from the baseline if the zero-cost claim holds.
//!
//! Run with:
//! ```text
//! cargo bench --bench evidence_lifecycle_bench --all-features
//! ```

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::witness::Ocel20;

// ── Baseline ──────────────────────────────────────────────────────────────────

fn bench_u64_baseline(c: &mut Criterion) {
    c.bench_function("u64 identity baseline", |b| {
        b.iter(|| black_box(black_box(42u64)))
    });
}

// ── Raw construction ──────────────────────────────────────────────────────────

fn bench_evidence_raw(c: &mut Criterion) {
    c.bench_function("Evidence::raw(u64) -- Raw stage", |b| {
        b.iter(|| {
            let ev = Evidence::<u64, _, Ocel20>::raw(black_box(1u64));
            black_box(ev.value)
        })
    });
}

// ── Raw -> Parsed ─────────────────────────────────────────────────────────────

fn bench_evidence_raw_to_parsed(c: &mut Criterion) {
    c.bench_function("Evidence::raw -> into_parsed()", |b| {
        b.iter(|| {
            let parsed = Evidence::<u64, _, Ocel20>::raw(black_box(2u64)).into_parsed();
            black_box(parsed.value)
        })
    });
}

// ── Admission -> Admitted evidence ───────────────────────────────────────────

fn bench_admission_into_evidence(c: &mut Criterion) {
    c.bench_function("Admission::new -> into_evidence()", |b| {
        b.iter(|| {
            let admitted = Admission::<u64, Ocel20>::new(black_box(3u64)).into_evidence();
            black_box(admitted.value)
        })
    });
}

// ── Admitted -> Projected -> Receipted ───────────────────────────────────────

fn bench_admitted_to_receipted_via_projected(c: &mut Criterion) {
    c.bench_function("Admitted -> into_projected() -> into_receipted()", |b| {
        b.iter(|| {
            let receipted = Admission::<u64, Ocel20>::new(black_box(4u64))
                .into_evidence()
                .into_projected()
                .into_receipted();
            black_box(receipted.value)
        })
    });
}

// ── Full chain: Admitted -> Exportable -> Receipted ──────────────────────────

fn bench_full_lifecycle_exportable_path(c: &mut Criterion) {
    c.bench_function(
        "Admitted -> into_exportable() -> into_receipted() (full export path)",
        |b| {
            b.iter(|| {
                let receipted = Admission::<u64, Ocel20>::new(black_box(5u64))
                    .into_evidence()
                    .into_exportable()
                    .into_receipted();
                black_box(receipted.value)
            })
        },
    );
}

// ── Refuse path ──────────────────────────────────────────────────────────────

fn bench_evidence_refuse_path(c: &mut Criterion) {
    c.bench_function("Evidence::raw -> .refuse() (fast-reject path)", |b| {
        b.iter(|| {
            let refused = Evidence::<u64, _, Ocel20>::raw(black_box(0u64)).refuse();
            black_box(refused.into_refused_value())
        })
    });
}

criterion_group!(
    benches,
    bench_u64_baseline,
    bench_evidence_raw,
    bench_evidence_raw_to_parsed,
    bench_admission_into_evidence,
    bench_admitted_to_receipted_via_projected,
    bench_full_lifecycle_exportable_path,
    bench_evidence_refuse_path,
);
criterion_main!(benches);
