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
use super::CompositeShaderUpdater::CompositeShaderUpdater;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimpleCompositeShaderUpdater {
    pub base: CompositeShaderUpdater,
}


impl SimpleCompositeShaderUpdater {
    

    pub fn new_struct(mut uniformBlockArray: Vec<String>, mut uniformArray: Vec<String>, mut attributeArray: Vec<String>, mut attributeHandleArray: Vec<i32>) -> Self {
        let mut this = Self {
            base: CompositeShaderUpdater::new_struct(uniformBlockArray.clone(), uniformArray.clone(), attributeArray.clone(), attributeHandleArray.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut uniformBlockArray: Vec<String>, mut uniformArray: Vec<String>, mut attributeArray: Vec<String>, mut attributeHandleArray: Vec<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(uniformBlockArray, uniformArray, attributeArray, attributeHandleArray))))
    }
}
impl std::ops::Deref for SimpleCompositeShaderUpdater {
    type Target = CompositeShaderUpdater;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SimpleCompositeShaderUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::opengles::shader::CompositeShaderUpdater::CompositeShaderUpdaterDyn for SimpleCompositeShaderUpdater {
    fn as_CompositeShaderUpdater(&self) -> &crate::org::allbinary::graphics::opengles::shader::CompositeShaderUpdater::CompositeShaderUpdater { &self.base }
    fn as_CompositeShaderUpdater_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::shader::CompositeShaderUpdater::CompositeShaderUpdater { &mut self.base }
}
impl crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdaterDyn for SimpleCompositeShaderUpdater {
    fn as_ShaderUpdater(&self) -> &crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdater { &self.base.base }
    fn as_ShaderUpdater_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdater { &mut self.base.base }
}


