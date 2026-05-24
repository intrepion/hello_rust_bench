use std::io::{self, Write};

/// Uses println! macro - locks stdout every call
pub fn println_version() {
    println!("Hello, world!");
}

/// Uses print! + manual newline - still locks
pub fn print_version() {
    print!("Hello, world!\n");
}

/// Uses writeln! macro - locks once, writes formatted
pub fn writeln_stdout() {
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // lock once
    let _ = writeln!(handle, "Hello, world!");
}

/// Uses write_all - fastest, no formatting, locks once
pub fn write_all_stdout() {
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // lock once
    let _ = handle.write_all(b"Hello, world!\n");
}

/// Buffered version - best for loops
pub fn buffered_write() {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let _ = writeln!(handle, "Hello, world!");
    let _ = handle.flush();
}
