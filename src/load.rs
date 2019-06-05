use super::native::jni_md::*;
use super::native::jni::*;
use super::native::jvmti::*;
use super::config::*;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::ptr;

#[no_mangle]
pub extern "C" fn Agent_OnLoad(vm: *mut JavaVM, opts: *mut c_char, reserved: *mut c_void) -> jint {
    let mut null_ptr = ptr::null_mut() as *mut c_void;
    let jvmti_ptr = &mut null_ptr as *mut *mut c_void;
    if JNI_OK != unsafe {(**vm).GetEnv.unwrap()(vm, jvmti_ptr, JVMTI_VERSION_1_2)} {
        println!("ERROR: Unable to access JVMTI.");
        return JNI_ERR;
    }
    let jvmti = unsafe {(*jvmti_ptr) as *mut jvmtiEnv};
    return on_load(vm, jvmti, opts);
}

pub extern fn on_load(jvm: *mut JavaVM, jvmti: *mut jvmtiEnv, opts: *const c_char) -> jint {
    let c_str = unsafe {
        assert!(!opts.is_null());
        CStr::from_ptr(opts)
    };
    let mut file = File::open(c_str.to_str().unwrap()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let b = BreakPoint::new_from_str(data.as_str());
    println!("{}", b);
    return JNI_OK;
}