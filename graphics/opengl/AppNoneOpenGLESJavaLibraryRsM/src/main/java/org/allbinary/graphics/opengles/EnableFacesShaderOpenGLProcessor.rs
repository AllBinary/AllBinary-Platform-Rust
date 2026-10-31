// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::org::allbinary::graphics::threed::min3d::renderer::processor::ObjectThreedOpenGLProcessor::ObjectThreedOpenGLProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EnableFacesShaderOpenGLProcessor {
    pub base: ObjectThreedOpenGLProcessor,
}

thread_local! { pub static EnableFacesShaderOpenGLProcessor_instance: Option<Rc<RefCell<EnableFacesShaderOpenGLProcessor>>> = EnableFacesShaderOpenGLProcessor::new(); }

impl EnableFacesShaderOpenGLProcessor {
    

    pub fn getInstance() -> Option<Rc<RefCell<EnableFacesShaderOpenGLProcessor>>> {
        return EnableFacesShaderOpenGLProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ObjectThreedOpenGLProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}
impl std::ops::Deref for EnableFacesShaderOpenGLProcessor {
    type Target = ObjectThreedOpenGLProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EnableFacesShaderOpenGLProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::threed::min3d::renderer::processor::ObjectThreedOpenGLProcessor::ObjectThreedOpenGLProcessorDyn for EnableFacesShaderOpenGLProcessor {
    fn as_ObjectThreedOpenGLProcessor(&self) -> &crate::org::allbinary::graphics::threed::min3d::renderer::processor::ObjectThreedOpenGLProcessor::ObjectThreedOpenGLProcessor { &self.base }
    fn as_ObjectThreedOpenGLProcessor_mut(&mut self) -> &mut crate::org::allbinary::graphics::threed::min3d::renderer::processor::ObjectThreedOpenGLProcessor::ObjectThreedOpenGLProcessor { &mut self.base }
    fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::process(self, gl, anyType) }
}


