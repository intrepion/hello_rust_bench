use iai_callgrind::{library_benchmark, library_benchmark_group, main};

#[library_benchmark]
#[bench::static_str(cli::hello_world())]
fn bench_return(s: &'static str) -> &'static str {
    s
}

library_benchmark_group!(name = hello_group; benchmarks = bench_return);
main!(library_benchmark_groups = hello_group);
