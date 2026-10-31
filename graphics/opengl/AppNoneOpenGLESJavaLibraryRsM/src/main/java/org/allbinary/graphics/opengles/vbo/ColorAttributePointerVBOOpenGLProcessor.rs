// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
//  
// By agreeing to this license you and any business entity you represent are 
// legally bound to the AllBinary Open License Version 1 legal agreement. 
//  
// You may obtain the AllBinary Open License Version 1 legal agreement from 
// AllBinary or the root directory of AllBinary's AllBinary Platform repository. 
//  
// Created By: Travis Berthelot 
//   
// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles.vbo
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

pub struct ColorAttributePointerVBOOpenGLProcessor {
    pub base: ObjectThreedOpenGLProcessor,
}

thread_local! { pub static ColorAttributePointerVBOOpenGLProcessor_instance: Option<Rc<RefCell<ColorAttributePointerVBOOpenGLProcessor>>> = ColorAttributePointerVBOOpenGLProcessor::new(); }

impl ColorAttributePointerVBOOpenGLProcessor {
    

    pub fn getInstance() -> Option<Rc<RefCell<ColorAttributePointerVBOOpenGLProcessor>>> {
        return ColorAttributePointerVBOOpenGLProcessor_instance.with(Clone::clone).clone();
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
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
impl std::ops::Deref for ColorAttributePointerVBOOpenGLProcessor {
    type Target = ObjectThreedOpenGLProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ColorAttributePointerVBOOpenGLProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::threed::min3d::renderer::processor::ObjectThreedOpenGLProcessor::ObjectThreedOpenGLProcessorDyn for ColorAttributePointerVBOOpenGLProcessor {
    fn as_ObjectThreedOpenGLProcessor(&self) -> &crate::org::allbinary::graphics::threed::min3d::renderer::processor::ObjectThreedOpenGLProcessor::ObjectThreedOpenGLProcessor { &self.base }
    fn as_ObjectThreedOpenGLProcessor_mut(&mut self) -> &mut crate::org::allbinary::graphics::threed::min3d::renderer::processor::ObjectThreedOpenGLProcessor::ObjectThreedOpenGLProcessor { &mut self.base }
    fn process(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::process(self, gl, anyType) }
}


