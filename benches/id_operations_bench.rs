use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_id_operations(c: &mut Criterion) {
    c.bench_function("id_operations_noop", |b| b.iter(|| black_box(())));
}

criterion_group!(benches, bench_id_operations);
criterion_main!(benches);
