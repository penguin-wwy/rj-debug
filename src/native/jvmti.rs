#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::os::raw::{c_void, c_char, c_uint, c_uchar};
use super::jni::*;
use super::jni_md::*;

pub const JVMTI_VERSION_1: jint = 0x30010000;
pub const JVMTI_VERSION_1_0: jint = 0x30010000;
pub const JVMTI_VERSION_1_1: jint = 0x30010100;
pub const JVMTI_VERSION_1_2: jint = 0x30010200;

pub const JVMTI_VERSION: jint = 0x30000000 + (1 * 0x10000) + (2 * 0x100) + 1;  /* version: 1.2.1 */

pub type jvmtiEnv = *const jvmtiInterface_1_;
pub type jthread = jobject;
pub type jthreadGroup = jobject;
pub type jlocation =jlong;
pub struct _jrawMonitorID {}
pub type jrawMonitorID = *mut _jrawMonitorID;
pub type jniNativeInterface = JNINativeInterface_;

pub const JVMTI_THREAD_STATE_ALIVE: jint = 0x0001;
pub const JVMTI_THREAD_STATE_TERMINATED: jint = 0x0002;
pub const JVMTI_THREAD_STATE_RUNNABLE: jint = 0x0004;
pub const JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER: jint = 0x0400;
pub const JVMTI_THREAD_STATE_WAITING: jint = 0x0080;
pub const JVMTI_THREAD_STATE_WAITING_INDEFINITELY: jint = 0x0010;
pub const JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT: jint = 0x0020;
pub const JVMTI_THREAD_STATE_SLEEPING: jint = 0x0040;
pub const JVMTI_THREAD_STATE_IN_OBJECT_WAIT: jint = 0x0100;
pub const JVMTI_THREAD_STATE_PARKED: jint = 0x0200;
pub const JVMTI_THREAD_STATE_SUSPENDED: jint = 0x100000;
pub const JVMTI_THREAD_STATE_INTERRUPTED: jint = 0x200000;
pub const JVMTI_THREAD_STATE_IN_NATIVE: jint = 0x400000;
pub const JVMTI_THREAD_STATE_VENDOR_1: jint = 0x10000000;
pub const JVMTI_THREAD_STATE_VENDOR_2: jint = 0x20000000;
pub const JVMTI_THREAD_STATE_VENDOR_3: jint = 0x40000000;

pub const JVMTI_JAVA_LANG_THREAD_STATE_MASK: jint = JVMTI_THREAD_STATE_TERMINATED | JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_RUNNABLE | JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_INDEFINITELY | JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;
pub const JVMTI_JAVA_LANG_THREAD_STATE_NEW: jint = 0;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED: jint = JVMTI_THREAD_STATE_TERMINATED;
pub const JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE: jint = JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_RUNNABLE;
pub const JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED: jint = JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER;
pub const JVMTI_JAVA_LANG_THREAD_STATE_WAITING: jint = JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_INDEFINITELY;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING: jint = JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;

pub const JVMTI_THREAD_MIN_PRIORITY: jint = 1;
pub const JVMTI_THREAD_NORM_PRIORITY: jint = 5;
pub const JVMTI_THREAD_MAX_PRIORITY: jint = 10;
pub const JVMTI_HEAP_FILTER_TAGGED: jint = 0x4;
pub const JVMTI_HEAP_FILTER_UNTAGGED: jint = 0x8;
pub const JVMTI_HEAP_FILTER_CLASS_TAGGED: jint = 0x10;
pub const JVMTI_HEAP_FILTER_CLASS_UNTAGGED: jint = 0x20;
pub const JVMTI_VISIT_OBJECTS: jint = 0x100;
pub const JVMTI_VISIT_ABORT: jint = 0x8000;

pub const JVMTI_HEAP_REFERENCE_CLASS: jint = 1;
pub const JVMTI_HEAP_REFERENCE_FIELD: jint = 2;
pub const JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT: jint = 3;
pub const JVMTI_HEAP_REFERENCE_CLASS_LOADER: jint = 4;
pub const JVMTI_HEAP_REFERENCE_SIGNERS: jint = 5;
pub const JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN: jint = 6;
pub const JVMTI_HEAP_REFERENCE_INTERFACE: jint = 7;
pub const JVMTI_HEAP_REFERENCE_STATIC_FIELD: jint = 8;
pub const JVMTI_HEAP_REFERENCE_CONSTANT_POOL: jint = 9;
pub const JVMTI_HEAP_REFERENCE_SUPERCLASS: jint = 10;
pub const JVMTI_HEAP_REFERENCE_JNI_GLOBAL: jint = 21;
pub const JVMTI_HEAP_REFERENCE_SYSTEM_CLASS: jint = 22;
pub const JVMTI_HEAP_REFERENCE_MONITOR: jint = 23;
pub const JVMTI_HEAP_REFERENCE_STACK_LOCAL: jint = 24;
pub const JVMTI_HEAP_REFERENCE_JNI_LOCAL: jint = 25;
pub const JVMTI_HEAP_REFERENCE_THREAD: jint = 26;
pub const JVMTI_HEAP_REFERENCE_OTHER: jint = 27;
pub type jvmtiHeapReferenceKind = jint;

pub const JVMTI_PRIMITIVE_TYPE_BOOLEAN: jint = 90;
pub const JVMTI_PRIMITIVE_TYPE_BYTE: jint = 66;
pub const JVMTI_PRIMITIVE_TYPE_CHAR: jint = 67;
pub const JVMTI_PRIMITIVE_TYPE_SHORT: jint = 83;
pub const JVMTI_PRIMITIVE_TYPE_INT: jint = 73;
pub const JVMTI_PRIMITIVE_TYPE_LONG: jint = 74;
pub const JVMTI_PRIMITIVE_TYPE_FLOAT: jint = 70;
pub const JVMTI_PRIMITIVE_TYPE_DOUBLE: jint = 68;
pub type jvmtiPrimitiveType = jint;

pub const JVMTI_HEAP_OBJECT_TAGGED: jint = 1;
pub const JVMTI_HEAP_OBJECT_UNTAGGED: jint = 2;
pub const JVMTI_HEAP_OBJECT_EITHER: jint = 3;
pub type jvmtiHeapObjectFilter = jint;

pub const JVMTI_HEAP_ROOT_JNI_GLOBAL: jint = 1;
pub const JVMTI_HEAP_ROOT_SYSTEM_CLASS: jint = 2;
pub const JVMTI_HEAP_ROOT_MONITOR: jint = 3;
pub const JVMTI_HEAP_ROOT_STACK_LOCAL: jint = 4;
pub const JVMTI_HEAP_ROOT_JNI_LOCAL: jint = 5;
pub const JVMTI_HEAP_ROOT_THREAD: jint = 6;
pub const JVMTI_HEAP_ROOT_OTHER: jint = 7;
pub type jvmtiHeapRootKind = jint;

