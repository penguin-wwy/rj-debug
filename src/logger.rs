use super::native::jni_md::jint;
use super::native::jni::{JNI_OK, JNI_ERR};
use std::process::exit;
use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn info_log(message: *const c_char) {
    assert!(!message.is_null());
    let info_str = unsafe {
        CStr::from_ptr(message).to_str().unwrap()
    };
    info(info_str);
}

#[no_mangle]
pub unsafe extern "C" fn error_log(message: *const c_char) {
    assert!(!message.is_null());
    let err_str = unsafe {
        CStr::from_ptr(message).to_str().unwrap()
    };
    error(err_str)
}

#[no_mangle]
pub unsafe extern "C" fn warn_log(message: *const c_char) {
    assert!(!message.is_null());
    let warn_str = unsafe {
        CStr::from_ptr(message).to_str().unwrap()
    };
    warn(warn_str)
}

pub fn assert_log(res: jint, error_message: Option<&str>, succ_message: Option<&str>) {
    if (JNI_OK != res) {
        if (error_message.is_some()) {
            log::error!("{}", error_message.unwrap());
        }
        exit(JNI_ERR)
    } else {
        if (succ_message.is_some()) {
            log::info!("{}", succ_message.unwrap());
        }
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