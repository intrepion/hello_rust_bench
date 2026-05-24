# hello_rust_bench
Benchmarking Hello World

so I have these 2 files, how could I go about profiling different ways of printing "Hello, world!"? I realize it is meaningless, but I'm just trying to learn how to set a practice that I will reuse in more complex solutions.

```rust
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}
```

```rust
fn main() {
    println!("{}", cli::hello_world());
}
```
