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
// use crate::javax::microedition::khronos::egl::EGLConfig::EGLConfig;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::ShaderUpdater::ShaderUpdater;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompositeShaderUpdater {
    pub base: ShaderUpdater,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
}

thread_local! { pub static CompositeShaderUpdater_instanceC: Option<Rc<RefCell<dyn CompositeShaderUpdaterDyn>>> = CompositeShaderUpdater::new(jcall!(StringUtil::getInstance()).getArrayInstance(), jcall!(StringUtil::getInstance()).getArrayInstance(), jcall!(StringUtil::getInstance()).getArrayInstance(), jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone()).map(|reference| reference as Rc<RefCell<dyn CompositeShaderUpdaterDyn>>); }

impl CompositeShaderUpdater {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn CompositeShaderUpdaterDyn>>> {
        return CompositeShaderUpdater_instanceC.with(Clone::clone).clone();
    }
    

    pub fn new_struct(mut uniformBlockArray: Vec<String>, mut uniformArray: Vec<String>, mut attributeArray: Vec<String>, mut attributeHandleArray: Vec<i32>) -> Self {
        let mut this = Self {
            base: ShaderUpdater::new_struct(),
            commonSeps: None,
        };
        this.commonSeps = CommonSeps::getInstance();
        this
    }

    pub fn new(mut uniformBlockArray: Vec<String>, mut uniformArray: Vec<String>, mut attributeArray: Vec<String>, mut attributeHandleArray: Vec<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(uniformBlockArray, uniformArray, attributeArray, attributeHandleArray))))
    }
    

    pub fn onSurfaceCreated(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut eglConfig: Option<Rc<RefCell<EGLConfig>>>, mut programHandle: i32) {
    }
    

    pub fn onSurfaceDestroyed(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
    }
}
impl std::ops::Deref for CompositeShaderUpdater {
    type Target = ShaderUpdater;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompositeShaderUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CompositeShaderUpdaterDyn: crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdaterDyn {
    fn as_CompositeShaderUpdater(&self) -> &CompositeShaderUpdater;
    fn as_CompositeShaderUpdater_mut(&mut self) -> &mut CompositeShaderUpdater;
    fn onSurfaceCreated(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut eglConfig: Option<Rc<RefCell<EGLConfig>>>, mut programHandle: i32) { CompositeShaderUpdater::onSurfaceCreated(self.as_CompositeShaderUpdater_mut(), gl, eglConfig, programHandle) }
    fn onSurfaceDestroyed(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { CompositeShaderUpdater::onSurfaceDestroyed(self.as_CompositeShaderUpdater_mut(), gl) }
}
impl CompositeShaderUpdaterDyn for CompositeShaderUpdater {
    fn as_CompositeShaderUpdater(&self) -> &CompositeShaderUpdater { self }
    fn as_CompositeShaderUpdater_mut(&mut self) -> &mut CompositeShaderUpdater { self }
}

impl crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdaterDyn for CompositeShaderUpdater {
    fn as_ShaderUpdater(&self) -> &crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdater { &self.base }
    fn as_ShaderUpdater_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::shader::ShaderUpdater::ShaderUpdater { &mut self.base }
}


