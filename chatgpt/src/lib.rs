use std::hint::black_box;

pub fn hello_world() -> &'static str {
    "Hello, world!"
}

// Variant 1: format with function call
pub fn compute_v1() {
    let s = format!("{}", hello_world());
    black_box(s);
}

// Variant 2: direct string formatting
pub fn compute_v2() {
    let s = format!("Hello, world!");
    black_box(s);
}

// Variant 3: no formatting (baseline)
pub fn compute_v3() {
    let s = hello_world();
    black_box(s);
}

// Variant 4: allocation + push
pub fn compute_v4() {
    let mut s = String::new();
    s.push_str("Hello, world!");
    black_box(s);
}

// Amplify work
pub fn run_many(f: fn()) {
    for _ in 0..100_000 {
        f();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}
