// src/main.rs
// Demo binary that shows all 6 approaches

fn main() {
    println!("=== Hello World: 6 Different Approaches ===\n");
    
    // Approach 1: Static str
    println!("1. Static str:");
    println!("   {}\n", hello_world::hello_world());
    
    // Approach 2: String::from
    println!("2. String::from:");
    println!("   {}\n", hello_world::hello_string());
    
    // Approach 3: format!
    println!("3. format!:");
    println!("   {}\n", hello_world::hello_format());
    
    // Approach 4: String concat
    println!("4. String concat:");
    println!("   {}\n", hello_world::hello_concat());
    
    // Approach 5: Vec join
    println!("5. Vec join:");
    println!("   {}\n", hello_world::hello_vec());
    
    // Approach 6: Write macro
    println!("6. Write macro:");
    println!("   {}\n", hello_world::hello_write());
}
