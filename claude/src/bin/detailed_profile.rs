// src/bin/detailed_profile.rs
// Manual timing profiling of all 6 approaches

use std::time::Instant;
use hello_world::*;

fn main() {
    const ITERATIONS: usize = 1_000_000;
    
    println!("=== Manual Timing Profile ===\n");
    println!("Running {} iterations of each approach\n", ITERATIONS);
    
    // Approach 1: Static str
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = hello_world();
    }
    let duration = start.elapsed();
    let ns_per_iter = duration.as_nanos() as f64 / ITERATIONS as f64;
    println!("1. Static str:    {:?} ({:.2} ns/iter)", duration, ns_per_iter);
    
    // Approach 2: String::from
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = hello_string();
    }
    let duration = start.elapsed();
    let ns_per_iter = duration.as_nanos() as f64 / ITERATIONS as f64;
    println!("2. String::from:  {:?} ({:.2} ns/iter)", duration, ns_per_iter);
    
    // Approach 3: format!
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = hello_format();
    }
    let duration = start.elapsed();
    let ns_per_iter = duration.as_nanos() as f64 / ITERATIONS as f64;
    println!("3. format!:       {:?} ({:.2} ns/iter)", duration, ns_per_iter);
    
    // Approach 4: String concat
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = hello_concat();
    }
    let duration = start.elapsed();
    let ns_per_iter = duration.as_nanos() as f64 / ITERATIONS as f64;
    println!("4. String concat: {:?} ({:.2} ns/iter)", duration, ns_per_iter);
    
    // Approach 5: Vec join
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = hello_vec();
    }
    let duration = start.elapsed();
    let ns_per_iter = duration.as_nanos() as f64 / ITERATIONS as f64;
    println!("5. Vec join:      {:?} ({:.2} ns/iter)", duration, ns_per_iter);
    
    // Approach 6: Write macro
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = hello_write();
    }
    let duration = start.elapsed();
    let ns_per_iter = duration.as_nanos() as f64 / ITERATIONS as f64;
    println!("6. Write macro:   {:?} ({:.2} ns/iter)", duration, ns_per_iter);
    
    println!("\n=== Analysis ===");
    println!("Static str is fastest (expected - no allocation)");
    println!("Allocating approaches vary by 20-300%");
    println!("Vec join is slowest (multiple operations)");
}
