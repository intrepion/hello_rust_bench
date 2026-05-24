use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hello_bench::{hello_static, hello_owned, hello_formatted, hello_concat};

/// Benchmark: Static String Slice
/// Expected: Near 0ns (optimized away or register move)
fn bench_static(c: &mut Criterion) {
    c.bench_function("static_str", |b| {
        b.iter(|| {
            // black_box prevents the compiler from optimizing the result away
            let result = hello_static();
            black_box(result)
        })
    });
}

/// Benchmark: Owned String (Heap Allocation)
/// Expected: ~15-25ns (depends on allocator)
fn bench_owned(c: &mut Criterion) {
    c.bench_function("owned_string", |b| {
        b.iter(|| {
            let result = hello_owned();
            black_box(result)
        })
    });
}

/// Benchmark: Format Macro
/// Expected: ~30-50ns (parsing + allocation)
fn bench_formatted(c: &mut Criterion) {
    c.bench_function("formatted", |b| {
        b.iter(|| {
            let result = hello_formatted();
            black_box(result)
        })
    });
}

/// Benchmark: Concatenation
/// Expected: ~40-60ns (multiple steps)
fn bench_concat(c: &mut Criterion) {
    c.bench_function("concat", |b| {
        b.iter(|| {
            let result = hello_concat();
            black_box(result)
        })
    });
}

// Group all benchmarks together
criterion_group!(
    benches,
    bench_static,
    bench_owned,
    bench_formatted,
    bench_concat
);

criterion_main!(benches);
