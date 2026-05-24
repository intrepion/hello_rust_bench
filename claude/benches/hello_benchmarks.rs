// benches/hello_benchmarks.rs
// Criterion benchmarks for all 6 approaches

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use hello_world::*;

// Benchmark 1: Just the function calls (no println)
fn bench_function_calls(c: &mut Criterion) {
    let mut group = c.benchmark_group("function_calls");
    
    group.bench_function("01_static_str", |b| {
        b.iter(|| {
            black_box(hello_world())
        });
    });
    
    group.bench_function("02_string_from", |b| {
        b.iter(|| {
            black_box(hello_string())
        });
    });
    
    group.bench_function("03_format_macro", |b| {
        b.iter(|| {
            black_box(hello_format())
        });
    });
    
    group.bench_function("04_string_concat", |b| {
        b.iter(|| {
            black_box(hello_concat())
        });
    });
    
    group.bench_function("05_vec_join", |b| {
        b.iter(|| {
            black_box(hello_vec())
        });
    });
    
    group.bench_function("06_write_macro", |b| {
        b.iter(|| {
            black_box(hello_write())
        });
    });
    
    group.finish();
}

// Benchmark 2: Including println
fn bench_with_println(c: &mut Criterion) {
    let mut group = c.benchmark_group("with_println");
    
    group.bench_function("01_static_str_println", |b| {
        b.iter(|| {
            let result = black_box(hello_world());
            println!("{}", result);
        });
    });
    
    group.bench_function("02_string_println", |b| {
        b.iter(|| {
            let result = black_box(hello_string());
            println!("{}", result);
        });
    });
    
    group.bench_function("03_format_println", |b| {
        b.iter(|| {
            let result = black_box(hello_format());
            println!("{}", result);
        });
    });
    
    group.finish();
}

// Benchmark 3: Allocation only (not printing)
fn bench_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocations");
    
    group.bench_function("01_static_str_no_alloc", |b| {
        b.iter(|| {
            black_box(hello_world());
        });
    });
    
    group.bench_function("02_string_with_alloc", |b| {
        b.iter(|| {
            let s = black_box(hello_string());
            black_box(s);
        });
    });
    
    group.bench_function("03_vec_with_alloc", |b| {
        b.iter(|| {
            let s = black_box(hello_vec());
            black_box(s);
        });
    });
    
    group.finish();
}

// Benchmark 4: Different iteration counts
fn bench_iterations(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterations");
    
    for i in [1, 10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(i), i, |b, &i| {
            b.iter(|| {
                for _ in 0..i {
                    black_box(hello_world());
                }
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, 
    bench_function_calls, 
    bench_with_println, 
    bench_allocations,
    bench_iterations
);
criterion_main!(benches);
