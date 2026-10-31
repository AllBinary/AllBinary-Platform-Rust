// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLProcessor {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}


impl OpenGLProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
    }
}
pub trait OpenGLProcessorDyn {
    fn as_OpenGLProcessor(&self) -> &OpenGLProcessor;
    fn as_OpenGLProcessor_mut(&mut self) -> &mut OpenGLProcessor;
    fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { OpenGLProcessor::process(self.as_OpenGLProcessor_mut(), gl) }
}
impl OpenGLProcessorDyn for OpenGLProcessor {
    fn as_OpenGLProcessor(&self) -> &OpenGLProcessor { self }
    fn as_OpenGLProcessor_mut(&mut self) -> &mut OpenGLProcessor { self }
}