pub const JVMTI_REFERENCE_CLASS: jint = 1;
pub const JVMTI_REFERENCE_FIELD: jint = 2;
pub const JVMTI_REFERENCE_ARRAY_ELEMENT: jint = 3;
pub const JVMTI_REFERENCE_CLASS_LOADER: jint = 4;
pub const JVMTI_REFERENCE_SIGNERS: jint = 5;
pub const JVMTI_REFERENCE_PROTECTION_DOMAIN: jint = 6;
pub const JVMTI_REFERENCE_INTERFACE: jint = 7;
pub const JVMTI_REFERENCE_STATIC_FIELD: jint = 8;
pub const JVMTI_REFERENCE_CONSTANT_POOL: jint = 9;
pub type jvmtiObjectReferenceKind = jint;

pub const JVMTI_ITERATION_CONTINUE: jint = 1;
pub const JVMTI_ITERATION_IGNORE: jint = 2;
pub const JVMTI_ITERATION_ABORT: jint = 0;
pub type jvmtiIterationControl = jint;

pub const JVMTI_CLASS_STATUS_VERIFIED: jint = 1;
pub const JVMTI_CLASS_STATUS_PREPARED: jint = 2;
pub const JVMTI_CLASS_STATUS_INITIALIZED: jint = 4;
pub const JVMTI_CLASS_STATUS_ERROR: jint = 8;
pub const JVMTI_CLASS_STATUS_ARRAY: jint = 16;
pub const JVMTI_CLASS_STATUS_PRIMITIVE: jint = 32;

pub const JVMTI_ENABLE: jint = 1;
pub const JVMTI_DISABLE: jint = 0;
pub type jvmtiEventMode = jint;

pub const JVMTI_TYPE_JBYTE: jint = 101;
pub const JVMTI_TYPE_JCHAR: jint = 102;
pub const JVMTI_TYPE_JSHORT: jint = 103;
pub const JVMTI_TYPE_JINT: jint = 104;
pub const JVMTI_TYPE_JLONG: jint = 105;
pub const JVMTI_TYPE_JFLOAT: jint = 106;
pub const JVMTI_TYPE_JDOUBLE: jint = 107;
pub const JVMTI_TYPE_JBOOLEAN: jint = 108;
pub const JVMTI_TYPE_JOBJECT: jint = 109;
pub const JVMTI_TYPE_JTHREAD: jint = 110;
pub const JVMTI_TYPE_JCLASS: jint = 111;
pub const JVMTI_TYPE_JVALUE: jint = 112;
pub const JVMTI_TYPE_JFIELDID: jint = 113;
pub const JVMTI_TYPE_JMETHODID: jint = 114;
pub const JVMTI_TYPE_CCHAR: jint = 115;
pub const JVMTI_TYPE_CVOID: jint = 116;
pub const JVMTI_TYPE_JNIENV: jint = 117;
pub type jvmtiParamTypes = jint;

pub const JVMTI_KIND_IN: jint = 91;
pub const JVMTI_KIND_IN_PTR: jint = 92;
pub const JVMTI_KIND_IN_BUF: jint = 93;
pub const JVMTI_KIND_ALLOC_BUF: jint = 94;
pub const JVMTI_KIND_ALLOC_ALLOC_BUF: jint = 95;
pub const JVMTI_KIND_OUT: jint = 96;
pub const JVMTI_KIND_OUT_BUF: jint = 97;
pub type jvmtiParamKind = jint;

pub const JVMTI_TIMER_USER_CPU: jint = 30;
pub const JVMTI_TIMER_TOTAL_CPU: jint = 31;
pub const JVMTI_TIMER_ELAPSED: jint = 32;
pub type jvmtiTimerKind = jint;

pub const JVMTI_PHASE_ONLOAD: jint = 1;
pub const JVMTI_PHASE_PRIMORDIAL: jint = 2;
pub const JVMTI_PHASE_START: jint = 6;
pub const JVMTI_PHASE_LIVE: jint = 4;
pub const JVMTI_PHASE_DEAD: jint = 8;
pub type jvmtiPhase = jint;

pub const JVMTI_VERSION_INTERFACE_JNI: jint = 0x00000000;
pub const JVMTI_VERSION_INTERFACE_JVMTI: jint = 0x30000000;

pub const JVMTI_VERSION_MASK_INTERFACE_TYPE: jint = 0x70000000;
pub const JVMTI_VERSION_MASK_MAJOR: jint = 0x0FFF0000;
pub const JVMTI_VERSION_MASK_MINOR: jint = 0x0000FF00;
pub const JVMTI_VERSION_MASK_MICRO: jint = 0x000000FF;

pub const JVMTI_VERSION_SHIFT_MAJOR: jint = 16;
pub const JVMTI_VERSION_SHIFT_MINOR: jint = 8;
pub const JVMTI_VERSION_SHIFT_MICRO: jint = 0;

pub const JVMTI_VERBOSE_OTHER: jint = 0;
pub const JVMTI_VERBOSE_GC: jint = 1;
pub const JVMTI_VERBOSE_CLASS: jint = 2;
pub const JVMTI_VERBOSE_JNI: jint = 4;
pub type jvmtiVerboseFlag = jint;

pub const JVMTI_JLOCATION_JVMBCI: jint = 1;
pub const JVMTI_JLOCATION_MACHINEPC: jint = 2;
pub const JVMTI_JLOCATION_OTHER: jint = 0;
pub type jvmtiJlocationFormat = jint;

pub const JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR: jint = 0x0001;
pub const JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP: jint = 0x0002;
pub const JVMTI_RESOURCE_EXHAUSTED_THREADS: jint = 0x0004;

