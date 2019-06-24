use std::fmt;
use std::mem;
use std::sync::Mutex;
use core::borrow::Borrow;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use super::messages::*;
use super::logger;
use super::native::jni::{jmethodID, JNI_OK, JNI_ERR};

const NULL_STRING: &'static str = "<null>";

pub static mut GLOBAL_CONFIG: GConfig = GConfig {
    config: None,
    breakpoints: None,
    watch_var: None,
    breakpoint_info: None,
    bytecode_methods: None,
};

pub unsafe fn config() -> &'static Configuration {
    if GLOBAL_CONFIG.config.is_none() {
        panic!("config not init!!!");
    }
    GLOBAL_CONFIG.config.as_ref().unwrap().borrow()
}

pub unsafe fn breakpoint_size() -> usize {
    if GLOBAL_CONFIG.breakpoints.is_none() {
        0
    } else {
        GLOBAL_CONFIG.breakpoints.as_ref().unwrap().len()
    }
}

pub unsafe fn breakpoints(i: usize) -> Option<&'static BreakPoint> {
    if GLOBAL_CONFIG.breakpoints.is_none() {
        return None;
    }
    let vb : &Vec<BreakPoint>= GLOBAL_CONFIG.breakpoints.as_ref().unwrap().borrow();
    if vb.len() < i {
        return None;
    }
    vb.get(i)
}

pub trait JsonNew {
    fn new_from_str(data: &str) -> Box<Self>;
    fn vec_from_str(data: &str) -> Box<Vec<Self>> where Self: std::marker::Sized;
}

#[derive(Deserialize, Serialize)]
pub struct BreakPoint {
    class_name: Option<String>,
    method_name: Option<String>,
    method_signature: Option<String>,
    line: Option<u32>,
    var: Option<String>,
}

impl BreakPoint {
    pub fn get_class_name(&self) -> Option<&String> {
        self.class_name.as_ref()
    }

    pub fn get_method_name(&self) -> Option<&String> {
        self.method_name.as_ref()
    }

    pub fn get_method_signature(&self) -> Option<&String> {
        self.method_signature.as_ref()
    }

    pub fn get_method_full_name(&self) -> Option<String> {
        Some(format!("{}{}", match self.method_name.as_ref() {
            Some(s) => s.as_str(),
            None => NULL_STRING
        }, match self.method_signature.as_ref() {
            Some(s) => s.as_ref(),
            None => NULL_STRING
        }))
    }

    pub fn get_line_number(&self) -> Option<&u32> {
        self.line.as_ref()
    }

    pub fn get_variable(&self) -> Option<&String> {
        self.var.as_ref()
    }
}

impl JsonNew for BreakPoint {
    fn new_from_str(data: &str) -> Box<Self> {
        Box::new(serde_json::from_str(data).expect("Break point parse json error!"))
    }

    fn vec_from_str(data: &str) -> Box<Vec<Self>> {
        Box::new(serde_json::from_str(data).expect("Break point array parse json error!"))
    }
}

impl fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "class_name: {}\nmethod_name: {}\nmethod_signature: {}\nline: {}\n",
            self.class_name.as_ref().unwrap().as_str(), self.method_name.as_ref().unwrap().as_str(),
            self.method_signature.as_ref().unwrap().as_str(), self.line.as_ref().unwrap()
        )
    }
}

#[derive(Deserialize, Serialize)]
pub struct WatchVar {
    var_name: Option<String>,
    class_name: Option<String>,
    method_name: Option<String>,
    line_start: Option<u32>,
    line_end: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub verbose: bool,
    pub log_file: Option<String>,
    pub bytecode_dump: Vec<String>,
    pub heap_print: bool,
    pub class_print: bool,
    pub break_point_json: Option<String>,
    pub watch_var: Option<String>,
}

impl JsonNew for Configuration {
    fn new_from_str(data: &str) -> Box<Self> {
        Box::new(serde_json::from_str(data).expect("Config file parse json error!"))
    }

    fn vec_from_str(data: &str) -> Box<Vec<Self>> {
        Box::new(serde_json::from_str(data).expect("Config files parse json error!"))
    }
}

pub struct GConfig {
    pub config: Option<Box<Configuration>>,
    pub breakpoints: Option<Box<Vec<BreakPoint>>>,
    pub watch_var: Option<Box<Vec<WatchVar>>>,
    pub breakpoint_info: Option<HashMap<jmethodID, *const BreakPoint>>,
    pub bytecode_methods: Option<HashMap<String, Vec<String>>>
}

impl GConfig {
    pub fn put_breakpoint_info(&mut self, method_id: jmethodID, bk_point: *const BreakPoint) {
        if self.breakpoint_info.is_none() {
            self.breakpoint_info = Some(HashMap::new());
        }
        (self.breakpoint_info.as_mut().unwrap()).insert(method_id, bk_point);
    }

    pub fn get_breakpoint_info(&self, method_id: jmethodID) -> Option<&*const BreakPoint> {
        if self.breakpoint_info.as_ref().unwrap().contains_key(&method_id) {
            return self.breakpoint_info.as_ref().unwrap().get(&method_id);
        } else {
            return None;
        }
    }

    pub fn init_methods_map(&mut self) {
        if self.config.as_ref().unwrap().bytecode_dump.len() == 0 {
            return;
        } else {
            if self.bytecode_methods.is_none() {
                self.bytecode_methods = Some(HashMap::new());
            }
            let method_size = self.config.as_ref().unwrap().bytecode_dump.len();
            for i in 0..method_size {
                let full_name = self.config.as_ref().unwrap().bytecode_dump.get(i).unwrap();
                let names: Vec<&str> = full_name.split(":").collect();
                logger::assert_log(
                    match names.len() == 2 {
                    true => JNI_OK,
                    false => JNI_ERR
                },Some(message_with_path(METHOD_SIGNATURE_ERROR, "bytecode_dump vector").as_str()),None);
                let last_dot = names[0].rfind('.')
                    .expect(message_with_path(METHOD_SIGNATURE_ERROR, "bytecode_dump vector").as_str());
                logger::assert_log(
                    match last_dot < names[0].len() - 1 {
                        true => JNI_OK,
                        false => JNI_ERR
                    },
                    Some(message_with_path(METHOD_SIGNATURE_ERROR, "bytecode_dump vector").as_str()),
                    None
                );
                let class_name = String::from(&names[0][..last_dot]);
                let method_name = &names[0][last_dot+1..];
                if !self.bytecode_methods.as_ref().unwrap().contains_key(&class_name) {
                    self.bytecode_methods.as_mut().unwrap()
                        .insert(class_name.clone(), Vec::new());
                }
                self.bytecode_methods.as_mut().unwrap()
                    .get_mut(&class_name).unwrap()
                    .push(format!("{}:{}", method_name, names[1]));
            }
        }
    }
}