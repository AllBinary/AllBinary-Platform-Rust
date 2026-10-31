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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::Shader::Shader;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShaderInitializer {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static ShaderInitializer_instance: Option<Rc<RefCell<dyn ShaderInitializerDyn>>> = ShaderInitializer::new().map(|reference| reference as Rc<RefCell<dyn ShaderInitializerDyn>>); }

impl ShaderInitializer {

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
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ShaderInitializerDyn>>> {
        return ShaderInitializer_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut shader: Vec<Option<Rc<RefCell<Shader>>>>, mut attributeArray: Vec<String>) -> i32 {
        return -1;
    }
}
pub trait ShaderInitializerDyn {
    fn as_ShaderInitializer(&self) -> &ShaderInitializer;
    fn as_ShaderInitializer_mut(&mut self) -> &mut ShaderInitializer;
    fn init(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut shader: Vec<Option<Rc<RefCell<Shader>>>>, mut attributeArray: Vec<String>) -> i32 { ShaderInitializer::init(self.as_ShaderInitializer_mut(), gl, shader, attributeArray) }
}
impl ShaderInitializerDyn for ShaderInitializer {
    fn as_ShaderInitializer(&self) -> &ShaderInitializer { self }
    fn as_ShaderInitializer_mut(&mut self) -> &mut ShaderInitializer { self }
}


