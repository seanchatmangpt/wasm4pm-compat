//! Benchmarks proving that `#[repr(transparent)]` id newtypes have zero
//! overhead compared to operating on raw primitives.
//!
//! Every typed id in `wasm4pm_compat::ids` is a single-field `repr(transparent)`
//! newtype with a zero-sized `PhantomData<K>` kind marker. The compiler lays
//! out `EventId<Log>` identically to `u64` — there is no indirection, no vtable,
//! no extra word. These benchmarks compare:
//!
//! - Raw `u64`/`u32` construction and display (the baseline)
//! - `EventId`, `ObjectId`, `TraceId`, `CaseId` construction (u64-backed)
//! - `ActivityId`, `RelationId` construction (u32-backed)
//! - `Display` formatting on typed ids vs raw primitives
//!
//! If `repr(transparent)` is working correctly, typed id operations should
//! be statistically indistinguishable from the raw-primitive baseline.
//!
//! Run with:
//! ```text
//! cargo bench --bench id_operations_bench --all-features
//! ```

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use wasm4pm_compat::ids::{ActivityId, CaseId, EventId, ObjectId, RelationId, TraceId};

enum Log {}

// ── Baseline: raw primitives ──────────────────────────────────────────────────

fn bench_u64_construct(c: &mut Criterion) {
    c.bench_function("u64 construct (raw baseline)", |b| {
        b.iter(|| black_box(black_box(42u64)))
    });
}

fn bench_u32_construct(c: &mut Criterion) {
    c.bench_function("u32 construct (raw baseline)", |b| {
        b.iter(|| black_box(black_box(42u32)))
    });
}

// ── u64-backed typed ids ──────────────────────────────────────────────────────

fn bench_event_id_new(c: &mut Criterion) {
    c.bench_function("EventId::<Log>::new(u64)", |b| {
        b.iter(|| black_box(EventId::<Log>::new(black_box(42u64))))
    });
}

fn bench_object_id_new(c: &mut Criterion) {
    c.bench_function("ObjectId::<Log>::new(u64)", |b| {
        b.iter(|| black_box(ObjectId::<Log>::new(black_box(42u64))))
    });
}

fn bench_trace_id_new(c: &mut Criterion) {
    c.bench_function("TraceId::<Log>::new(u64)", |b| {
        b.iter(|| black_box(TraceId::<Log>::new(black_box(42u64))))
    });
}

fn bench_case_id_new(c: &mut Criterion) {
    c.bench_function("CaseId::<Log>::new(u64)", |b| {
        b.iter(|| black_box(CaseId::<Log>::new(black_box(42u64))))
    });
}

// ── u32-backed typed ids ──────────────────────────────────────────────────────

fn bench_activity_id_new(c: &mut Criterion) {
    c.bench_function("ActivityId::<Log>::new(u32)", |b| {
        b.iter(|| black_box(ActivityId::<Log>::new(black_box(7u32))))
    });
}

fn bench_relation_id_new(c: &mut Criterion) {
    c.bench_function("RelationId::<Log>::new(u32)", |b| {
        b.iter(|| black_box(RelationId::<Log>::new(black_box(7u32))))
    });
}

// ── Raw extraction (.raw()) ───────────────────────────────────────────────────

fn bench_event_id_raw(c: &mut Criterion) {
    let id = EventId::<Log>::new(99u64);
    c.bench_function("EventId::raw() extraction", |b| {
        b.iter(|| black_box(black_box(id).raw()))
    });
}

fn bench_activity_id_raw(c: &mut Criterion) {
    let id = ActivityId::<Log>::new(3u32);
    c.bench_function("ActivityId::raw() extraction", |b| {
        b.iter(|| black_box(black_box(id).raw()))
    });
}

// ── Display formatting ────────────────────────────────────────────────────────

fn bench_u64_display(c: &mut Criterion) {
    c.bench_function("u64 Display format (raw baseline)", |b| {
        b.iter(|| {
            let s = format!("{}", black_box(42u64));
            black_box(s)
        })
    });
}

fn bench_event_id_display(c: &mut Criterion) {
    let id = EventId::<Log>::new(42u64);
    c.bench_function("EventId Display format", |b| {
        b.iter(|| {
            let s = format!("{}", black_box(id));
            black_box(s)
        })
    });
}

// ── From<u64> / Into<u64> conversions ────────────────────────────────────────

fn bench_event_id_from(c: &mut Criterion) {
    c.bench_function("EventId::<Log>::from(u64) (From trait)", |b| {
        b.iter(|| {
            let id: EventId<Log> = EventId::from(black_box(55u64));
            black_box(id)
        })
    });
}

fn bench_event_id_into_u64(c: &mut Criterion) {
    let id = EventId::<Log>::new(55u64);
    c.bench_function("u64::from(EventId) (Into trait)", |b| {
        b.iter(|| {
            let raw: u64 = u64::from(black_box(id));
            black_box(raw)
        })
    });
}

criterion_group!(
    benches,
    bench_u64_construct,
    bench_u32_construct,
    bench_event_id_new,
    bench_object_id_new,
    bench_trace_id_new,
    bench_case_id_new,
    bench_activity_id_new,
    bench_relation_id_new,
    bench_event_id_raw,
    bench_activity_id_raw,
    bench_u64_display,
    bench_event_id_display,
    bench_event_id_from,
    bench_event_id_into_u64,
);
criterion_main!(benches);
