// Generated Code Do Not Modify
// package javax.microedition.khronos.egl
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EGLConfig {
}


impl EGLConfig {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

