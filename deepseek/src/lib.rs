/// Print using println! macro (stdout, buffered)
pub fn print_stdout_macro() {
    println!("Hello, world!");
}

/// Print using write! to stdout (manual flush not called)
pub fn print_stdout_write() -> std::io::Result<()> {
    use std::io::Write;
    std::io::stdout().write_all(b"Hello, world!\n")
}

/// Print using eprintln! (stderr, unbuffered)
pub fn print_eprintln() {
    eprintln!("Hello, world!");
}

/// Print using print! macro with explicit newline
pub fn print_format_macro() {
    print!("{}\n", "Hello, world!");
}

/// Run a chosen print method many times for profiling
#[inline(never)]
pub fn run_iterations(method: usize, iterations: usize) {
    for _ in 0..iterations {
        match method {
            0 => print_stdout_macro(),
            1 => { let _ = print_stdout_write(); },
            2 => print_eprintln(),
            3 => print_format_macro(),
            _ => (),
        }
    }
}

/// A more realistic workload: printing formatted numbers (to avoid being optimized out)
#[inline(never)]
pub fn print_numbers(iterations: usize) {
    for i in 0..iterations {
        println!("Number: {}", i);
    }
}
