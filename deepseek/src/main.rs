use print_profiler::run_iterations;
use std::time::Instant;

const ITERATIONS: usize = 1_000_000;

fn main() {
    println!("Profiling different print methods ({} iterations each)\n", ITERATIONS);

    let methods = [
        ("println! macro (stdout, buffered)", 0),
        ("stdout.write_all (manual)", 1),
        ("eprintln! (stderr, unbuffered)", 2),
        ("print! macro (stdout)", 3),
    ];

    for (name, method) in methods {
        let start = Instant::now();
        run_iterations(method, ITERATIONS);
        let duration = start.elapsed();
        println!("{:<40} took {:?}", name, duration);
    }

    println!("\nProfiling complete. Run with `cargo flamegraph` to see detailed call stacks.");
}
