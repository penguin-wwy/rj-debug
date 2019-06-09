use super::native::jni_md::jint;
use super::native::jni::{JNI_OK, JNI_ERR};
use std::process::exit;

pub fn assert_log(res: jint, error_message: Option<&str>, succ_message: Option<&str>) {
    if (JNI_OK != res) {
        if (error_message.is_some()) {
            log::error!("{}", error_message.unwrap());
        }
        exit(JNI_ERR)
    } else {
        log::info!("{}", succ_message.unwrap());
    }
}

pub fn info(message: &str) {
    log::info!("{}", message);
}

pub fn warn(message: &str) {
    log::warn!("{}", message);
}

pub fn error(message: &str) {
    log::error!("{}", message);
}