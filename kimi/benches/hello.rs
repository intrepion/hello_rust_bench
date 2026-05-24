use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("hello_output");
    
    group.bench_function("println", |b| {
        b.iter(|| {
            println!("{}", black_box(cli::hello_world()));
        })
    });
    
    group.bench_function("format_to_string", |b| {
        b.iter(|| {
            let _s = format!("{}", black_box(cli::hello_world()));
        })
    });
    
    group.bench_function("write_to_stack", |b| {
        use std::io::Write;
        let mut buf = [0u8; 64];
        b.iter(|| {
            let mut cursor = std::io::Cursor::new(&mut buf[..]);
            let _ = write!(cursor, "{}", black_box(cli::hello_world()));
        })
    });
    
    group.bench_function("return_static_str", |b| {
        b.iter(|| {
            let _s = black_box(cli::hello_world());
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
