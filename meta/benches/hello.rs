use criterion::{criterion_group, criterion_main, Criterion};
use hello_bench::*;

fn bench_hello_world(c: &mut Criterion) {
    let mut group = c.benchmark_group("hello_world");

    // Redirect stdout so benchmarks don't spam terminal
    // Criterion captures this anyway, but keeps output clean
    group.bench_function("println!", |b| b.iter(|| {
        std::io::sink(); // discard output
        println_version();
    }));

    group.bench_function("print!+\\n", |b| b.iter(|| {
        std::io::sink();
        print_version();
    }));

    group.bench_function("writeln!+lock", |b| b.iter(|| {
        std::io::sink();
        writeln_stdout();
    }));

    group.bench_function("write_all+lock", |b| b.iter(|| {
        std::io::sink();
        write_all_stdout();
    }));

    group.bench_function("buffered", |b| b.iter(|| {
        std::io::sink();
        buffered_write();
    }));

    group.finish();
}

criterion_group!(benches, bench_hello_world);
criterion_main!(benches);
