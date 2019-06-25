use super::native::jvmti::*;
use super::native::jni::*;
use super::native::jni_md::*;
use super::logger::*;
use super::config::*;
use super::writer::*;
use super::messages;
use std::os::raw::c_char;
use std::ptr;
use std::ffi::CStr;
use core::borrow::Borrow;
use crate::messages::{GET_LOCAL_VARIABLE_ERROR, GET_LINE_TABLE_ERROR, SET_BREAKPOINT_ERROR, GET_CLASS_SIGNATURE_ERROR, UNKNOWN_BREAKPOINT};

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
        let mut method_signature: *mut c_char = ptr::null_mut();
        assert_log(
            (**jvmti).GetMethodName.unwrap()(jvmti, jmethods[i], &mut method_name as *mut *mut c_char, &mut method_signature as *mut *mut c_char, ptr::null_mut()),
            Some(messages::GET_METHOD_NAME_ERROR),
            None
        );
        if CStr::from_ptr(method_name).to_str().unwrap().eq(name) {
            info(format!("method_signature : {}", CStr::from_ptr(method_signature).to_str().unwrap()).as_str());
            return Some(jmethods[i]);
        }
    }
    return None;
}

unsafe fn get_local_variable(jvmti: *mut jvmtiEnv, jmethod: jmethodID, name: &str, location: jlocation) -> Option<(jint, *mut c_char)> {
    let mut count: jint = 0;
    let mut _jLocalVarTableEntry: *mut jvmtiLocalVariableEntry = ptr::null_mut();
    assert_log(
        (**jvmti).GetLocalVariableTable.unwrap()(jvmti, jmethod, &mut count as *mut jint, &mut _jLocalVarTableEntry as *mut *mut jvmtiLocalVariableEntry),
        Some(GET_LOCAL_VARIABLE_ERROR),
        None
    );
    let jLocalVarTabelEntry: &[jvmtiLocalVariableEntry] = std::slice::from_raw_parts(
        _jLocalVarTableEntry as *const jvmtiLocalVariableEntry, count as usize);
    for i in 0..count as usize {
        let entry = jLocalVarTabelEntry[i].borrow();
        if location >= entry.start_location &&
            location < entry.start_location + entry.length as jlocation &&
            CStr::from_ptr(entry.name).to_str().unwrap() == name {
            return Some((entry.slot, entry.signature));
        }
    }
    return None;
}

unsafe fn set_break_point(jvmti: *mut jvmtiEnv, klass: jclass, bk: &BreakPoint) {
    let method: jmethodID = match get_method_id(jvmti, klass, bk.get_method_name().unwrap().as_str()) {
        Some(id) => id,
        None => return,
    };

    // cache method id and breakpoint
    GLOBAL_CONFIG.put_breakpoint_info(method, bk as *const BreakPoint);

    let mut count: jint = 0;
    let mut _entry: *mut jvmtiLineNumberEntry = ptr::null_mut();
    assert_log(
        (**jvmti).GetLineNumberTable.unwrap()(jvmti, method, &mut count as *mut jint, &mut _entry as *mut *mut jvmtiLineNumberEntry),
        Some(GET_LINE_TABLE_ERROR),
        None
    );
    let entry: &[jvmtiLineNumberEntry] = std::slice::from_raw_parts(_entry, count as usize);
    for i in 0..count as usize {
        let entry_ref: &jvmtiLineNumberEntry = entry.get(i).unwrap();
        if entry_ref.line_number == *(bk.get_line_number().unwrap()) as i32 {
            assert_log(
                (**jvmti).SetBreakpoint.unwrap()(jvmti, method, entry_ref.start_location),
                Some(SET_BREAKPOINT_ERROR),
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
        Some(GET_CLASS_SIGNATURE_ERROR),
        None
    );
    assert!(!class_name_native.is_null());
    let class_name= String::from((CStr::from_ptr(class_name_native).to_str().unwrap()));
    if config().class_print {
        writer(format!("[class prepare] {}", class_name.as_str()).as_str());
    }

    for i in 0..breakpoint_size() {
        if class_name.eq(format!("L{};", breakpoints(i).unwrap().get_class_name().unwrap()).as_str()) {
            set_break_point(jvmti_env, klass, breakpoints(i).unwrap());
        }
    }
}

pub unsafe extern "C" fn event_break_point(jvmti: *mut jvmtiEnv, jni_env: *mut JNIEnv,
                                thread: jthread, method: jmethodID, location: jlocation) {
    let mut method_name: *mut c_char = ptr::null_mut();
    assert_log(
        (**jvmti).GetMethodName.unwrap()(jvmti, method, &mut method_name as *mut *mut c_char, ptr::null_mut(), ptr::null_mut()),
        Some(messages::GET_METHOD_NAME_ERROR),
        None
    );
    let mut count: jint = 0;
    let mut _entry: *mut jvmtiLineNumberEntry = ptr::null_mut();
    assert_log(
        (**jvmti).GetLineNumberTable.unwrap()(jvmti, method, &mut count as *mut jint, &mut _entry as *mut *mut jvmtiLineNumberEntry),
        Some(GET_LINE_TABLE_ERROR),
        None
    );
    let entry: &[jvmtiLineNumberEntry] = std::slice::from_raw_parts(_entry, count as usize);

    let bk: Option<&*const BreakPoint> = GLOBAL_CONFIG.get_breakpoint_info(method);

    let mut hit: bool = false;
    for i in 0..count as usize {
        let entry_ref: &jvmtiLineNumberEntry = entry.get(i).unwrap();
        if entry_ref.start_location == location && bk.is_some() {
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
        expect(UNKNOWN_BREAKPOINT, 1);
    }

    let var_name = (&**(bk.unwrap())).get_variable();
    if var_name.is_some() {
        let var_info = get_local_variable(jvmti, method, var_name.unwrap().as_str(), location);
        if var_info.is_some() {
            let (slot, signature) = var_info.unwrap();
            writer(
                format!("[Breakpoint] {} : {}",
                        slot,
                        CStr::from_ptr(signature).to_str().unwrap()
                ).as_str()
            );
            match CStr::from_ptr(signature).to_str().unwrap() {
                "I" => {
                    let mut int_value: jint= 0;
                    (**jvmti).GetLocalInt.unwrap()(jvmti, thread, 0, slot, &mut int_value as *mut jint);
                    writer(format!("[Variable] {}", int_value).as_str());
                },
                _ => { // TODO
                    writer("NULL");
                }
            };
        }
    }
}