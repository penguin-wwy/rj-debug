use super::native::jvmti::*;
use super::native::jni::*;
use super::native::jni_md::*;
use super::logger::*;
use super::config::*;
use super::writer::*;
use std::os::raw::c_char;
use std::ptr;
use std::ffi::CStr;

unsafe fn get_method_id(jvmti: *mut jvmtiEnv, jklass: jclass, name: &str) -> Option<jmethodID> {
    let mut count: jint = 0;
    let mut jmethods: *mut jmethodID = ptr::null_mut();
    assert_log(
        (**jvmti).GetClassMethods.unwrap()(jvmti, jklass, &count as *mut jint, &jmethods as *mut *mut jmethodID),
        Some("Get class methods failed..."),
        None
    );

    for i in 0..count as u32 {
        let method_name: *mut c_char = ptr::null_mut();
        assert_log(
            (**jvmti).GetMethodName.unwrap()(jvmti, jmethods[i], &method_name as *mut *mut c_char, ptr::null_mut(), ptr::null_mut()),
            Some("Get method name failed..."),
            None
        );
        if (CStr::from_ptr(method_name).to_str().unwrap().eq(name)) {
            return Some(jmethods[i]);
        }
    }
    return None;
}

pub fn set_break_point(jvmti: *mut jvmtiEnv, bk: &BreakPoint) {
    let method: jmethodID = match get_method_id() { }
    (**jvmti).GetLineNumberTable.unwrap()(jvmti, )
}

pub unsafe extern "C" fn event_class_prepare(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv,
                                             thread: jthread, klass: jclass) -> () {
    let mut class_name_native: *mut c_char = ptr::null_mut();

    assert_log(
        (**jvmti_env).GetClassSignature.unwrap()(jvmti_env, klass, &mut class_name_native as *mut *mut c_char, ptr::null_mut()),
        Some("GetClassSignature Error..."),
        None
    );
    assert!(!class_name_native.is_null());
    let class_name= String::from((CStr::from_ptr(class_name_native).to_str().unwrap()));
    if config().class_print {
        writer(format!("[class prepare] {}", class_name.as_str()).as_str());
    }

    if config().bytecode_dump {

    }

    for i in 0..breakpoint_size() {
        if class_name.eq(breakpoints(i).unwrap().get_class_name().unwrap()) {
            set_break_point(jvmti_env, breakpoints(i).unwrap());
        }
    }
}