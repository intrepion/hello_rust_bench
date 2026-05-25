🚀 How to Use This Practice

Initialize & Run:

```bash
cargo init --name hello_profiling
# Replace files with the ones above
cargo bench
```

View Results:

Open target/criterion/report/index.html in your browser. You'll see statistical comparisons, confidence intervals, and performance trends.

Profile Hot Paths:

```bash
cargo install cargo-instruments
just profile
# Opens a flamegraph in your browser showing exactly which functions consume CPU time
```

Scale to Real Projects:

Replace hello_world() with your actual business logic (DB query, JSON parse, auth check, etc.)
Keep black_box() around return values
Never benchmark println!, std::fs::write, or network calls directly; benchmark the preparation logic instead
Add just bench to your CI pipeline to block PRs on regressions >5%
