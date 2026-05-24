// src/lib.rs
// 6 Different approaches to creating "Hello, world!"

pub fn hello_world() -> &'static str {
    "Hello, world!"
}

pub fn hello_string() -> String {
    String::from("Hello, world!")
}

pub fn hello_format() -> String {
    format!("Hello, world!")
}

pub fn hello_concat() -> String {
    "Hello, ".to_string() + "world!"
}

pub fn hello_vec() -> String {
    vec!["Hello", ", ", "world!"].join("")
}

pub fn hello_write() -> String {
    use std::fmt::Write;
    let mut s = String::new();
    write!(s, "Hello, world!").unwrap();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }

    #[test]
    fn string_equals_static() {
        assert_eq!(hello_string(), "Hello, world!");
    }

    #[test]
    fn all_approaches_equal() {
        let static_str = hello_world();
        let from_string = hello_string();
        let format_string = hello_format();
        let concat_string = hello_concat();
        let vec_string = hello_vec();
        let write_string = hello_write();
        
        assert_eq!(from_string, "Hello, world!");
        assert_eq!(format_string, "Hello, world!");
        assert_eq!(concat_string, "Hello, world!");
        assert_eq!(vec_string, "Hello, world!");
        assert_eq!(write_string, "Hello, world!");
        assert_eq!(static_str, "Hello, world!");
    }
}
