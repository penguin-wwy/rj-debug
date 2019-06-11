use super::native::jni_md::*;
use super::native::jni::*;
use super::native::jvmti::*;
use super::config::*;
use super::wrapper::capabilities::JVMTI_Capabilities;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::ptr;
use super::logger;
use simple_logging::{log_to, log_to_file};
use log::LevelFilter::{Info, Debug};
use core::borrow::Borrow;
use std::mem::size_of;

fn parse_config_file(path: *mut c_char) -> Option<Box<Configuration>> {
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
        logger::error("config file read failed!");
        None
    }
}

fn parse_bk_file(path: &str) -> Option<Box<Vec<BreakPoint>>> {
    let mut file = File::open(path).expect("breakpoint file open failed!");
    let mut content = String::new();
    if let Ok(_) = file.read_to_string(&mut content) {
        Some(BreakPoint::vec_from_str(content.as_str()))
    } else {
        None
    }
}

#[no_mangle]
pub unsafe extern "C" fn Agent_OnLoad(vm: *mut JavaVM, opts: *mut c_char, reserved: *mut c_void) -> jint {
        GLOBAL_CONFIG.config = match parse_config_file(opts) {
            Some(c) => Some(c),
            None => return JNI_ERR,
        };
    let config :&Configuration = GLOBAL_CONFIG.config.as_ref().unwrap().borrow();

    logger::info("parse config...");

    let mut null_ptr = ptr::null_mut() as *mut c_void;
    let jvmti_ptr = &mut null_ptr as *mut *mut c_void;
    logger::assert_log(
        (**vm).GetEnv.unwrap()(vm, jvmti_ptr, JVMTI_VERSION_1_2),
        Some("Unable to access JVMTI."),
        None
    );
    let jvmti = (*jvmti_ptr) as *mut jvmtiEnv;
    logger::info("access JVMTI...");

    let mut log_level = Info;
    if (config.verbose) {
        log_level = Debug;
    }
    if (config.log_file.is_some()) {
        log_to_file(config.log_file.as_ref().unwrap(), log_level);
    } else {
        log_to(std::io::stdout(), Info);
    }

    let mut c: JVMTI_Capabilities = JVMTI_Capabilities::new();
    if (config.bytecode_dump) {
        c.can_get_bytecodes = true;
    }
    if (config.heap_print) {

    }

    if (config.break_point_json.is_some()) {
        c.can_access_local_variables = true;
        c.can_generate_breakpoint_events = true;
        c.can_get_line_numbers = true;
        GLOBAL_CONFIG.breakpoints = parse_bk_file(config.break_point_json.as_ref().unwrap().as_str());
    }
    if (config.watch_var.is_some()) {
        c.can_access_local_variables = true;
        c.can_generate_field_access_events = true;
    }
    logger::assert_log(
        (**jvmti).AddCapabilities.unwrap()(jvmti, &c.to_native()),
        Some("add capabilities failed!"),
        Some("add capabilities...")
    );

    return on_load(vm, jvmti);
}

pub unsafe extern fn on_load(jvm: *mut JavaVM, jvmti: *mut jvmtiEnv) -> jint {

    let mut call_back = jvmtiEventCallbacks::new();
    logger::assert_log((**jvmti).SetEventCallbacks.unwrap()(jvmti, &call_back, size_of::<jvmtiEventCallbacks>() as jint),
        Some("set event callbacks failed!"),
        Some("set event callbacks success!")
    );
    return JNI_OK;
}