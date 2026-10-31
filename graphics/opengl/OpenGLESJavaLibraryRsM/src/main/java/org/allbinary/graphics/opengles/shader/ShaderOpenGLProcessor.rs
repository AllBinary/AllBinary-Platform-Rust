// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles.shader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShaderOpenGLProcessor {
    pub base: OpenGLProcessor,
}


impl ShaderOpenGLProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: OpenGLProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process0(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
    }
    

    pub fn process1(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
    }
    

    pub fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
    }
    

    pub fn process2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
    }
    

    pub fn process3(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
    }
}
impl std::ops::Deref for ShaderOpenGLProcessor {
    type Target = OpenGLProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ShaderOpenGLProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ShaderOpenGLProcessorDyn: crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn {
    fn as_ShaderOpenGLProcessor(&self) -> &ShaderOpenGLProcessor;
    fn as_ShaderOpenGLProcessor_mut(&mut self) -> &mut ShaderOpenGLProcessor;
    fn process0(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderOpenGLProcessor::process0(self.as_ShaderOpenGLProcessor_mut(), gl) }
    fn process1(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderOpenGLProcessor::process1(self.as_ShaderOpenGLProcessor_mut(), gl) }
    fn process2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderOpenGLProcessor::process2(self.as_ShaderOpenGLProcessor_mut(), gl) }
    fn process3(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderOpenGLProcessor::process3(self.as_ShaderOpenGLProcessor_mut(), gl) }
    fn process0(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderOpenGLProcessor::process0(self.as_ShaderOpenGLProcessor_mut(), gl) }
    fn process1(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderOpenGLProcessor::process1(self.as_ShaderOpenGLProcessor_mut(), gl) }
    fn process2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderOpenGLProcessor::process2(self.as_ShaderOpenGLProcessor_mut(), gl) }
    fn process3(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderOpenGLProcessor::process3(self.as_ShaderOpenGLProcessor_mut(), gl) }
}
impl ShaderOpenGLProcessorDyn for ShaderOpenGLProcessor {
    fn as_ShaderOpenGLProcessor(&self) -> &ShaderOpenGLProcessor { self }
    fn as_ShaderOpenGLProcessor_mut(&mut self) -> &mut ShaderOpenGLProcessor { self }
}

impl crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn for ShaderOpenGLProcessor {
    fn as_OpenGLProcessor(&self) -> &crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessor { &self.base }
    fn as_OpenGLProcessor_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessor { &mut self.base }
    fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { Self::process(self, gl) }
}


