pub fn hello_println() {
    println!("Hello, world!");
}

pub fn hello_format() {
    println!("{}", "Hello, world!");
}

pub fn hello_write_all() {
    use std::io::Write;
    let _ = std::io::stdout().write_all(b"Hello, world!\n");
}

pub fn hello_write_fmt() {
    use std::io::Write;
    let _ = std::io::stdout().write_fmt(format_args!("Hello, world!\n"));
}

pub fn hello_eprint() {
    eprintln!("Hello, world!");
}

pub fn hello_string() {
    println!("{}", String::from("Hello, world!"));
}
