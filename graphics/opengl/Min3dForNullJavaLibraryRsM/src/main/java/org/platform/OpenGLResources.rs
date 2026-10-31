// Generated Code Do Not Modify
// package org.platform
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLResources {
}

thread_local! { pub static OpenGLResources_instance: Option<Rc<RefCell<OpenGLResources>>> = OpenGLResources::new(); }

impl OpenGLResources {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLResources>>> {
        return OpenGLResources_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
    }
    

    pub fn isExcluded(&mut self, mut name: String) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getIdentifier(&mut self, mut name: String, mut defType: String, mut defPackage: String) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn openRawResource(&mut self, mut id: i32) -> Option<Rc<RefCell<InputStream>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn openRawResourceFromName(&mut self, mut resourceName: String) -> Option<Rc<RefCell<InputStream>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getResourceAsStream(&mut self, mut resourceName: String) -> Option<Rc<RefCell<InputStream>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getPackageName(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

