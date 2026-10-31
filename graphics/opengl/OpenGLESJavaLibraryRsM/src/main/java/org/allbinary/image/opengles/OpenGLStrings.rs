// Generated Code Do Not Modify
// package org.allbinary.image.opengles
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

pub struct OpenGLStrings {
    pub GL_ERROR_LABEL: String,
    pub SET: String,
}

thread_local! { pub static OpenGLStrings_instance: Option<Rc<RefCell<OpenGLStrings>>> = OpenGLStrings::new(); }

impl OpenGLStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            GL_ERROR_LABEL: String::new(),
            SET: String::new(),
        };
        this.GL_ERROR_LABEL = String::from("GLError: ");
        this.SET = String::from("set(GL10)");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLStrings>>> {
        return OpenGLStrings_instance.with(Clone::clone).clone();
    }
}

