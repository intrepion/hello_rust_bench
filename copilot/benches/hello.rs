use criterion::{criterion_group, criterion_main, Criterion};
use your_crate::{hello_world, hello_format, hello_owned};

fn bench_hello_world(c: &mut Criterion) {
    c.bench_function("hello_world_static_str", |b| {
        b.iter(|| hello_world())
    });
}

fn bench_hello_format(c: &mut Criterion) {
    c.bench_function("hello_world_format", |b| {
        b.iter(|| hello_format())
    });
}

fn bench_hello_owned(c: &mut Criterion) {
    c.bench_function("hello_world_owned", |b| {
        b.iter(|| hello_owned())
    });
}

criterion_group!(
    benches,
    bench_hello_world,
    bench_hello_format,
    bench_hello_owned
);
criterion_main!(benches);
