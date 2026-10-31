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
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use super::OpenGLObject3dProcessor::OpenGLObject3dProcessor;
use super::ShaderComposite::ShaderComposite;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UniformTextureOpenGLProcessor {
    pub base: OpenGLObject3dProcessor,
}


impl UniformTextureOpenGLProcessor {
    

    pub fn new_struct(mut shaderComposite: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderCompositeDyn>>>, mut handleIndex: i32) -> Self {
        let mut this = Self {
            base: OpenGLObject3dProcessor::new_struct(),
        };
        this
    }

    pub fn new(mut shaderComposite: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderCompositeDyn>>>, mut handleIndex: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(shaderComposite, handleIndex))))
    }
    

    pub fn process3(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut unused: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut index: i32) {
    }
}
impl std::ops::Deref for UniformTextureOpenGLProcessor {
    type Target = OpenGLObject3dProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UniformTextureOpenGLProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessorDyn for UniformTextureOpenGLProcessor {
    fn as_OpenGLObject3dProcessor(&self) -> &crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessor { &self.base }
    fn as_OpenGLObject3dProcessor_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessor { &mut self.base }
    fn process3(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut unused: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut index: i32) { Self::process3(self, gl, unused, index) }
}


