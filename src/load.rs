use super::native::jni_md::*;
use super::native::jni::*;
use super::native::jvmti::*;
use super::config::*;
use super::method::*;
use super::wrapper::capabilities::JVMTI_Capabilities;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::ptr;
use super::logger;
use super::messages;
use simple_logging::{log_to, log_to_file};
use log::LevelFilter::{Info, Debug};
use core::borrow::Borrow;
use std::mem::size_of;
use crate::logger::assert_log;
use std::path::{Path, PathBuf};
use std::fs;

fn parse_config_path(path: *mut c_char) -> PathBuf {
    assert!(!path.is_null());
    let path_name = unsafe {
        CStr::from_ptr(path).to_str().unwrap()
    };
    PathBuf::from(path_name)
}

fn parse_config_file(path: &str) -> Option<Box<Configuration>> {
    let mut file = File::open(path).expect("config file open failed!");
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

fn create_capabilities(config: &Configuration) -> JVMTI_Capabilities {
    let mut c: JVMTI_Capabilities = JVMTI_Capabilities::new();
    if !config.bytecode_dump.is_empty() {
        c.can_get_bytecodes = true;
    }
    if config.heap_print {

    }

    if config.class_print {

    }
    if !config.bytecode_dump.is_empty() {
        c.can_get_bytecodes = true;
    }
    if config.break_point_json.is_some() {
        c.can_access_local_variables = true;
        c.can_generate_breakpoint_events = true;
        c.can_get_line_numbers = true;
    }
    if config.watch_var.is_some() {
        c.can_access_local_variables = true;
        c.can_generate_field_access_events = true;
    }
    return c;
}

#[no_mangle]
pub unsafe extern "C" fn Agent_OnLoad(vm: *mut JavaVM, opts: *mut c_char, reserved: *mut c_void) -> jint {
    let path_name = parse_config_path(opts);
    GLOBAL_CONFIG.config = match parse_config_file(path_name
                                                    .canonicalize()
                                                    .expect("Can't find config file.").to_str().unwrap()) {
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
    if config.verbose {
        log_level = Debug;
    }
    if config.log_file.is_some() {
        log_to_file(Path::new(
                    config.log_file.as_ref().unwrap()).canonicalize().unwrap().to_str().unwrap(),
                    log_level
        );
    } else {
        log_to(std::io::stdout(), Info);
    }

    // init bytecode_dump vector to cache map
    GLOBAL_CONFIG.init_methods_map();
    // read breakpoints from json file
    if config.break_point_json.is_some() {
        let bk_path = Path::new(config.break_point_json.as_ref().unwrap());
        if !bk_path.is_absolute() {
            GLOBAL_CONFIG.breakpoints = parse_bk_file(
                path_name.parent().unwrap().join(bk_path).to_str().unwrap()
            );
        } else {
            GLOBAL_CONFIG.breakpoints = parse_bk_file(
                config.break_point_json.as_ref().unwrap().as_str()
            );
        }

    }
    let c: JVMTI_Capabilities = create_capabilities(config);
    logger::assert_log(
        (**jvmti).AddCapabilities.unwrap()(jvmti, &c.to_native()),
        Some("add capabilities failed!"),
        Some("add capabilities...")
    );

    if config.class_print {
        assert_log(
            (**jvmti).SetEventNotificationMode.unwrap()(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_PREPARE, ptr::null_mut()),
            Some(messages::error_with_set_event("class prepare").as_str()),
            None
        );
    }

    if !config.bytecode_dump.is_empty() {
        assert_log(
            (**jvmti).SetEventNotificationMode.unwrap()(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_PREPARE, ptr::null_mut()),
            Some(messages::error_with_set_event("class prepare").as_str()),
            None
        );
    }

    if config.break_point_json.is_some() {
        assert_log(
            (**jvmti).SetEventNotificationMode.unwrap()(jvmti, JVMTI_ENABLE, JVMTI_EVENT_BREAKPOINT, ptr::null_mut()),
            Some(messages::error_with_set_event("event breakpoint").as_str()),
            None
        );
        assert_log(
            (**jvmti).SetEventNotificationMode.unwrap()(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_PREPARE, ptr::null_mut()),
            Some(messages::error_with_set_event("class prepare").as_str()),
            None
        );
    }

    return on_load(vm, jvmti);
}

pub unsafe extern fn on_load(jvm: *mut JavaVM, jvmti: *mut jvmtiEnv) -> jint {
    let config :&Configuration = GLOBAL_CONFIG.config.as_ref().unwrap().borrow();
    let mut call_back = jvmtiEventCallbacks::new();
    if config.class_print {
        call_back.ClassPrepare = Some(event_class_prepare);
    }
    if !config.bytecode_dump.is_empty() {
        call_back.ClassPrepare = Some(event_class_prepare);
    }
    if config.break_point_json.is_some() {
        call_back.ClassPrepare = Some(event_class_prepare);
        call_back.Breakpoint = Some(event_break_point);
    }
    logger::assert_log((**jvmti).SetEventCallbacks.unwrap()(jvmti, &call_back, size_of::<jvmtiEventCallbacks>() as jint),
        Some("set event callbacks failed!"),
        Some("set event callbacks success!")
    );
    return JNI_OK;
}