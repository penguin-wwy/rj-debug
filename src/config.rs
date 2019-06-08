use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

const NULL_STRING: &'static str = "<null>";

pub trait JsonNew {
    fn new_from_str(data: &str) -> Self;
    fn vec_from_str(data: &str) -> Vec<Self> where Self: std::marker::Sized;
}

#[derive(Deserialize, Serialize)]
pub struct BreakPoint {
    class_name: Option<String>,
    method_name: Option<String>,
    method_signature: Option<String>,
    line: Option<u32>,
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
}

impl JsonNew for BreakPoint {
    fn new_from_str(data: &str) -> Self {
        serde_json::from_str(data).expect("Break point parse json error!")
    }

    fn vec_from_str(data: &str) -> Vec<Self> {
        serde_json::from_str(data).expect("Break point array parse json error!")
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

pub struct Configuration {
    pub bytecode_dump: bool,
    pub heap_print: bool,
    pub break_point_json: Option<String>,
}

impl JsonNew for Configuration {
    fn new_from_str(data: &str) -> Self {
        serde_json::from_str(data).expect("Config file parse json error!")
    }

    fn vec_from_str(data: &str) -> Vec<Self> {
        serde_json::from_str(data).expect("Config files parse json error!")
    }
}
