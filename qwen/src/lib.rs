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
