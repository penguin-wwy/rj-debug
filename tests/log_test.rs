#[macro_use]
extern crate log;
use log::LevelFilter::Info;
use simple_logging::log_to;

#[test]
fn log_to_file() {
    simple_logging::log_to_file("test.log", Info);
    info!("test");
    warn!("warn test");
    debug!("debug test");
}

#[test]
fn log_consle() {
    log_to(std::io::stdout(), Info);
    info!("test!");
}