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
// package org.allbinary.graphics.opengles.shader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::OpenGLObject3dProcessor::OpenGLObject3dProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullOpenGLTextureProcessor {
}

thread_local! { pub static NullOpenGLTextureProcessor_instance: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessorDyn>>> = OpenGLObject3dProcessor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessorDyn>>); }

impl NullOpenGLTextureProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessorDyn>>> {
        return NullOpenGLTextureProcessor_instance.with(Clone::clone).clone();
    }
}

