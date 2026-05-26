use hello_world::{
    hello_world, formatted_hello_world, print_hello_world, print_hello_world_function,
    eprint_hello_world, print_formatted_hello_world,
};

fn main() {
    // Call each function to demonstrate usage
    println!("--- Direct println! ---");
    print_hello_world();

    println!("\n--- println! with function ---");
    print_hello_world_function();

    println!("\n--- eprintln! ---");
    eprint_hello_world();

    println!("\n--- println! with format! ---");
    print_formatted_hello_world();

    println!("\n--- Returning strings ---");
    println!("{}", hello_world());
    println!("{}", formatted_hello_world());
}
