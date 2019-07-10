use std::process::exit;
use std::sync::Mutex;
use std::io::Write;
use std::path::Path;

lazy_static! {
    static ref RESULT_WRITER: SimpleWriter = SimpleWriter {
        inner: Mutex::new(None),
    };
}

struct SimpleWriter {
    inner: Mutex<Option<SimpleWriterInner>>
}

impl SimpleWriter {
    fn renew<T: Write + Send + 'static>(&self, sink: T) {
        *self.inner.lock().unwrap() = Some(SimpleWriterInner{
            sink: Box::new(sink),
        });
    }
}

struct SimpleWriterInner {
    sink: Box<Write + Send>,
}

impl SimpleWriterInner {
    fn write(&mut self, message: &str) {
        let _ = write!(self.sink, "{}", message);
    }
}

pub fn result_to_file<T: AsRef<Path>> (path: T) -> std::io::Result<()>{
    let result_file = std::fs::File::create(path)?;
    result_to(result_file);
    Ok(())
}

pub fn result_to<T: Write + Send + 'static>(sink: T) {
    RESULT_WRITER.renew(sink);
}

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