/// Returns the static string "Hello, world!".
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

/// Returns a formatted string.
pub fn formatted_hello_world() -> String {
    format!("Hello, {}!", "world")
}

/// Prints "Hello, world!" directly.
pub fn print_hello_world() {
    println!("Hello, world!");
}

/// Prints using the `hello_world` function.
pub fn print_hello_world_function() {
    println!("{}", hello_world());
}

/// Prints using `eprintln!`.
pub fn eprint_hello_world() {
    eprintln!("Hello, world!");
}

/// Prints using `format!` + `println!`.
pub fn print_formatted_hello_world() {
    println!("{}", formatted_hello_world());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }

    #[test]
    fn test_formatted_hello_world() {
        assert_eq!(formatted_hello_world(), "Hello, world!");
    }
}
