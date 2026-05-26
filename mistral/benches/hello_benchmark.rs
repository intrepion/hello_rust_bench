use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use hello_world::{
    hello_world, formatted_hello_world, print_hello_world, print_hello_world_function,
    eprint_hello_world, print_formatted_hello_world,
};

/// Benchmark the function that returns "Hello, world!".
fn bench_hello_world_function(c: &mut Criterion) {
    c.bench_function("hello_world_function", |b| {
        b.iter(|| black_box(hello_world()))
    });
}

/// Benchmark the function that returns a formatted string.
fn bench_formatted_hello_world_function(c: &mut Criterion) {
    c.bench_function("formatted_hello_world_function", |b| {
        b.iter(|| black_box(formatted_hello_world()))
    });
}

/// Benchmark different ways to print "Hello, world!".
fn bench_print_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("Print Methods");

    // Benchmark: Direct println! with a static string
    group.bench_function("println_static", |b| {
        b.iter(|| print_hello_world())
    });

    // Benchmark: println! with a function call
    group.bench_function("println_function", |b| {
        b.iter(|| print_hello_world_function())
    });

    // Benchmark: eprintln!
    group.bench_function("eprintln", |b| {
        b.iter(|| eprint_hello_world())
    });

    // Benchmark: println! with format!
    group.bench_function("format_println", |b| {
        b.iter(|| print_formatted_hello_world())
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10); // Reduce sample size for faster benchmarks
    targets = bench_hello_world_function, bench_formatted_hello_world_function, bench_print_methods
);
criterion_main!(benches);
