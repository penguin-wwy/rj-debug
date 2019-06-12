use super::native::jvmti::*;
use super::native::jni::*;
use super::logger::*;
use super::config::*;
use super::writer::*;
use std::os::raw::c_char;
use std::ptr;
use std::ffi::CStr;

pub unsafe extern "C" fn event_class_prepare(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv,
                                             thread: jthread, klass: jclass) -> () {
    let mut class_name: *mut c_char = ptr::null_mut();

    assert_log(
        (**jvmti_env).GetClassSignature.unwrap()(jvmti_env, klass, &mut class_name as *mut *mut c_char, ptr::null_mut()),
        Some("GetClassSignature Error..."),
        None
    );
    if config().class_print {
        assert!(!class_name.is_null());
        writer(format!("[class prepare] {}", CStr::from_ptr(class_name).to_str().unwrap()).as_str())
    }
}