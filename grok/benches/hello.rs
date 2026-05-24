use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hello_bench::*;

fn bench_hello_world(c: &mut Criterion) {
    let mut group = c.benchmark_group("hello_world_printing");

    group.bench_function("println! macro", |b| b.iter(|| hello_println()));
    group.bench_function("format! + println", |b| b.iter(|| hello_format()));
    group.bench_function("stdout.write_all", |b| b.iter(|| hello_write_all()));
    group.bench_function("stdout.write_fmt", |b| b.iter(|| hello_write_fmt()));
    group.bench_function("eprintln!", |b| b.iter(|| hello_eprint()));
    group.bench_function("String + println", |b| b.iter(|| hello_string()));

    group.finish();
}

criterion_group!(benches, bench_hello_world);
criterion_main!(benches);
