// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles
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

pub struct OpenGLThreadUtil {
}

thread_local! { pub static OpenGLThreadUtil_instance: Option<Rc<RefCell<OpenGLThreadUtil>>> = OpenGLThreadUtil::new(); }

impl OpenGLThreadUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLThreadUtil>>> {
        return OpenGLThreadUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn set(&mut self, mut glSurfaceView: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onPause(&mut self) {
    }
    

    pub fn onResume(&mut self) {
    }
}

