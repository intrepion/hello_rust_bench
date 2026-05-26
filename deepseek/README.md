# Print Profiler

Learn profiling in Rust by comparing different ways to print "Hello, world!".

## ✅ macOS Profiling Solution

### ✅ Install and use samply

```bash
cargo install samply
```

Then run your program under samply:

```bash
samply record ./target/release/print-profiler
```

This will open a Firefox Profiler UI in your browser automatically. You'll see a flamegraph of all function calls, including print_stdout_macro, print_eprintln, and the underlying write syscalls.

Note: The program still prints a million lines to the terminal, which might be slow. To only profile CPU (not terminal I/O), redirect stdout to /dev/null:

```bash
samply record ./target/release/print-profiler > /dev/null
```

Now you'll see time spent in the print logic, not in scrolling the terminal.

## Run benchmarks (optional)

```bash
cargo bench
```

