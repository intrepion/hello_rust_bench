📊 Running the Benchmarks
To run the benchmarks, use:

```bash
cargo bench
```

Expected Output:

```text
hello_world_function          time:   [1.2345 ns 1.2350 ns 1.2356 ns]
formatted_hello_world_function time:   [12.345 ns 12.350 ns 12.356 ns]
Print Methods/println_static    time:   [123.45 µs 123.50 µs 123.56 µs]
Print Methods/println_function  time:   [124.56 µs 124.60 µs 124.65 µs]
Print Methods/eprintln          time:   [125.67 µs 125.70 µs 125.75 µs]
Print Methods/format_println    time:   [126.78 µs 126.80 µs 126.85 µs]
```

🔥 Profiling on macOS

📌 Option 1: CPU Profiling with cargo-flamegraph (Recommended)
cargo-flamegraph is the easiest way to generate flamegraphs on macOS.

Step 1: Install cargo-flamegraph

```bash
cargo install flamegraph
```

Step 2: Profile Your Code

Run your program with cargo flamegraph:

```bash
cargo flamegraph --bench hello_benchmark -- --bench
```

This will:

Build your benchmark in release mode.
Run it with CPU profiling.
Generate a flamegraph.svg file.

Step 3: Open the Flamegraph

```bash
open flamegraph.svg
```

This will open the flamegraph in your default browser.

📌 Option 2: CPU Profiling with dtrace (Advanced)
If you prefer low-level profiling, you can use dtrace. However, dtrace is restricted on newer macOS versions due to SIP. If it doesn’t work, skip to Option 3 (Instruments.app).

Step 1: Check if dtrace is Available

```bash
dtrace -V
```

If you see a version, it’s available. If not, SIP is blocking it.

Step 2: Record a Profile with dtrace

```bash
sudo dtrace -n 'profile-997 /execname == "hello_world"/ { @[ustack()] = count(); }' -o dtrace.out
```

Then run your program in another terminal:

```bash
./target/release/hello_world

After the program finishes, stop dtrace with Ctrl+C.

Step 3: Generate a Flamegraph

```bash
./FlameGraph/stackcollapse.pl dtrace.out | ./FlameGraph/flamegraph.pl > flamegraph.svg
open flamegraph.svg
```

📌 Option 3: CPU Profiling with Instruments.app (GUI)
Instruments is a powerful GUI tool for profiling CPU, memory, and more.

Step 1: Build Your Program in Release Mode

```bash
cargo build --release
```

Step 2: Open Instruments.app

Open Instruments.app (from /Applications/Xcode.app/Contents/Applications/).
Select Time Profiler (for CPU profiling).
Click the red record button and select your binary (./target/release/hello_world).
Run your program and interact with it (e.g., trigger the hello_world functions).
Stop recording and analyze the call stack.

Step 3: Analyze the Results

You’ll see a timeline of CPU usage.
Drill down into functions to see where time is spent.

📌 Option 4: Memory Profiling with lldb

lldb is a debugger that can also help with memory analysis.

Step 1: Build with Debug Symbols

```bash
cargo build
```

Step 2: Run with lldb

```bash
lldb ./target/debug/hello_world
```

Inside lldb, run:

```bash
(lldb) run
```

This will run your program. If it crashes or has memory issues, lldb can help debug them.

Step 3: Check for Memory Leaks

lldb isn’t as powerful as valgrind for memory leaks, but you can use Instruments.app for this:

Open Instruments.app.
Select Allocations or Leaks template.
Profile your program to check for memory issues.

📌 Option 5: Memory Profiling with heaptrack (Alternative)

heaptrack is a memory profiler for Linux, but it doesn’t work on macOS. Instead, use Instruments.app (as described above) or lldb.
 
