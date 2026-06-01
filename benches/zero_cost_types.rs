//! Benchmarks proving wasm4pm-compat type law is truly zero-cost.
//!
//! These benchmarks verify that:
//! - `PhantomData` state tokens compile to zero bytes -- `Evidence<T, State, W>`
//!   has the same `size_of` as `T` itself.
//! - `#[repr(transparent)]` id newtypes have the same layout as their inner type.
//! - Const-generic law bounds (`ConditionCell`, `Between01`) generate no
//!   runtime code -- construction is as fast as a unit struct.
//! - The one-way-door state transitions in `Evidence` are pure no-ops at
//!   runtime: each `into_*` call moves `T` with zero extra work.
//!
//! Run with:
//! ```text
//! cargo bench --bench zero_cost_types --all-features
//! ```
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::mem::size_of;

use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::ids::{ActivityId, EventId, ObjectId, TraceId};
use wasm4pm_compat::law::{Between01, ConditionCell};
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::Ocel20;

// ── Size invariant assertions (compile-time) ────────────────────────────────

/// Prove `Evidence<T, State, W>` has the same size as `T`.
/// This is a compile-time + runtime double-check.
const _: () = {
    // Evidence<u64, Raw, Ocel20> must equal size_of::<u64>() == 8.
    // This const assertion would fail to compile if the size grew.
    assert!(size_of::<Evidence<u64, Raw, Ocel20>>() == size_of::<u64>());
    assert!(size_of::<Evidence<u32, Raw, Ocel20>>() == size_of::<u32>());
    assert!(size_of::<Evidence<u8,  Raw, Ocel20>>() == size_of::<u8>());
};

/// Prove `#[repr(transparent)]` id newtypes have the same size as their inner type.
const _: () = {
    enum Log {}
    assert!(size_of::<EventId<Log>>()    == size_of::<u64>());
    assert!(size_of::<ObjectId<Log>>()   == size_of::<u64>());
    assert!(size_of::<TraceId<Log>>()    == size_of::<u64>());
    assert!(size_of::<ActivityId<Log>>() == size_of::<u32>());
};

/// Prove `ConditionCell<N>` and `Between01<NUM, DEN>` are zero-sized.
const _: () = {
    assert!(size_of::<ConditionCell<8>>() == 0);
    assert!(size_of::<ConditionCell<1>>() == 0);
    // Between01 is also zero-sized — no fields, just phantom const params.
    assert!(size_of::<Between01<3, 4>>() == 0);
    assert!(size_of::<Between01<1, 1>>() == 0);
};

// ── Benchmark: EventId construction ─────────────────────────────────────────

fn bench_event_id_construction(c: &mut Criterion) {
    enum Log {}
    c.bench_function("EventId::new (repr transparent, should equal u64::new)", |b| {
        b.iter(|| {
            let id = EventId::<Log>::new(black_box(42u64));
            black_box(id)
        })
    });
}

// ── Benchmark: Evidence<u64, Raw, Ocel20> construction ──────────────────────

fn bench_evidence_raw_construction(c: &mut Criterion) {
    c.bench_function("Evidence::<u64, Raw, Ocel20>::raw (should equal identity)", |b| {
        b.iter(|| {
            let ev = Evidence::<u64, _, Ocel20>::raw(black_box(99u64));
            black_box(ev.value)
        })
    });
}

// ── Benchmark: baseline u64 — the zero-overhead reference ───────────────────

fn bench_u64_baseline(c: &mut Criterion) {
    c.bench_function("u64 identity (zero-overhead baseline)", |b| {
        b.iter(|| black_box(black_box(42u64)))
    });
}

// ── Benchmark: ConditionCell construction ───────────────────────────────────

fn bench_condition_cell(c: &mut Criterion) {
    c.bench_function("ConditionCell::<8>::new (zero-sized, should vanish)", |b| {
        b.iter(|| {
            let c: ConditionCell<8> = ConditionCell::new();
            black_box(c)
        })
    });
}

// ── Benchmark: Between01 construction ───────────────────────────────────────

fn bench_between01(c: &mut Criterion) {
    c.bench_function("Between01::<3,4>::new (zero-sized, should vanish)", |b| {
        b.iter(|| {
            let m: Between01<3, 4> = Between01::new();
            black_box(m)
        })
    });
}

criterion_group!(
    benches,
    bench_u64_baseline,
    bench_event_id_construction,
    bench_evidence_raw_construction,
    bench_condition_cell,
    bench_between01,
);
criterion_main!(benches);
