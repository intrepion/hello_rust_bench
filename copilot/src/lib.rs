pub fn hello_world() -> &'static str {
    "Hello, world!"
}

pub fn hello_format() -> String {
    format!("Hello, world!")
}

pub fn hello_owned() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}
