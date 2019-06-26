use std::collections::HashMap;
use std::ptr;

use super::native::jni::{jmethodID, jclass};
use std::cell::RefCell;

const RTINFO: *mut RTInfo = ptr::null_mut();

struct Klasses<'a> {
    id_map: RefCell<HashMap<&'a String, jclass>>,
    name_map: RefCell<HashMap<jclass, &'a String>>,
}

impl<'a> Klasses<'a> {
    fn new() -> Self {
        Klasses {
            id_map: RefCell::new(HashMap::new()),
            name_map: RefCell::new(HashMap::new()),
        }
    }

    fn get_class_id(&self, name: &String) -> Option<jclass> {
        match self.id_map.borrow().get(name) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    fn get_class_name(&self, id: &jclass) -> Option<&String> {
        match self.name_map.borrow().get(id) {
            Some(s) => Some(*s),
            None => None,
        }
    }
}

struct Methods<'a> {
    id_map: RefCell<HashMap<&'a String, jmethodID>>,
    name_map: RefCell<HashMap<jmethodID, &'a String>>,
}

impl<'a> Methods<'a> {
    fn new() -> Self {
        Methods {
            id_map: RefCell::new(HashMap::new()),
            name_map: RefCell::new(HashMap::new()),
        }
    }

    fn get_method_id(&self, name: &String) -> Option<jmethodID> {
        match self.id_map.borrow().get(name) {
            Some(i) => Some(*i),
            None => None,
        }
    }

    fn get_method_name(&self, id: &jmethodID) -> Option<&String> {
        match self.name_map.borrow().get(id) {
            Some(s) => Some(*s),
            None => None
        }
    }
}

// mark runtime info, example method id map
pub struct RTInfo<'a> {
    klasses: Klasses<'a>,
    methods: Methods<'a>,
}

impl<'a> RTInfo<'a> {
    pub unsafe fn rt_info() -> &'static Self {
        if RTINFO == ptr::null_mut() {
           *RTINFO = RTInfo {
               klasses: Klasses::new(),
               methods: Methods::new(),
           };
        }
        &(*RTINFO)
    }

    fn get_class_id(&self, name: &String) -> Option<jclass> {
        self.klasses.get_class_id(name)
    }

    fn get_class_name(&self, id: &jclass) -> Option<&String> {
        self.klasses.get_class_name(id)
    }

    pub fn get_method_id(&self, name: &String) -> Option<jmethodID> {
        self.methods.get_method_id(name)
    }

    pub fn get_method_name(&self, id: &jmethodID) -> Option<&String> {
        self.methods.get_method_name(id)
    }
}