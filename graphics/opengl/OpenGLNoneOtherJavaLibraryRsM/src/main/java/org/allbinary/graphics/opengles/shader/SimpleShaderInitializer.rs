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
use super::ShaderInitializer::ShaderInitializer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimpleShaderInitializer {
    pub base: ShaderInitializer,
}

thread_local! { pub static SimpleShaderInitializer_instanceC: Option<Rc<RefCell<SimpleShaderInitializer>>> = SimpleShaderInitializer::new(); }

impl SimpleShaderInitializer {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ShaderInitializer::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SimpleShaderInitializer>>> {
        return SimpleShaderInitializer_instanceC.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for SimpleShaderInitializer {
    type Target = ShaderInitializer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SimpleShaderInitializer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::opengles::shader::ShaderInitializer::ShaderInitializerDyn for SimpleShaderInitializer {
    fn as_ShaderInitializer(&self) -> &crate::org::allbinary::graphics::opengles::shader::ShaderInitializer::ShaderInitializer { &self.base }
    fn as_ShaderInitializer_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::shader::ShaderInitializer::ShaderInitializer { &mut self.base }
}


