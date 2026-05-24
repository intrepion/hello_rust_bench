Executiion commands to run the profiler:

```bash
# 1. Compile the workspace using your specialized tracking profile configuration
cargo build --profile profiling

# 2. Grant permissions for non-privileged sampling (Required on Linux architectures)
echo '1' | sudo tee /proc/sys/kernel/perf_event_paranoid

cargo install samply
samply record ./target/profiling/cli > /dev/null
```
