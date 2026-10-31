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
use super::ShaderUpdater::ShaderUpdater;
use super::ShaderComposite::ShaderComposite;
use super::ShaderOpenGLProcessor::ShaderOpenGLProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AppRendererShaderUpdaterFactory {
    pub base: ShaderUpdater,
    pub NULL_SHADER_COMPOSITE_ARRAY: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderCompositeDyn>>>>,
    pub shaderCompositeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderCompositeDyn>>>>,
    pub shaderOpenGLProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderOpenGLProcessor::ShaderOpenGLProcessorDyn>>>>,
}

thread_local! { pub static AppRendererShaderUpdaterFactory_instanceC: Option<Rc<RefCell<AppRendererShaderUpdaterFactory>>> = AppRendererShaderUpdaterFactory::new(); }

impl AppRendererShaderUpdaterFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ShaderUpdater::new_struct(),
            NULL_SHADER_COMPOSITE_ARRAY: Vec::new(),
            shaderCompositeArray: Vec::new(),
            shaderOpenGLProcessorArray: Vec::new(),
        };
        this.NULL_SHADER_COMPOSITE_ARRAY = vec![None; (0) as usize];
        this.shaderCompositeArray = this.NULL_SHADER_COMPOSITE_ARRAY.clone();
        this.shaderOpenGLProcessorArray = vec![None; (0) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AppRendererShaderUpdaterFactory>>> {
        return AppRendererShaderUpdaterFactory_instanceC.with(Clone::clone).clone();
    }
    

    pub fn getTestShaders(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderOpenGLProcessor::ShaderOpenGLProcessorDyn>>>> {
        return self.shaderOpenGLProcessorArray.clone();
    }
    

    pub fn setShaderComposite(&mut self, mut object3d: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}
impl std::ops::Deref for AppRendererShaderUpdaterFactory {
    type Target = ShaderUpdater;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AppRendererShaderUpdaterFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdaterDyn for AppRendererShaderUpdaterFactory {
    fn as_ShaderUpdater(&self) -> &crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdater { &self.base }
    fn as_ShaderUpdater_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdater { &mut self.base }
}