pub const JVMTI_ERROR_NONE: jint = 0;
pub const JVMTI_ERROR_INVALID_THREAD: jint = 10;
pub const JVMTI_ERROR_INVALID_THREAD_GROUP: jint = 11;
pub const JVMTI_ERROR_INVALID_PRIORITY: jint = 12;
pub const JVMTI_ERROR_THREAD_NOT_SUSPENDED: jint = 13;
pub const JVMTI_ERROR_THREAD_SUSPENDED: jint = 14;
pub const JVMTI_ERROR_THREAD_NOT_ALIVE: jint = 15;
pub const JVMTI_ERROR_INVALID_OBJECT: jint = 20;
pub const JVMTI_ERROR_INVALID_CLASS: jint = 21;
pub const JVMTI_ERROR_CLASS_NOT_PREPARED: jint = 22;
pub const JVMTI_ERROR_INVALID_METHODID: jint = 23;
pub const JVMTI_ERROR_INVALID_LOCATION: jint = 24;
pub const JVMTI_ERROR_INVALID_FIELDID: jint = 25;
pub const JVMTI_ERROR_NO_MORE_FRAMES: jint = 31;
pub const JVMTI_ERROR_OPAQUE_FRAME: jint = 32;
pub const JVMTI_ERROR_TYPE_MISMATCH: jint = 34;
pub const JVMTI_ERROR_INVALID_SLOT: jint = 35;
pub const JVMTI_ERROR_DUPLICATE: jint = 40;
pub const JVMTI_ERROR_NOT_FOUND: jint = 41;
pub const JVMTI_ERROR_INVALID_MONITOR: jint = 50;
pub const JVMTI_ERROR_NOT_MONITOR_OWNER: jint = 51;
pub const JVMTI_ERROR_INTERRUPT: jint = 52;
pub const JVMTI_ERROR_INVALID_CLASS_FORMAT: jint = 60;
pub const JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION: jint = 61;
pub const JVMTI_ERROR_FAILS_VERIFICATION: jint = 62;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED: jint = 63;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED: jint = 64;
pub const JVMTI_ERROR_INVALID_TYPESTATE: jint = 65;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED: jint = 66;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED: jint = 67;
pub const JVMTI_ERROR_UNSUPPORTED_VERSION: jint = 68;
pub const JVMTI_ERROR_NAMES_DONT_MATCH: jint = 69;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED: jint = 70;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED: jint = 71;
pub const JVMTI_ERROR_UNMODIFIABLE_CLASS: jint = 79;
pub const JVMTI_ERROR_NOT_AVAILABLE: jint = 98;
pub const JVMTI_ERROR_MUST_POSSESS_CAPABILITY: jint = 99;
pub const JVMTI_ERROR_NULL_POINTER: jint = 100;
pub const JVMTI_ERROR_ABSENT_INFORMATION: jint = 101;
pub const JVMTI_ERROR_INVALID_EVENT_TYPE: jint = 102;
pub const JVMTI_ERROR_ILLEGAL_ARGUMENT: jint = 103;
pub const JVMTI_ERROR_NATIVE_METHOD: jint = 104;
pub const JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED: jint = 106;
pub const JVMTI_ERROR_OUT_OF_MEMORY: jint = 110;
pub const JVMTI_ERROR_ACCESS_DENIED: jint = 111;
pub const JVMTI_ERROR_WRONG_PHASE: jint = 112;
pub const JVMTI_ERROR_INTERNAL: jint = 113;
pub const JVMTI_ERROR_UNATTACHED_THREAD: jint = 115;
pub const JVMTI_ERROR_INVALID_ENVIRONMENT: jint = 116;
pub const JVMTI_ERROR_MAX: jint = 116;
pub type jvmtiError = jint;

pub const JVMTI_MIN_EVENT_TYPE_VAL: jint = 50;
pub const JVMTI_EVENT_VM_INIT: jint = 50;
pub const JVMTI_EVENT_VM_DEATH: jint = 51;
pub const JVMTI_EVENT_THREAD_START: jint = 52;
pub const JVMTI_EVENT_THREAD_END: jint = 53;
pub const JVMTI_EVENT_CLASS_FILE_LOAD_HOOK: jint = 54;
pub const JVMTI_EVENT_CLASS_LOAD: jint = 55;
pub const JVMTI_EVENT_CLASS_PREPARE: jint = 56;
pub const JVMTI_EVENT_VM_START: jint = 57;
pub const JVMTI_EVENT_EXCEPTION: jint = 58;
pub const JVMTI_EVENT_EXCEPTION_CATCH: jint = 59;
pub const JVMTI_EVENT_SINGLE_STEP: jint = 60;
pub const JVMTI_EVENT_FRAME_POP: jint = 61;
pub const JVMTI_EVENT_BREAKPOINT: jint = 62;
pub const JVMTI_EVENT_FIELD_ACCESS: jint = 63;
pub const JVMTI_EVENT_FIELD_MODIFICATION: jint = 64;
pub const JVMTI_EVENT_METHOD_ENTRY: jint = 65;
pub const JVMTI_EVENT_METHOD_EXIT: jint = 66;
pub const JVMTI_EVENT_NATIVE_METHOD_BIND: jint = 67;
pub const JVMTI_EVENT_COMPILED_METHOD_LOAD: jint = 68;
pub const JVMTI_EVENT_COMPILED_METHOD_UNLOAD: jint = 69;
pub const JVMTI_EVENT_DYNAMIC_CODE_GENERATED: jint = 70;
pub const JVMTI_EVENT_DATA_DUMP_REQUEST: jint = 71;
pub const JVMTI_EVENT_MONITOR_WAIT: jint = 73;
pub const JVMTI_EVENT_MONITOR_WAITED: jint = 74;
pub const JVMTI_EVENT_MONITOR_CONTENDED_ENTER: jint = 75;
pub const JVMTI_EVENT_MONITOR_CONTENDED_ENTERED: jint = 76;
pub const JVMTI_EVENT_RESOURCE_EXHAUSTED: jint = 80;
pub const JVMTI_EVENT_GARBAGE_COLLECTION_START: jint = 81;
pub const JVMTI_EVENT_GARBAGE_COLLECTION_FINISH: jint = 82;
pub const JVMTI_EVENT_OBJECT_FREE: jint = 83;
pub const JVMTI_EVENT_VM_OBJECT_ALLOC: jint = 84;
pub const JVMTI_MAX_EVENT_TYPE_VAL: jint = 84;
pub type jvmtiEvent = jint;

