use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hello_profiling::hello_world;

// 1. Benchmark the pure function return (no I/O)
fn bench_string_return(c: &mut Criterion) {
    c.bench_function("return_hello_world", |b| {
        b.iter(|| {
            let msg = hello_world();
            black_box(msg); // Prevents compiler dead-code elimination
        })
    });
}

// 2. Benchmark string formatting overhead (isolates CPU cost of `format!`)
fn bench_formatting_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatting");

    group.bench_function("format_macro", |b| {
        b.iter(|| {
            let _ = format!("{}", black_box(hello_world()));
        })
    });

    group.bench_function("concat_macro", |b| {
        b.iter(|| {
            let _ = format!("{}{}", black_box(hello_world()), black_box("!"));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_string_return, bench_formatting_overhead);
criterion_main!(benches);
