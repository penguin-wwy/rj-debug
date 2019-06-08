use super::native::jni_md::*;
use super::native::jni::*;
use super::native::jvmti::*;
use super::config::*;
use super::wrapper::*;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::ptr;

fn parse_config_file(path: *mut c_char) -> Option<Configuration> {
    let path_name = unsafe {
        assert!(!path.is_null());
        CStr::from_ptr(path)
    };
    let mut file = File::open(path_name.to_str().unwrap())
        .expect("config file open failed!");
    let mut content = String::new();
    if let Ok(_) = file.read_to_string(&mut content) {
        Some(Configuration::new_from_str(content.as_str()))
    } else {
        println!("config file read failed!");
        None
    }
}

fn parse_bk_file(path: &str) -> Option<Vec<BreakPoint>> {
    let mut file = File::open(path).expect("breakpoint file open failed!");
    let mut content = String::new();
    if let Ok(_) = file.read_to_string(&mut content) {
        Some(BreakPoint::vec_from_str(content.as_str()))
    } else {
        None
    }
}

#[no_mangle]
pub extern "C" fn Agent_OnLoad(vm: *mut JavaVM, opts: *mut c_char, reserved: *mut c_void) -> jint {
    let config = match parse_config_file(opts) {
        Some(c) => c,
        None => return JNI_ERR,
    };
    println!("parse config...");

    let mut null_ptr = ptr::null_mut() as *mut c_void;
    let jvmti_ptr = &mut null_ptr as *mut *mut c_void;
    if JNI_OK != unsafe {(**vm).GetEnv.unwrap()(vm, jvmti_ptr, JVMTI_VERSION_1_2)} {
        println!("ERROR: Unable to access JVMTI.");
        return JNI_ERR;
    }
    let jvmti = unsafe {(*jvmti_ptr) as *mut jvmtiEnv};
    println!("access JVMTI...");

    let c = capabilities::JVMTI_Capabilities::new();
    if (config.bytecode_dump) {

    }
    if (config.heap_print) {

    }
    if (config.break_point_json.is_some()) {

    }
    if JNI_OK != unsafe {(**jvmti).AddCapabilities.unwrap()(jvmti, &c.to_native())} {
        println!("ERROR: add capabilities failed!");
        return JNI_ERR;
    }

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