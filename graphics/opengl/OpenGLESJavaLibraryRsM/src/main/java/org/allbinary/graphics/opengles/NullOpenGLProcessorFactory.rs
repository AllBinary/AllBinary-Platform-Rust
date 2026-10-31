// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::OpenGLProcessor::OpenGLProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullOpenGLProcessorFactory {
}

thread_local! { pub static NullOpenGLProcessorFactory_instance: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>> = OpenGLProcessor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>); }

impl NullOpenGLProcessorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>> {
        return NullOpenGLProcessorFactory_instance.with(Clone::clone).clone();
    }
}

