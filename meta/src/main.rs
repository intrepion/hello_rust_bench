use hello_bench::*;

fn main() {
    println!("Running each version once:");
    println_version();
    print_version();
    writeln_stdout();
    write_all_stdout();
    buffered_write();
}
