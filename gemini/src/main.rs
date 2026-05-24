use std::io::{self, Write};

fn main() {
    let msg = cli::hello_world();

    // =========================================================================
    // APPROACH 1: Unbuffered, Macro-Driven Standard Output
    // This forces an internal mutex lock and string reallocation on every invocation.
    // =========================================================================
    println!("--- Starting Approach 1 (println!) ---");
    for _ in 0..500_000 {
        println!("{}", msg);
    }

    // =========================================================================
    // APPROACH 2: Manually Locked, Unbuffered Standard Output
    // This bypasses recurring runtime lock contention but still triggers frequent syscalls.
    // =========================================================================
    println!("--- Starting Approach 2 (stdout lock) ---");
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for _ in 0..500_000 {
        let _ = writeln!(handle, "{}", msg);
    }

    // =========================================================================
    // APPROACH 3: Explicitly Buffered, Manually Locked Output
    // This stores byte streams in memory, making a system call only when full.
    // =========================================================================
    println!("--- Starting Approach 3 (BufWriter) ---");
    let mut writer = io::BufWriter::new(io::stdout().lock());
    for _ in 0..500_000 {
        let _ = writeln!(writer, "{}", msg);
    }
    let _ = writer.flush(); // Ensure final remaining bytes clear cleanly
}