pub type jvmtiThreadInfo = _jvmtiThreadInfo;
pub type jvmtiMonitorStackDepthInfo = _jvmtiMonitorStackDepthInfo;
pub type jvmtiThreadGroupInfo = _jvmtiThreadGroupInfo;
pub type jvmtiFrameInfo = _jvmtiFrameInfo;
pub type jvmtiStackInfo =_jvmtiStackInfo;
pub type jvmtiHeapReferenceInfoField = _jvmtiHeapReferenceInfoField;
pub type jvmtiHeapReferenceInfoArray = _jvmtiHeapReferenceInfoArray;
pub type jvmtiHeapReferenceInfoConstantPool = _jvmtiHeapReferenceInfoConstantPool;
pub type jvmtiHeapReferenceInfoStackLocal = _jvmtiHeapReferenceInfoStackLocal;
pub type jvmtiHeapReferenceInfoJniLocal = _jvmtiHeapReferenceInfoJniLocal;
pub type jvmtiHeapReferenceInfoReserved = _jvmtiHeapReferenceInfoReserved;
pub type jvmtiHeapReferenceInfo = _jvmtiHeapReferenceInfo;
pub type jvmtiHeapCallbacks = _jvmtiHeapCallbacks;
pub type jvmtiClassDefinition = _jvmtiClassDefinition;
pub type jvmtiMonitorUsage = _jvmtiMonitorUsage;
pub type jvmtiLineNumberEntry = _jvmtiLineNumberEntry;
pub type jvmtiLocalVariableEntry = _jvmtiLocalVariableEntry;
pub type jvmtiParamInfo = _jvmtiParamInfo;
pub type jvmtiExtensionFunctionInfo = _jvmtiExtensionFunctionInfo;
pub type jvmtiExtensionEventInfo = _jvmtiExtensionEventInfo;
pub type jvmtiTimerInfo = _jvmtiTimerInfo;
pub type jvmtiAddrLocationMap = _jvmtiAddrLocationMap;

pub type jvmtiStartFunction = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, arg: *mut c_void) -> ()>;
pub type jvmtiHeapIterationCallback =
Option<unsafe extern "C" fn(class_tag: jlong, size: jlong, tag_ptr: *mut jlong, length: jint, user_data: *mut c_void) -> jint>;
pub type jvmtiHeapReferenceCallback =
Option<unsafe extern "C" fn(reference_kind: jvmtiHeapReferenceKind, reference_info: *const jvmtiHeapReferenceInfo, class_tag: jlong,
                            referrer_class_tag: jlong, size: jlong, tag_ptr: *mut jlong, referrer_tag_ptr: *mut jlong, length: jint,
                            user_data: *mut c_void) -> jint>;
pub type jvmtiPrimitiveFieldCallback =
Option<unsafe extern "C" fn(kind: jvmtiHeapReferenceKind, info: *const jvmtiHeapReferenceInfo, object_class_tag: jlong,
                            object_tag_ptr: *mut jlong, value: jvalue, value_type: jvmtiPrimitiveType,
                            user_data: *mut c_void) -> jint>;
pub type jvmtiArrayPrimitiveValueCallback =
Option<unsafe extern "C" fn(class_tag: jlong, size: jlong, tag_ptr: *mut jlong, element_count: jint, element_type: jvmtiPrimitiveType,
                            elements: *const c_void, user_data: *mut c_void) -> jint>;
pub type jvmtiStringPrimitiveValueCallback =
Option<unsafe extern "C" fn(class_tag: jlong, size: jlong, tag_ptr: *mut jlong, value: *const jchar, value_length: jint,
                            user_data: *mut c_void) -> jint>;
pub type jvmtiReservedCallback = Option<extern "C" fn() -> jint>;
pub type jvmtiHeapObjectCallback =
Option<unsafe extern "C" fn(class_tag: jlong, size: jlong, tag_ptr: *mut jlong, user_data: *mut c_void) -> jvmtiIterationControl>;
pub type jvmtiHeapRootCallback =
Option<unsafe extern "C" fn(root_kind: jvmtiHeapRootKind, class_tag: jlong, size: jlong, tag_ptr: *mut jlong,
                            user_data: *mut c_void) -> jvmtiIterationControl>;
pub type jvmtiStackReferenceCallback =
Option<unsafe extern "C" fn(root_kind: jvmtiHeapRootKind, class_tag: jlong, size: jlong, tag_ptr: *mut jlong, thread_tag: jlong,
                            depth: jint, method: jmethodID, slot: jint, user_data: *mut c_void) -> jvmtiIterationControl>;
pub type jvmtiObjectReferenceCallback =
Option<unsafe extern "C" fn(reference_kind: jvmtiObjectReferenceKind, class_tag: jlong, size: jlong, tag_ptr: *mut jlong,
                            referrer_tag: jlong, referrer_index: jint, user_data: *mut c_void) -> jvmtiIterationControl>;
