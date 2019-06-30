use std::process::exit;

pub fn writer(message: &str) {
    // TODO : output message to file or stdout
    println!("{}", message);
}

pub fn expect(message: &str, code: i32) {
    println!("{}", message);
    exit(code);
}

pub fn write_str_vec(data: &Vec<String>, range: usize) {
    let mut index: usize = 0;
    while ((index + 1) * range) < data.len() {
        let start = index * range;
        let end = (index + 1) * range;
        index += 1;
        writer(format!("{:?}", data[start..end].join(" ")).as_str());
    }
    writer(format!("{:?}", data[(index * range)..].join(" ")).as_str());
}