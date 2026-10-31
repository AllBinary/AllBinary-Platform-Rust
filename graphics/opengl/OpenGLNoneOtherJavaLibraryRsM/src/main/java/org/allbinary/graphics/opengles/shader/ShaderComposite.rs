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
use crate::org::allbinary::graphics::opengles::NullOpenGLProcessorFactory::NullOpenGLProcessorFactory;
use crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessor;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::Shader::Shader;
use super::ShaderInitializer::ShaderInitializer;
use super::CompositeShaderUpdater::CompositeShaderUpdater;
use super::ModelViewProjection::ModelViewProjection;
use super::OpenGLObject3dProcessor::OpenGLObject3dProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShaderComposite {
    pub disableProgramShaderOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub requiresOpenGLVersion: String,
    pub shaderArray: Vec<Option<Rc<RefCell<Shader>>>>,
    pub shaderInitializer: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderInitializer::ShaderInitializerDyn>>>,
    pub compositeShaderUpdater: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::CompositeShaderUpdater::CompositeShaderUpdaterDyn>>>,
    pub modelViewProjection: Option<Rc<RefCell<ModelViewProjection>>>,
    pub colorOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub vertexOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub useProgramShaderOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub shaderMatrixOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub colorEnableVertexAttribArrayOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub normalEnableVertexAttribArrayOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub vertexEnableVertexAttribArrayOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub textureEnableVertexAttribArrayOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub colorDisableVertexAttribArrayOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub normalDisableVertexAttribArrayOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub vertexDisableVertexAttribArrayOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub textureDisableVertexAttribArrayOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>,
    pub uniformLightPositionOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessorDyn>>>,
    pub uniformLightColorOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessorDyn>>>,
    pub uniformCameraPositionOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessorDyn>>>,
    pub uniformTextureUnitOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::OpenGLObject3dProcessor::OpenGLObject3dProcessorDyn>>>,
    pub programHandle: i32,
}


impl ShaderComposite {
    

    pub fn new_struct(mut requiresOpenGLVersion: String, mut shaderArray: Vec<Option<Rc<RefCell<Shader>>>>, mut compositeShaderUpdater: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::CompositeShaderUpdater::CompositeShaderUpdaterDyn>>>, mut shaderInitializer: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderInitializer::ShaderInitializerDyn>>>, mut modelViewProjection: Option<Rc<RefCell<ModelViewProjection>>>, mut colorOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>, mut vertexOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>, mut disableProgramShaderOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>) -> Self {
        let mut this = Self {
            disableProgramShaderOpenGLProcessor: None,
            requiresOpenGLVersion: String::new(),
            shaderArray: Vec::new(),
            shaderInitializer: None,
            compositeShaderUpdater: None,
            modelViewProjection: None,
            colorOpenGLProcessor: None,
            vertexOpenGLProcessor: None,
            useProgramShaderOpenGLProcessor: None,
            shaderMatrixOpenGLProcessor: None,
            colorEnableVertexAttribArrayOpenGLProcessor: None,
            normalEnableVertexAttribArrayOpenGLProcessor: None,
            vertexEnableVertexAttribArrayOpenGLProcessor: None,
            textureEnableVertexAttribArrayOpenGLProcessor: None,
            colorDisableVertexAttribArrayOpenGLProcessor: None,
            normalDisableVertexAttribArrayOpenGLProcessor: None,
            vertexDisableVertexAttribArrayOpenGLProcessor: None,
            textureDisableVertexAttribArrayOpenGLProcessor: None,
            uniformLightPositionOpenGLProcessor: None,
            uniformLightColorOpenGLProcessor: None,
            uniformCameraPositionOpenGLProcessor: None,
            uniformTextureUnitOpenGLProcessor: None,
            programHandle: 0,
        };
        this.useProgramShaderOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.shaderMatrixOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.colorEnableVertexAttribArrayOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.normalEnableVertexAttribArrayOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.vertexEnableVertexAttribArrayOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.textureEnableVertexAttribArrayOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.colorDisableVertexAttribArrayOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.normalDisableVertexAttribArrayOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.vertexDisableVertexAttribArrayOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.textureDisableVertexAttribArrayOpenGLProcessor = NullOpenGLProcessorFactory::getInstance();
        this.uniformLightPositionOpenGLProcessor = NullOpenGLTextureProcessor::NullOpenGLTextureProcessor::getInstance();
        this.uniformLightColorOpenGLProcessor = NullOpenGLTextureProcessor::NullOpenGLTextureProcessor::getInstance();
        this.uniformCameraPositionOpenGLProcessor = NullOpenGLTextureProcessor::NullOpenGLTextureProcessor::getInstance();
        this.uniformTextureUnitOpenGLProcessor = NullOpenGLTextureProcessor::NullOpenGLTextureProcessor::getInstance();
        this.requiresOpenGLVersion = requiresOpenGLVersion.clone();
        this.shaderArray = shaderArray.clone();
        this.shaderInitializer = shaderInitializer.clone();
        this.compositeShaderUpdater = compositeShaderUpdater.clone();
        this.modelViewProjection = modelViewProjection.clone();
        this.colorOpenGLProcessor = colorOpenGLProcessor.clone();
        this.vertexOpenGLProcessor = vertexOpenGLProcessor.clone();
        this.disableProgramShaderOpenGLProcessor = disableProgramShaderOpenGLProcessor.clone();
        this
    }

    pub fn new(mut requiresOpenGLVersion: String, mut shaderArray: Vec<Option<Rc<RefCell<Shader>>>>, mut compositeShaderUpdater: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::CompositeShaderUpdater::CompositeShaderUpdaterDyn>>>, mut shaderInitializer: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::shader::ShaderInitializer::ShaderInitializerDyn>>>, mut modelViewProjection: Option<Rc<RefCell<ModelViewProjection>>>, mut colorOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>, mut vertexOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>, mut disableProgramShaderOpenGLProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::OpenGLProcessor::OpenGLProcessorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(requiresOpenGLVersion, shaderArray, compositeShaderUpdater, shaderInitializer, modelViewProjection, colorOpenGLProcessor, vertexOpenGLProcessor, disableProgramShaderOpenGLProcessor))))
    }
    

    pub fn init(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
        self.programHandle = { let java_receiver = self.shaderInitializer.clone(); let java_arguments = (gl.clone(), self.shaderArray.clone(), jcall!(StringUtil::getInstance()).getArrayInstance(),); let java_result = jcall!(java_receiver).init(java_arguments.0, java_arguments.1, java_arguments.2); java_result };
    }
}
pub trait ShaderCompositeDyn {
    fn as_ShaderComposite(&self) -> &ShaderComposite;
    fn as_ShaderComposite_mut(&mut self) -> &mut ShaderComposite;
    fn init(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { ShaderComposite::init(self.as_ShaderComposite_mut(), gl) }
}
impl ShaderCompositeDyn for ShaderComposite {
    fn as_ShaderComposite(&self) -> &ShaderComposite { self }
    fn as_ShaderComposite_mut(&mut self) -> &mut ShaderComposite { self }
}


