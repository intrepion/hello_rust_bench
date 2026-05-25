fn main() {
    // Keep the loop so `cargo instruments` has enough CPU time to sample
    for _ in 0..1_000_000 {
        std::hint::black_box(hello_profiling::hello_world());
    }

    println!("{}", hello_profiling::hello_world());
}
