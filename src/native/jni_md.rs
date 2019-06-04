#![allow(non_camel_case_types)]
use std::os::raw::{c_int, c_long, c_char};

pub type jint = c_int;
pub type jlong = c_long;
pub type jbyte = c_char;
