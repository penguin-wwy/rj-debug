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
    let mut _jmethods: *mut jmethodID = ptr::null_mut();
    assert_log(
        (**jvmti).GetClassMethods.unwrap()(jvmti, jklass, &mut count as *mut jint, &mut _jmethods as *mut *mut jmethodID),
        Some("Get class methods failed..."),
        None
    );

    let jmethods: &[jmethodID] = std::slice::from_raw_parts(_jmethods as *const jmethodID, count as usize);
    for i in 0..count as usize {
        let mut method_name: *mut c_char = ptr::null_mut();
        assert_log(
            (**jvmti).GetMethodName.unwrap()(jvmti, jmethods[i], &mut method_name as *mut *mut c_char, ptr::null_mut(), ptr::null_mut()),
            Some("Get method name failed..."),
            None
        );
        if CStr::from_ptr(method_name).to_str().unwrap().eq(name) {
            return Some(jmethods[i]);
        }
    }
    return None;
}

unsafe fn set_break_point(jvmti: *mut jvmtiEnv, klass: jclass, bk: &BreakPoint) {
    info("set_break_point");
    let method: jmethodID = match get_method_id(jvmti, klass, bk.get_method_name().unwrap().as_str()) {
        Some(id) => id,
        None => return,
    };
    let mut count: jint = 0;
    let mut _entry: *mut jvmtiLineNumberEntry = ptr::null_mut();
    assert_log(
        (**jvmti).GetLineNumberTable.unwrap()(jvmti, method, &mut count as *mut jint, &mut _entry as *mut *mut jvmtiLineNumberEntry),
        Some("Get line number table failed..."),
        None
    );
    let entry: &[jvmtiLineNumberEntry] = std::slice::from_raw_parts(_entry, count as usize);
    for i in 0..count as usize {
        let entry_ref: &jvmtiLineNumberEntry = entry.get(i).unwrap();
        if entry_ref.line_number == *(bk.get_line_number().unwrap()) as i32 {
            assert_log(
                (**jvmti).SetBreakpoint.unwrap()(jvmti, method, entry_ref.start_location),
                Some("Set breakpoint failed."),
                None,
            );
        }
    }
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
            set_break_point(jvmti_env, klass, breakpoints(i).unwrap());
        }
    }
}

pub unsafe extern "C" fn event_break_point(jvmti: *mut jvmtiEnv, jni_env: *mut JNIEnv,
                                thread: jthread, method: jmethodID, location: jlocation) {
    info("event_break_point");
    let mut method_name: *mut c_char = ptr::null_mut();
    assert_log(
        (**jvmti).GetMethodName.unwrap()(jvmti, method, &mut method_name as *mut *mut c_char, ptr::null_mut(), ptr::null_mut()),
        Some("Get method name failed..."),
        None
    );
    let mut count: jint = 0;
    let mut _entry: *mut jvmtiLineNumberEntry = ptr::null_mut();
    assert_log(
        (**jvmti).GetLineNumberTable.unwrap()(jvmti, method, &mut count as *mut jint, &mut _entry as *mut *mut jvmtiLineNumberEntry),
        Some("Get line number table failed..."),
        None
    );
    let entry: &[jvmtiLineNumberEntry] = std::slice::from_raw_parts(_entry, count as usize);
    let mut hit: bool = false;
    for i in 0..count as usize {
        let entry_ref: &jvmtiLineNumberEntry = entry.get(i).unwrap();
        if entry_ref.start_location == location {
            writer(
                format!("[Breakpoint] {} : {}",
                        CStr::from_ptr(method_name).to_str().unwrap(),
                        entry_ref.line_number
                ).as_str()
            );
            hit = true;
            break;
        }
    }
    if hit == false {
        expect("Error breakpoint...", 1);
    }
}