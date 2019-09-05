use super::native::jvmti::*;
use super::native::jni::*;
use super::native::jni_md::*;
use super::logger::*;
use super::config::*;
use super::writer::*;
use super::messages;
use super::runtime::RTInfo;
use std::os::raw::{c_char, c_uchar};
use std::ptr;
use std::ffi::{CStr, CString};
use core::borrow::Borrow;
use crate::messages::*;

unsafe fn get_method_id(jvmti: *mut jvmtiEnv, jni_env: *mut JNIEnv, jklass: jclass, name: &str, signature: &str) -> Option<jmethodID> {
    let func_sig = format!("{}.{}:{}", RTInfo::rt_instance().get_class_name(&jklass).unwrap(), name, signature);
    if let Some(result) = RTInfo::rt_instance().get_method_id(&func_sig) {
        return Some(result);
    }


    let mut count: jint = 0;
    let mut _jmethods: *mut jmethodID = ptr::null_mut();
    assert_log(
        (**jvmti).GetClassMethods.unwrap()(jvmti, jklass, &mut count as *mut jint, &mut _jmethods as *mut *mut jmethodID),
        Some("Get class methods failed..."),
        None,
    );

    let jmethods: &[jmethodID] = std::slice::from_raw_parts(_jmethods as *const jmethodID, count as usize);
    for i in 0..count as usize {
        let mut method_name: *mut c_char = ptr::null_mut();
        let mut method_signature: *mut c_char = ptr::null_mut();
        assert_log(
            (**jvmti).GetMethodName.unwrap()(jvmti, jmethods[i], &mut method_name as *mut *mut c_char, &mut method_signature as *mut *mut c_char, ptr::null_mut()),
            Some(messages::GET_METHOD_NAME_ERROR),
            None,
        );
        let c_name = CStr::from_ptr(method_name).to_str().unwrap();
        let c_signature = CStr::from_ptr(method_signature).to_str().unwrap();
        if c_name.eq(name) && c_signature.eq(signature) {
            RTInfo::rt_instance().insert_method_id(jmethods[i], func_sig.as_str());
            return Some(jmethods[i]);
        }
    }
    return None;

    // next code can't work normal
    /*let name_char = CString::new(name).expect(error_create_c_string(name).as_str());
    let signature_char = CString::new(signature).expect(error_create_c_string(signature).as_str());
    let method_id = (**jni_env).GetMethodID.unwrap()(jni_env, jklass, name_char.into_raw(), signature_char.into_raw());
    if method_id.is_null() {
        return None;
    }
    RTInfo::rt_instance().insert_method_id(method_id, func_sig.as_str());
    return Some(method_id);*/
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

unsafe fn set_break_point(jvmti: *mut jvmtiEnv, jni_env: *mut JNIEnv, klass: jclass, bk: &BreakPoint) {
    let method: jmethodID = match get_method_id(jvmti,
                                                jni_env,
                                                klass,
                                                bk.get_method_name().unwrap().as_str(),
                                                bk.get_method_signature().unwrap().as_str()) {
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

    for (name, methods) in GLOBAL_CONFIG.bytecode_methods.as_ref().unwrap() {
        if class_name.eq(format!("L{};", name.replace(".", "/")).as_str()) {
            println!("{}", class_name);
            RTInfo::rt_instance().insert_class_id(klass, name.as_str());
            for method_name in methods {
                let name_signature: Vec<&str> = method_name.split(":").collect();
                match get_method_id(jvmti_env, jni_env, klass, name_signature[0], name_signature[1]) {
                    None => break,
                    Some(id) => {
                        let mut _bytecodes: *mut c_uchar = ptr::null_mut();
                        let mut size: jint = 0;
                        assert_log(
                            (**jvmti_env).GetBytecodes.unwrap()(jvmti_env, id, &mut size as *mut jint, &mut _bytecodes as *mut *mut c_uchar),
                            Some(message_with_method(
                                BYTECODE_DUMP_ERROR,
                                RTInfo::rt_instance().get_method_name(&id).unwrap().as_str()
                            ).as_str()),
                            None
                        );
                        let bytecodes: &[u8] = std::slice::from_raw_parts(_bytecodes, size as usize);
                        let bytecodes_data: Vec<String> = bytecodes.iter().map(|b| format!("{:02X}", b)).collect();
                        writer(format!("[method: <{}>]:", RTInfo::rt_instance().get_method_name(&id).unwrap().as_str()).as_str());
                        write_str_vec(&bytecodes_data, 5);
                    }
                }
            }
        }
    }


    for i in 0..breakpoint_size() {
        let name = breakpoints(i).unwrap().get_class_name().unwrap();
        if class_name.eq(format!("L{};", name.replace(".", "/")).as_str()) {
            // inner class is not match
            RTInfo::rt_instance().insert_class_id(klass, name.as_str());
            set_break_point(jvmti_env, jni_env, klass, breakpoints(i).unwrap());
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
                "Z" => { // boolean
                    let mut boolean_value: jint = 0;
                    assert_log(
                        (**jvmti).GetLocalInt.unwrap()(jvmti, thread, 0, slot, &mut boolean_value as *mut jint),
                        Some(GET_LOCAL_BOOLEAN_ERROR),
                        None
                    );
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), boolean_value).as_str());
                },
                "B" => { // byte
                    let mut byte_value: jint = 0;
                    assert_log(
                        (**jvmti).GetLocalInt.unwrap()(jvmti, thread, 0, slot, &mut byte_value as *mut jint),
                        Some(GET_LOCAL_BYTE_ERROR),
                        None
                    );
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), byte_value).as_str());
                },
                "C" => { // char
                    let mut char_value: jint = 0;
                    assert_log(
                        (**jvmti).GetLocalInt.unwrap()(jvmti, thread, 0, slot, &mut char_value as *mut jint),
                        Some(GET_LOCAL_CHAR_ERROR),
                        None
                    );
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), char_value).as_str());
                },
                "S" => { // short
                    let mut short_value: jint = 0;
                    assert_log(
                        (**jvmti).GetLocalInt.unwrap()(jvmti, thread, 0, slot, &mut short_value as *mut jint),
                        Some(GET_LOCAL_SHORT_ERROR),
                        None
                    );
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), short_value).as_str());
                },
                "I" => {
                    let mut int_value: jint = 0;
                    assert_log(
                        (**jvmti).GetLocalInt.unwrap()(jvmti, thread, 0, slot, &mut int_value as *mut jint),
                        Some(GET_LOCAL_INT_ERROR),
                        None
                    );
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), int_value).as_str());
                },
                "J" => { // long
                    let mut long_value: jlong = 0;
                    assert_log(
                        (**jvmti).GetLocalLong.unwrap()(jvmti, thread, 0, slot, &mut long_value as *mut jlong),
                        Some(GET_LOCAL_LONG_ERROR),
                        None
                    );
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), long_value).as_str());
                },
                "F" => { // float
                    let mut float_value: jfloat = 0.0;
                    assert_log(
                        (**jvmti).GetLocalFloat.unwrap()(jvmti, thread, 0, slot, &mut float_value as *mut jfloat),
                        Some(GET_LOCAL_FLOAT_ERROR),
                        None
                    );
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), float_value).as_str());
                },
                "D" => { // double
                    let mut double_value: jdouble = 0.0;
                    assert_log(
                        (**jvmti).GetLocalDouble.unwrap()(jvmti, thread, 0, slot, &mut double_value as *mut jdouble),
                        Some(GET_LOCAL_DOUBLE_ERROR),
                        None
                    );
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), double_value).as_str());
                },
                _ => {
                    let mut object_value: jobject = ptr::null_mut();
                    assert_log(
                        (**jvmti).GetLocalObject.unwrap()(jvmti, thread, 0, slot, &mut object_value as *mut jobject),
                        Some(GET_LOCAL_OBJECT_ERROR),
                        None
                    );
                    let clazz: jclass = (**jni_env).GetObjectClass.unwrap()(jni_env, object_value);
                    let to_string_name = CString::new("toString")
                        .expect(error_create_c_string("toString").as_str());
                    let to_string_signature = CString::new("()Ljava/lang/String;")
                        .expect(error_create_c_string("()Ljava/lang/String;").as_str());
                    let methid_id: jmethodID = (**jni_env).GetMethodID.unwrap()(jni_env, clazz, to_string_name.into_raw(), to_string_signature.into_raw());
                    let string_object: jstring = (**jni_env).CallObjectMethod.unwrap()(jni_env, object_value, methid_id) as jstring;
                    let raw_string = (**jni_env).GetStringUTFChars.unwrap()(jni_env, string_object, ptr::null_mut() as *mut jboolean);
                    assert!(!raw_string.is_null());
                    let string_ref = CStr::from_ptr(raw_string).to_str().unwrap();
                    writer(format!("[Variable] {} : {}", var_name.unwrap().as_str(), string_ref).as_str());
                }
            };
        }
    }
}