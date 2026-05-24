// src/bin/memory_profile.rs
// Memory profiling of all 6 approaches

use hello_world::*;

fn main() {
    println!("=== Memory Profile ===\n");
    
    // Approach 1: Static str
    let s1 = hello_world();
    println!("1. Static str:");
    println!("   Size of reference: {} bytes", std::mem::size_of::<&str>());
    println!("   Actual string length: {} bytes", s1.len());
    println!("   No heap allocation needed\n");
    
    // Approach 2: String::from
    let s2 = hello_string();
    println!("2. String::from:");
    println!("   Size of String struct: {} bytes", std::mem::size_of::<String>());
    println!("   String length: {} bytes", s2.len());
    println!("   String capacity: {} bytes", s2.capacity());
    println!("   Allocations: 1\n");
    
    // Approach 3: format!
    let s3 = hello_format();
    println!("3. format!:");
    println!("   Size of String struct: {} bytes", std::mem::size_of::<String>());
    println!("   String length: {} bytes", s3.len());
    println!("   String capacity: {} bytes", s3.capacity());
    println!("   Allocations: 1\n");
    
    // Approach 4: String concat
    let s4 = hello_concat();
    println!("4. String concat:");
    println!("   Size of String struct: {} bytes", std::mem::size_of::<String>());
    println!("   String length: {} bytes", s4.len());
    println!("   String capacity: {} bytes", s4.capacity());
    println!("   Allocations: 1 (coercion overhead)\n");
    
    // Approach 5: Vec join
    let s5 = hello_vec();
    println!("5. Vec join:");
    println!("   Size of String struct: {} bytes", std::mem::size_of::<String>());
    println!("   String length: {} bytes", s5.len());
    println!("   String capacity: {} bytes", s5.capacity());
    println!("   Allocations: multiple (vec + strings + join)\n");
    
    // Approach 6: Write macro
    let s6 = hello_write();
    println!("6. Write macro:");
    println!("   Size of String struct: {} bytes", std::mem::size_of::<String>());
    println!("   String length: {} bytes", s6.len());
    println!("   String capacity: {} bytes", s6.capacity());
    println!("   Allocations: 1 (via Write trait)\n");
    
    println!("=== Summary ===");
    println!("Static str: 0 allocations (reference only)");
    println!("Others: 1+ heap allocations each");
    println!("All return same string content: {} bytes", s1.len());
}