pub type jvmtiExtensionFunction = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> jvmtiError>;
pub type jvmtiExtensionEvent = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiThreadInfo {
    pub name: *mut c_char,
    pub priority: jint,
    pub is_daemon: jboolean,
    pub thread_group: jthreadGroup,
    pub context_class_loader: jobject,
}
impl ::std::clone::Clone for _jvmtiThreadInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiThreadInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiMonitorStackDepthInfo {
    pub monitor: jobject,
    pub stack_depth: jint,
}
impl ::std::clone::Clone for _jvmtiMonitorStackDepthInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiMonitorStackDepthInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiThreadGroupInfo {
    pub parent: jthreadGroup,
    pub name: *mut c_char,
    pub max_priority: jint,
    pub is_daemon: jboolean,
}
impl ::std::clone::Clone for _jvmtiThreadGroupInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiThreadGroupInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiFrameInfo {
    pub method: jmethodID,
    pub location: jlocation,
}
impl ::std::clone::Clone for _jvmtiFrameInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiFrameInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiStackInfo {
    pub thread: jthread,
    pub state: jint,
    pub frame_buffer: *mut jvmtiFrameInfo,
    pub frame_count: jint,
}
impl ::std::clone::Clone for _jvmtiStackInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiStackInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiHeapReferenceInfoField {
    pub index: jint,
}
impl ::std::clone::Clone for _jvmtiHeapReferenceInfoField {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiHeapReferenceInfoField {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiHeapReferenceInfoArray {
    pub index: jint,
}
impl ::std::clone::Clone for _jvmtiHeapReferenceInfoArray {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiHeapReferenceInfoArray {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiHeapReferenceInfoConstantPool {
    pub index: jint,
}
impl ::std::clone::Clone for _jvmtiHeapReferenceInfoConstantPool {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiHeapReferenceInfoConstantPool {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiHeapReferenceInfoStackLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
    pub location: jlocation,
    pub slot: jint,
}
impl ::std::clone::Clone for _jvmtiHeapReferenceInfoStackLocal {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiHeapReferenceInfoStackLocal {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiHeapReferenceInfoJniLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
}
impl ::std::clone::Clone for _jvmtiHeapReferenceInfoJniLocal {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiHeapReferenceInfoJniLocal {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiHeapReferenceInfoReserved {
    pub reserved1: jlong,
    pub reserved2: jlong,
    pub reserved3: jlong,
    pub reserved4: jlong,
    pub reserved5: jlong,
    pub reserved6: jlong,
    pub reserved7: jlong,
    pub reserved8: jlong,
}
impl ::std::clone::Clone for _jvmtiHeapReferenceInfoReserved {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiHeapReferenceInfoReserved {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub union _jvmtiHeapReferenceInfo {
    field: jvmtiHeapReferenceInfoField,
    array: jvmtiHeapReferenceInfoArray,
    constant_pool: jvmtiHeapReferenceInfoConstantPool,
    stack_local: jvmtiHeapReferenceInfoStackLocal,
    jni_local: jvmtiHeapReferenceInfoJniLocal,
    other: jvmtiHeapReferenceInfoReserved,
}

impl ::std::clone::Clone for _jvmtiHeapReferenceInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiHeapReferenceInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiHeapCallbacks {
    pub heap_iteration_callback: jvmtiHeapIterationCallback,
    pub heap_reference_callback: jvmtiHeapReferenceCallback,
    pub primitive_field_callback: jvmtiPrimitiveFieldCallback,
    pub array_primitive_value_callback: jvmtiArrayPrimitiveValueCallback,
    pub string_primitive_value_callback: jvmtiStringPrimitiveValueCallback,
    pub reserved5: jvmtiReservedCallback,
    pub reserved6: jvmtiReservedCallback,
    pub reserved7: jvmtiReservedCallback,
    pub reserved8: jvmtiReservedCallback,
    pub reserved9: jvmtiReservedCallback,
    pub reserved10: jvmtiReservedCallback,
    pub reserved11: jvmtiReservedCallback,
    pub reserved12: jvmtiReservedCallback,
    pub reserved13: jvmtiReservedCallback,
    pub reserved14: jvmtiReservedCallback,
    pub reserved15: jvmtiReservedCallback,
}
impl ::std::clone::Clone for _jvmtiHeapCallbacks {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiHeapCallbacks {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiClassDefinition {
    pub klass: jclass,
    pub class_byte_count: jint,
    pub class_bytes: *const c_uchar,
}
impl ::std::clone::Clone for _jvmtiClassDefinition {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiClassDefinition {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiMonitorUsage {
    pub owner: jthread,
    pub entry_count: jint,
    pub waiter_count: jint,
    pub waiters: *mut jthread,
    pub notify_waiter_count: jint,
    pub notify_waiters: *mut jthread,
}
impl ::std::clone::Clone for _jvmtiMonitorUsage {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiMonitorUsage {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiLineNumberEntry {
    pub start_location: jlocation,
    pub line_number: jint,
}
impl ::std::clone::Clone for _jvmtiLineNumberEntry {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiLineNumberEntry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiLocalVariableEntry {
    pub start_location: jlocation,
    pub length: jint,
    pub name: *mut c_char,
    pub signature: *mut c_char,
    pub generic_signature: *mut c_char,
    pub slot: jint,
}
impl ::std::clone::Clone for _jvmtiLocalVariableEntry {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiLocalVariableEntry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiParamInfo {
    pub name: *mut c_char,
    pub kind: jvmtiParamKind,
    pub base_type: jvmtiParamTypes,
    pub null_ok: jboolean,
}
impl ::std::clone::Clone for _jvmtiParamInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiParamInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiExtensionFunctionInfo {
    pub func: jvmtiExtensionFunction,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
    pub error_count: jint,
    pub errors: *mut jvmtiError,
}
impl ::std::clone::Clone for _jvmtiExtensionFunctionInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiExtensionFunctionInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiExtensionEventInfo {
    pub extension_event_index: jint,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
}
impl ::std::clone::Clone for _jvmtiExtensionEventInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiExtensionEventInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiTimerInfo {
    pub max_value: jlong,
    pub may_skip_forward: jboolean,
    pub may_skip_backward: jboolean,
    pub kind: jvmtiTimerKind,
    pub reserved1: jlong,
    pub reserved2: jlong,
}
impl ::std::clone::Clone for _jvmtiTimerInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiTimerInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _jvmtiAddrLocationMap {
    pub start_address: *const c_void,
    pub location: jlocation,
}
impl ::std::clone::Clone for _jvmtiAddrLocationMap {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _jvmtiAddrLocationMap {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiCapabilities {
    pub _bindgen_bitfield_1_: c_uint,
    pub _bindgen_bitfield_2_: c_uint,
    pub _bindgen_bitfield_3_: c_uint,
    pub _bindgen_bitfield_4_: c_uint,
}
impl ::std::clone::Clone for jvmtiCapabilities {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for jvmtiCapabilities {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type jvmtiEventReserved = Option<extern "C" fn() -> ()>;
pub type jvmtiEventBreakpoint =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            location: jlocation) -> ()>;
pub type jvmtiEventClassFileLoadHook =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            class_being_redefined: jclass,
                            loader: jobject,
                            name: *const c_char,
                            protection_domain: jobject,
                            class_data_len: jint,
                            class_data:
                            *const c_uchar,
                            new_class_data_len: *mut jint,
                            new_class_data:
                            *mut *mut c_uchar)
                            -> ()>;
pub type jvmtiEventClassLoad =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread, klass: jclass)
                            -> ()>;
pub type jvmtiEventClassPrepare =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread, klass: jclass)
                            -> ()>;
pub type jvmtiEventCompiledMethodLoad =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            method: jmethodID,
                            code_size: jint,
                            code_addr:
                            *const c_void,
                            map_length: jint,
                            map:
                            *const jvmtiAddrLocationMap,
                            compile_info:
                            *const c_void)
                            -> ()>;
pub type jvmtiEventCompiledMethodUnload =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            method: jmethodID,
                            code_addr:
                            *const c_void)
                            -> ()>;
pub type jvmtiEventDataDumpRequest =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)
                            -> ()>;
pub type jvmtiEventDynamicCodeGenerated =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            name: *const c_char,
                            address: *const c_void,
                            length: jint) -> ()>;
pub type jvmtiEventException =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            location: jlocation,
                            exception: jobject,
                            catch_method: jmethodID,
                            catch_location: jlocation)
                            -> ()>;
pub type jvmtiEventExceptionCatch =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            location: jlocation,
                            exception: jobject) -> ()>;
pub type jvmtiEventFieldAccess =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            location: jlocation,
                            field_klass: jclass,
                            object: jobject,
                            field: jfieldID) -> ()>;
pub type jvmtiEventFieldModification =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            location: jlocation,
                            field_klass: jclass,
                            object: jobject,
                            field: jfieldID,
                            signature_type: c_char,
                            new_value: jvalue) -> ()>;
pub type jvmtiEventFramePop =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            was_popped_by_exception:
                            jboolean) -> ()>;
pub type jvmtiEventGarbageCollectionFinish =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)
                            -> ()>;
pub type jvmtiEventGarbageCollectionStart =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)
                            -> ()>;
pub type jvmtiEventMethodEntry =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID) -> ()>;
pub type jvmtiEventMethodExit =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            was_popped_by_exception:
                            jboolean,
                            return_value: jvalue) -> ()>;
pub type jvmtiEventMonitorContendedEnter =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            object: jobject) -> ()>;
pub type jvmtiEventMonitorContendedEntered =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            object: jobject) -> ()>;
pub type jvmtiEventMonitorWait =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            object: jobject,
                            timeout: jlong) -> ()>;
pub type jvmtiEventMonitorWaited =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            object: jobject,
                            timed_out: jboolean) -> ()>;
pub type jvmtiEventNativeMethodBind =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            address: *mut c_void,
                            new_address_ptr:
                            *mut *mut c_void)
                            -> ()>;
