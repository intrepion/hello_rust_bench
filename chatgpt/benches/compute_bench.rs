use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cli::*;

fn bench_compute(c: &mut Criterion) {
    c.bench_function("compute_v1_format_fn", |b| {
        b.iter(|| run_many(black_box(compute_v1)))
    });

    c.bench_function("compute_v2_format_literal", |b| {
        b.iter(|| run_many(black_box(compute_v2)))
    });

    c.bench_function("compute_v3_static", |b| {
        b.iter(|| run_many(black_box(compute_v3)))
    });

    c.bench_function("compute_v4_push_str", |b| {
        b.iter(|| run_many(black_box(compute_v4)))
    });
}

criterion_group!(benches, bench_compute);
criterion_main!(benches);
