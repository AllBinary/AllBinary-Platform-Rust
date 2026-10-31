// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles.shader
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

pub struct OpenGLObject3dProcessor {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}


impl OpenGLObject3dProcessor {

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
    

    pub fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut object3d: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn process3(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut object3d: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut index: i32) {
    }
}
pub trait OpenGLObject3dProcessorDyn {
    fn as_OpenGLObject3dProcessor(&self) -> &OpenGLObject3dProcessor;
    fn as_OpenGLObject3dProcessor_mut(&mut self) -> &mut OpenGLObject3dProcessor;
    fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut object3d: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { OpenGLObject3dProcessor::process(self.as_OpenGLObject3dProcessor_mut(), gl, object3d) }
    fn process3(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut object3d: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut index: i32) { OpenGLObject3dProcessor::process3(self.as_OpenGLObject3dProcessor_mut(), gl, object3d, index) }
    fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut object3d: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { OpenGLObject3dProcessor::process(self.as_OpenGLObject3dProcessor_mut(), gl, object3d) }
    fn process3(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut object3d: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut index: i32) { OpenGLObject3dProcessor::process3(self.as_OpenGLObject3dProcessor_mut(), gl, object3d, index) }
}
impl OpenGLObject3dProcessorDyn for OpenGLObject3dProcessor {
    fn as_OpenGLObject3dProcessor(&self) -> &OpenGLObject3dProcessor { self }
    fn as_OpenGLObject3dProcessor_mut(&mut self) -> &mut OpenGLObject3dProcessor { self }
}


