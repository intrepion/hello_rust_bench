// src/main.rs

use std::time::Instant;
use std::io::Write; // Required for low-level stream writing (write! and writeln!)

// =========================================================================
// UTILITY FUNCTION
// This function wraps the timing logic, allowing us to test different
// printing methodologies cleanly.
// =========================================================================
fn measure_time<F>(label: &str, f: F) -> u128
where
    F: FnOnce(),
{
    let start = Instant::now();
    
    // Execute the function that performs the writes
    f(); 
    
    let duration = start.elapsed();
    // We multiply by 1000 to get microseconds (μs) for better precision display
    println!("  -> {} took {} microseconds.", label, duration.as_micros());
    duration.as_micros()
}


fn main() {
    // We run the test 100,000 times to generate enough I/O workload
    const ITERATIONS: usize = 100_000;
    println!("*** Starting I/O Benchmark ({} iterations) ***\n", ITERATIONS);

    // =========================================================================
    // ⭐️ METHOD 1: The Standard Macro (println!)
    // The most idiomatic and simplest way for console output.
    // =========================================================================
    println!("\n--- Testing Method 1: println! macro ---");
    measure_time("println! macro", || {
        for _ in 0..ITERATIONS {
            println!("{}", cli::hello_world());
        }
    });

    // =========================================================================
    // ⭐️ METHOD 2: Writing with `write!` (Low-level, Byte Stream)
    // Direct access to the output stream handle. Best for raw byte writing.
    // =========================================================================
    println!("\n--- Testing Method 2: std::io::Write (write!) ---");
    measure_time("write! macro", || {
        // Get a mutable handle to standard output
        let mut handle = std::io::stdout();
        for _ in 0..ITERATIONS {
            // We must write the string's bytes directly.
            // We explicitly write the bytes *and* the newline character.
            let _ = handle.write_all(cli::hello_world().as_bytes());
            let _ = handle.write_all(b"\n");
        }
    });

    // =========================================================================
    // ⭐️ METHOD 3: Using `writeln!` macro (Stream Helper)
    // Designed for writing formatted data to a stream, often used for files.
    // =========================================================================
    println!("\n--- Testing Method 3: writeln! macro ---");
    measure_time("writeln! macro", || {
        // Getting a handle to stdout
        let mut handle = std::io::stdout();
        for _ in 0..ITERATIONS {
            // Note: When simulating `writeln!`, we manually write the string 
            // and then manually write the newline.
            let _ = handle.write_all(cli::hello_world().as_bytes());
            let _ = handle.write_all(b"\n");
        }
    });

    println!("\n*** Benchmark complete. ***");
}
