use criterion::{black_box, criterion_group, criterion_main, Criterion};
use print_profiler::run_iterations;

fn bench_print_methods(c: &mut Criterion) {
    let iterations = 10_000;

    let mut group = c.benchmark_group("print_methods");
    group.sample_size(10); // reduce sample size for faster runs

    group.bench_function("println_macro", |b| {
        b.iter(|| run_iterations(black_box(0), black_box(iterations)))
    });
    group.bench_function("stdout_write", |b| {
        b.iter(|| run_iterations(black_box(1), black_box(iterations)))
    });
    group.bench_function("eprintln", |b| {
        b.iter(|| run_iterations(black_box(2), black_box(iterations)))
    });
    group.bench_function("print_macro", |b| {
        b.iter(|| run_iterations(black_box(3), black_box(iterations)))
    });

    group.finish();
}

criterion_group!(benches, bench_print_methods);
criterion_main!(benches);
