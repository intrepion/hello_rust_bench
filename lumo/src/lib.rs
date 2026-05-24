/// Variation 1: Static string slice (Zero allocation, fastest)
pub fn hello_static() -> &'static str {
    "Hello, world!"
}

/// Variation 2: Owned String (Heap allocation)
pub fn hello_owned() -> String {
    "Hello, world!".to_string()
}

/// Variation 3: Formatted string (Parsing + Allocation)
pub fn hello_formatted() -> String {
    format!("Hello, {}!", "world")
}

/// Variation 4: Concatenation (Multiple allocations)
pub fn hello_concat() -> String {
    let part1 = "Hello";
    let part2 = ", world!";
    format!("{}{}", part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hello_world() {
        assert_eq!(hello_static(), "Hello, world!");
        assert_eq!(hello_owned(), "Hello, world!");
        assert_eq!(hello_formatted(), "Hello, world!");
        assert_eq!(hello_concat(), "Hello, world!");
    }
}
