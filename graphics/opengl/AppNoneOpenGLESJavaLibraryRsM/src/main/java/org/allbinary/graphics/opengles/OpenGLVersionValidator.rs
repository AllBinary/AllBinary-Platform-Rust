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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderComposite;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLVersionValidator {
    pub EGL_CONTEXT_CLIENT_VERSION_VALUE: i32,
}

thread_local! { pub static OpenGLVersionValidator_instance: Option<Rc<RefCell<OpenGLVersionValidator>>> = OpenGLVersionValidator::new(); }

impl OpenGLVersionValidator {

    pub fn new_struct() -> Self {
        let mut this = Self {
            EGL_CONTEXT_CLIENT_VERSION_VALUE: 0,
        };
        this.EGL_CONTEXT_CLIENT_VERSION_VALUE = -1;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLVersionValidator>>> {
        return OpenGLVersionValidator_instance.with(Clone::clone).clone();
    }
    

    pub fn isGL31OrHigher(&mut self) -> bool {
        return false;
    }
    

    pub fn isAvailable(&mut self, mut glVersionRequired: String) -> bool {
        return false;
    }
    

    pub fn setShaderComposite(&mut self, mut shaderComposite: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderCompositeDyn>>>, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}

