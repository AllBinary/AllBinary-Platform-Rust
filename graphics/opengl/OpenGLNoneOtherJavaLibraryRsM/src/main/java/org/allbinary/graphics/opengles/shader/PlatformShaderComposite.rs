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
use crate::org::allbinary::graphics::opengles::NullOpenGLProcessorFactory::NullOpenGLProcessorFactory;
use crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessor;
use super::ShaderComposite::ShaderComposite;
use super::Shader::Shader;
use super::CompositeShaderUpdater::CompositeShaderUpdater;
use super::ShaderInitializer::ShaderInitializer;
use super::ModelViewProjection::ModelViewProjection;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformShaderComposite {
    pub base: ShaderComposite,
}


impl PlatformShaderComposite {
    

    pub fn new_struct(mut requiresOpenGLVersion: String, mut shaderArray: Vec<Option<Rc<RefCell<Shader>>>>, mut compositeShaderUpdater: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::CompositeShaderUpdater::CompositeShaderUpdaterDyn>>>, mut shaderInitializer: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderInitializer::ShaderInitializerDyn>>>, mut modelViewProjection: Option<Rc<RefCell<ModelViewProjection>>>, mut colorOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>, mut vertexOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>) -> Self {
        let mut this = Self {
            base: ShaderComposite::new_struct(requiresOpenGLVersion.clone(), shaderArray.clone(), compositeShaderUpdater.clone(), shaderInitializer.clone(), modelViewProjection.clone(), colorOpenGLProcessor.clone(), vertexOpenGLProcessor.clone(), NullOpenGLProcessorFactory::getInstance()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut requiresOpenGLVersion: String, mut shaderArray: Vec<Option<Rc<RefCell<Shader>>>>, mut compositeShaderUpdater: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::CompositeShaderUpdater::CompositeShaderUpdaterDyn>>>, mut shaderInitializer: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderInitializer::ShaderInitializerDyn>>>, mut modelViewProjection: Option<Rc<RefCell<ModelViewProjection>>>, mut colorOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>, mut vertexOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(requiresOpenGLVersion, shaderArray, compositeShaderUpdater, shaderInitializer, modelViewProjection, colorOpenGLProcessor, vertexOpenGLProcessor))))
    }
}
impl std::ops::Deref for PlatformShaderComposite {
    type Target = ShaderComposite;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PlatformShaderComposite {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderCompositeDyn for PlatformShaderComposite {
    fn as_ShaderComposite(&self) -> &crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderComposite { &self.base }
    fn as_ShaderComposite_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::shader::ShaderComposite::ShaderComposite { &mut self.base }
}