pub type jvmtiEventObjectFree =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            tag: jlong) -> ()>;
pub type jvmtiEventResourceExhausted =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            flags: jint,
                            reserved:
                            *const c_void,
                            description:
                            *const c_char)
                            -> ()>;
pub type jvmtiEventSingleStep =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            method: jmethodID,
                            location: jlocation) -> ()>;
pub type jvmtiEventThreadEnd =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread) -> ()>;
pub type jvmtiEventThreadStart =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread) -> ()>;
pub type jvmtiEventVMDeath =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv) -> ()>;
pub type jvmtiEventVMInit =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread) -> ()>;
pub type jvmtiEventVMObjectAlloc =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv,
                            thread: jthread,
                            object: jobject,
                            object_klass: jclass,
                            size: jlong) -> ()>;
pub type jvmtiEventVMStart =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv,
                            jni_env: *mut JNIEnv) -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiEventCallbacks {
    pub VMInit: jvmtiEventVMInit,
    pub VMDeath: jvmtiEventVMDeath,
    pub ThreadStart: jvmtiEventThreadStart,
    pub ThreadEnd: jvmtiEventThreadEnd,
    pub ClassFileLoadHook: jvmtiEventClassFileLoadHook,
    pub ClassLoad: jvmtiEventClassLoad,
    pub ClassPrepare: jvmtiEventClassPrepare,
    pub VMStart: jvmtiEventVMStart,
    pub Exception: jvmtiEventException,
    pub ExceptionCatch: jvmtiEventExceptionCatch,
    pub SingleStep: jvmtiEventSingleStep,
    pub FramePop: jvmtiEventFramePop,
    pub Breakpoint: jvmtiEventBreakpoint,
    pub FieldAccess: jvmtiEventFieldAccess,
    pub FieldModification: jvmtiEventFieldModification,
    pub MethodEntry: jvmtiEventMethodEntry,
    pub MethodExit: jvmtiEventMethodExit,
    pub NativeMethodBind: jvmtiEventNativeMethodBind,
    pub CompiledMethodLoad: jvmtiEventCompiledMethodLoad,
    pub CompiledMethodUnload: jvmtiEventCompiledMethodUnload,
    pub DynamicCodeGenerated: jvmtiEventDynamicCodeGenerated,
    pub DataDumpRequest: jvmtiEventDataDumpRequest,
    pub reserved72: jvmtiEventReserved,
    pub MonitorWait: jvmtiEventMonitorWait,
    pub MonitorWaited: jvmtiEventMonitorWaited,
    pub MonitorContendedEnter: jvmtiEventMonitorContendedEnter,
    pub MonitorContendedEntered: jvmtiEventMonitorContendedEntered,
    pub reserved77: jvmtiEventReserved,
    pub reserved78: jvmtiEventReserved,
    pub reserved79: jvmtiEventReserved,
    pub ResourceExhausted: jvmtiEventResourceExhausted,
    pub GarbageCollectionStart: jvmtiEventGarbageCollectionStart,
    pub GarbageCollectionFinish: jvmtiEventGarbageCollectionFinish,
    pub ObjectFree: jvmtiEventObjectFree,
    pub VMObjectAlloc: jvmtiEventVMObjectAlloc,
}
impl ::std::clone::Clone for jvmtiEventCallbacks {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for jvmtiEventCallbacks {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct jvmtiInterface_1_ {
    pub reserved1: *mut c_void,
    pub SetEventNotificationMode: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, mode: jvmtiEventMode, event_type: jvmtiEvent, event_thread: jthread, ...) -> jvmtiError>,
    pub reserved3: *mut c_void,
    pub GetAllThreads: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, threads_count_ptr: *mut jint, threads_ptr: *mut *mut jthread) -> jvmtiError>,
    pub SuspendThread: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub ResumeThread: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub StopThread: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, exception: jobject) -> jvmtiError>,
    pub InterruptThread: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub GetThreadInfo: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, info_ptr: *mut jvmtiThreadInfo) -> jvmtiError>,
    pub GetOwnedMonitorInfo: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, owned_monitor_count_ptr: *mut jint, owned_monitors_ptr: *mut *mut jobject) -> jvmtiError>,
    pub GetCurrentContendedMonitor: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, monitor_ptr: *mut jobject) -> jvmtiError>,
    pub RunAgentThread: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, _proc: jvmtiStartFunction, arg: *const c_void, priority: jint) -> jvmtiError>,
    pub GetTopThreadGroups: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, group_count_ptr: *mut jint, groups_ptr: *mut *mut jthreadGroup) -> jvmtiError>,
    pub GetThreadGroupInfo: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, group: jthreadGroup, info_ptr: *mut jvmtiThreadGroupInfo) -> jvmtiError>,
    pub GetThreadGroupChildren: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, group: jthreadGroup, thread_count_ptr: *mut jint, threads_ptr: *mut *mut jthread, group_count_ptr: *mut jint, groups_ptr: *mut *mut jthreadGroup) -> jvmtiError>,
    pub GetFrameCount: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, count_ptr: *mut jint) -> jvmtiError>,
    pub GetThreadState: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, thread_state_ptr: *mut jint) -> jvmtiError>,
    pub GetCurrentThread: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError>,
    pub GetFrameLocation: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, method_ptr: *mut jmethodID, location_ptr: *mut jlocation) -> jvmtiError>,
    pub NotifyFramePop: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError>,
    pub GetLocalObject: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jobject) -> jvmtiError>,
    pub GetLocalInt: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jint) -> jvmtiError>,
    pub GetLocalLong: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jlong) -> jvmtiError>,
    pub GetLocalFloat: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jfloat) -> jvmtiError>,
    pub GetLocalDouble: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value_ptr: *mut jdouble) -> jvmtiError>,
    pub SetLocalObject: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jobject) -> jvmtiError>,
    pub SetLocalInt: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jint) -> jvmtiError>,
    pub SetLocalLong: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jlong) -> jvmtiError>,
    pub SetLocalFloat: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jfloat) -> jvmtiError>,
    pub SetLocalDouble: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, slot: jint, value: jdouble) -> jvmtiError>,
    pub CreateRawMonitor: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, name: *const c_char, monitor_ptr: *mut jrawMonitorID) -> jvmtiError>,
    pub DestroyRawMonitor: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub RawMonitorEnter: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub RawMonitorExit: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub RawMonitorWait: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID, millis: jlong) -> jvmtiError>,
    pub RawMonitorNotify: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub RawMonitorNotifyAll: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError>,
    pub SetBreakpoint: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, location: jlocation) -> jvmtiError>,
    pub ClearBreakpoint: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, location: jlocation) -> jvmtiError>,
    pub reserved40: *mut c_void,
    pub SetFieldAccessWatch: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
    pub ClearFieldAccessWatch: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
    pub SetFieldModificationWatch: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
    pub ClearFieldModificationWatch: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError>,
    pub IsModifiableClass: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, is_modifiable_class_ptr: *mut jboolean) -> jvmtiError>,
    pub Allocate: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, size: jlong, mem_ptr: *mut *mut c_uchar) -> jvmtiError>,
    pub Deallocate: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, mem: *mut c_uchar) -> jvmtiError>,
    pub GetClassSignature: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetClassStatus: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, status_ptr: *mut jint) -> jvmtiError>,
    pub GetSourceFileName: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, source_name_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetClassModifiers: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, modifiers_ptr: *mut jint) -> jvmtiError>,
    pub GetClassMethods: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, method_count_ptr: *mut jint, methods_ptr: *mut *mut jmethodID) -> jvmtiError>,
    pub GetClassFields: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field_count_ptr: *mut jint, fields_ptr: *mut *mut jfieldID) -> jvmtiError>,
    pub GetImplementedInterfaces: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, interface_count_ptr: *mut jint, interfaces_ptr: *mut *mut jclass) -> jvmtiError>,
    pub IsInterface: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, is_interface_ptr: *mut jboolean) -> jvmtiError>,
    pub IsArrayClass: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, is_array_class_ptr: *mut jboolean) -> jvmtiError>,
    pub GetClassLoader: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, classloader_ptr: *mut jobject) -> jvmtiError>,
    pub GetObjectHashCode: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, hash_code_ptr: *mut jint) -> jvmtiError>,
    pub GetObjectMonitorUsage: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, info_ptr: *mut jvmtiMonitorUsage) -> jvmtiError>,
    pub GetFieldName: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, name_ptr: *mut *mut c_char, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetFieldDeclaringClass: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, declaring_class_ptr: *mut jclass) -> jvmtiError>,
    pub GetFieldModifiers: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, modifiers_ptr: *mut jint) -> jvmtiError>,
    pub IsFieldSynthetic: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID, is_synthetic_ptr: *mut jboolean) -> jvmtiError>,
    pub GetMethodName: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, name_ptr: *mut *mut c_char, signature_ptr: *mut *mut c_char, generic_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetMethodDeclaringClass: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, declaring_class_ptr: *mut jclass) -> jvmtiError>,
    pub GetMethodModifiers: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, modifiers_ptr: *mut jint) -> jvmtiError>,
    pub reserved67: *mut c_void,
    pub GetMaxLocals: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, max_ptr: *mut jint) -> jvmtiError>,
    pub GetArgumentsSize: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, size_ptr: *mut jint) -> jvmtiError>,
    pub GetLineNumberTable: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, entry_count_ptr: *mut jint, table_ptr: *mut *mut jvmtiLineNumberEntry) -> jvmtiError>,
    pub GetMethodLocation: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, start_location_ptr: *mut jlocation, end_location_ptr: *mut jlocation) -> jvmtiError>,
    pub GetLocalVariableTable: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, entry_count_ptr: *mut jint, table_ptr: *mut *mut jvmtiLocalVariableEntry) -> jvmtiError>,
    pub SetNativeMethodPrefix: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, prefix: *const c_char) -> jvmtiError>,
    pub SetNativeMethodPrefixes: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, prefix_count: jint, prefixes: *mut *mut c_char) -> jvmtiError>,
    pub GetBytecodes: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, bytecode_count_ptr: *mut jint, bytecodes_ptr: *mut *mut c_uchar) -> jvmtiError>,
    pub IsMethodNative: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, is_native_ptr: *mut jboolean) -> jvmtiError>,
    pub IsMethodSynthetic: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, is_synthetic_ptr: *mut jboolean) -> jvmtiError>,
    pub GetLoadedClasses: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, class_count_ptr: *mut jint, classes_ptr: *mut *mut jclass) -> jvmtiError>,
    pub GetClassLoaderClasses: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, initiating_loader: jobject, class_count_ptr: *mut jint, classes_ptr: *mut *mut jclass) -> jvmtiError>,
    pub PopFrame: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub ForceEarlyReturnObject: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jobject) -> jvmtiError>,
    pub ForceEarlyReturnInt: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError>,
    pub ForceEarlyReturnLong: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError>,
    pub ForceEarlyReturnFloat: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError>,
    pub ForceEarlyReturnDouble: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jdouble) -> jvmtiError>,
    pub ForceEarlyReturnVoid: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError>,
    pub RedefineClasses: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, class_count: jint, class_definitions: *const jvmtiClassDefinition) -> jvmtiError>,
    pub GetVersionNumber: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError>,
    pub GetCapabilities: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, capabilities_ptr: *mut jvmtiCapabilities) -> jvmtiError>,
    pub GetSourceDebugExtension: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, source_debug_extension_ptr: *mut *mut c_char) -> jvmtiError>,
    pub IsMethodObsolete: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, method: jmethodID, is_obsolete_ptr: *mut jboolean) -> jvmtiError>,
    pub SuspendThreadList: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, request_count: jint, request_list: *const jthread, results: *mut jvmtiError) -> jvmtiError>,
    pub ResumeThreadList: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, request_count: jint, request_list: *const jthread, results: *mut jvmtiError) -> jvmtiError>,
    pub reserved94: *mut c_void,
    pub reserved95: *mut c_void,
    pub reserved96: *mut c_void,
    pub reserved97: *mut c_void,
    pub reserved98: *mut c_void,
    pub reserved99: *mut c_void,
    pub GetAllStackTraces: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, max_frame_count: jint, stack_info_ptr: *mut *mut jvmtiStackInfo, thread_count_ptr: *mut jint) -> jvmtiError>,
    pub GetThreadListStackTraces: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread_count: jint, thread_list: *const jthread, max_frame_count: jint, stack_info_ptr: *mut *mut jvmtiStackInfo) -> jvmtiError>,
    pub GetThreadLocalStorage: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, data_ptr: *mut *mut c_void) -> jvmtiError>,
    pub SetThreadLocalStorage: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, data: *const c_void) -> jvmtiError>,
    pub GetStackTrace: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, start_depth: jint, max_frame_count: jint, frame_buffer: *mut jvmtiFrameInfo, count_ptr: *mut jint) -> jvmtiError>,
    pub reserved105: *mut c_void,
    pub GetTag: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, tag_ptr: *mut jlong) -> jvmtiError>,
    pub SetTag: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError>,
    pub ForceGarbageCollection: Option<unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError>,
    pub IterateOverObjectsReachableFromObject: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, object_reference_callback: jvmtiObjectReferenceCallback, user_data: *const c_void) -> jvmtiError>,
    pub IterateOverReachableObjects: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, heap_root_callback: jvmtiHeapRootCallback, stack_ref_callback: jvmtiStackReferenceCallback, object_ref_callback: jvmtiObjectReferenceCallback, user_data: *const c_void) -> jvmtiError>,
    pub IterateOverHeap: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, object_filter: jvmtiHeapObjectFilter, heap_object_callback: jvmtiHeapObjectCallback, user_data: *const c_void) -> jvmtiError>,
    pub IterateOverInstancesOfClass: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, object_filter: jvmtiHeapObjectFilter, heap_object_callback: jvmtiHeapObjectCallback, user_data: *const c_void) -> jvmtiError>,
    pub reserved113: *mut c_void,
    pub GetObjectsWithTags: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, tag_count: jint, tags: *const jlong, count_ptr: *mut jint, object_result_ptr: *mut *mut jobject, tag_result_ptr: *mut *mut jlong) -> jvmtiError>,
    pub FollowReferences: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, heap_filter: jint, klass: jclass, initial_object: jobject, callbacks: *const jvmtiHeapCallbacks, user_data: *const c_void) -> jvmtiError>,
    pub IterateThroughHeap: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, heap_filter: jint, klass: jclass, callbacks: *const jvmtiHeapCallbacks, user_data: *const c_void) -> jvmtiError>,
    pub reserved117: *mut c_void,
    pub reserved118: *mut c_void,
    pub reserved119: *mut c_void,
    pub SetJNIFunctionTable: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, function_table: *const jniNativeInterface) -> jvmtiError>,
    pub GetJNIFunctionTable: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, function_table: *mut *mut jniNativeInterface) -> jvmtiError>,
    pub SetEventCallbacks: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, callbacks: *const jvmtiEventCallbacks, size_of_callbacks: jint) -> jvmtiError>,
    pub GenerateEvents: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError>,
    pub GetExtensionFunctions: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, extension_count_ptr: *mut jint, extensions: *mut *mut jvmtiExtensionFunctionInfo) -> jvmtiError>,
    pub GetExtensionEvents: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, extension_count_ptr: *mut jint, extensions: *mut *mut jvmtiExtensionEventInfo) -> jvmtiError>,
    pub SetExtensionEventCallback: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, extension_event_index: jint, callback: jvmtiExtensionEvent) -> jvmtiError>,
    pub DisposeEnvironment: Option<unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError>,
    pub GetErrorName: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, error: jvmtiError, name_ptr: *mut *mut c_char) -> jvmtiError>,
    pub GetJLocationFormat: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, format_ptr: *mut jvmtiJlocationFormat) -> jvmtiError>,
    pub GetSystemProperties: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, count_ptr: *mut jint, property_ptr: *mut *mut *mut c_char) -> jvmtiError>,
    pub GetSystemProperty: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, property: *const c_char, value_ptr: *mut *mut c_char) -> jvmtiError>,
    pub SetSystemProperty: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, property: *const c_char, value: *const c_char) -> jvmtiError>,
    pub GetPhase: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError>,
    pub GetCurrentThreadCpuTimerInfo: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
    pub GetCurrentThreadCpuTime: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError>,
    pub GetThreadCpuTimerInfo: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
    pub GetThreadCpuTime: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, nanos_ptr: *mut jlong) -> jvmtiError>,
    pub GetTimerInfo: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError>,
    pub GetTime: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError>,
    pub GetPotentialCapabilities: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, capabilities_ptr: *mut jvmtiCapabilities) -> jvmtiError>,
    pub reserved141: *mut c_void,
    pub AddCapabilities: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, capabilities_ptr: *const jvmtiCapabilities) -> jvmtiError>,
    pub RelinquishCapabilities: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, capabilities_ptr: *const jvmtiCapabilities) -> jvmtiError>,
    pub GetAvailableProcessors: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError>,
    pub GetClassVersionNumbers: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, minor_version_ptr: *mut jint, major_version_ptr: *mut jint) -> jvmtiError>,
    pub GetConstantPool: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, constant_pool_count_ptr: *mut jint, constant_pool_byte_count_ptr: *mut jint, constant_pool_bytes_ptr: *mut *mut c_uchar) -> jvmtiError>,
    pub GetEnvironmentLocalStorage: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, data_ptr: *mut *mut c_void) -> jvmtiError>,
    pub SetEnvironmentLocalStorage: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, data: *const c_void) -> jvmtiError>,
    pub AddToBootstrapClassLoaderSearch: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError>,
    pub SetVerboseFlag: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, flag: jvmtiVerboseFlag, value: jboolean) -> jvmtiError>,
    pub AddToSystemClassLoaderSearch: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError>,
    pub RetransformClasses: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, class_count: jint, classes: *const jclass) -> jvmtiError>,
    pub GetOwnedMonitorStackDepthInfo: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, monitor_info_count_ptr: *mut jint, monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo) -> jvmtiError>,
    pub GetObjectSize: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, size_ptr: *mut jlong) -> jvmtiError>,
    pub GetLocalInstance: Option<unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint, value_ptr: *mut jobject) -> jvmtiError>,
}
impl ::std::clone::Clone for jvmtiInterface_1_ {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for jvmtiInterface_1_ {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
