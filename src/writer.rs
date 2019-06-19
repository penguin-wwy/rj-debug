use std::process::exit;

pub fn writer(message: &str) {
    // TODO : output message to file or stdout
    println!("{}", message);
}

pub fn expect(message: &str, code: i32) {
    println!("{}", message);
    exit(code);
}