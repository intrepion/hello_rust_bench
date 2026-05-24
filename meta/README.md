How to use:

```bash
# 1. Create project structure
mkdir hello_bench && cd hello_bench
mkdir src benches

# 2. Add the 4 files above

# 3. Run once to see output
cargo run

# 4. Benchmark
cargo bench

# 5. View HTML report
open target/criterion/hello_world/report/index.html
# Linux: xdg-open target/criterion/hello_world/report/index.html

# 6. Profile why println! is slow
cargo install flamegraph
sudo cargo flamegraph --bench hello -- --bench
# Output: flamegraph.svg
```
