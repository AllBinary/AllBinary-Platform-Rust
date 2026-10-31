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
use crate::org::allbinary::graphics::opengles::renderer::RendererStrings::RendererStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShaderUpdater {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub rendererStrings: Option<Rc<RefCell<RendererStrings>>>,
}

thread_local! { pub static ShaderUpdater_instance: Option<Rc<RefCell<dyn ShaderUpdaterDyn>>> = ShaderUpdater::new().map(|reference| reference as Rc<RefCell<dyn ShaderUpdaterDyn>>); }

impl ShaderUpdater {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
            rendererStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.rendererStrings = RendererStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ShaderUpdaterDyn>>> {
        return ShaderUpdater_instance.with(Clone::clone).clone();
    }
    

    pub fn onSurfaceCreated(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut eglConfig: Option<Rc<RefCell<EGLConfig>>>) {
    }
}
pub trait ShaderUpdaterDyn {
    fn as_ShaderUpdater(&self) -> &ShaderUpdater;
    fn as_ShaderUpdater_mut(&mut self) -> &mut ShaderUpdater;
    fn onSurfaceCreated(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut eglConfig: Option<Rc<RefCell<EGLConfig>>>) { ShaderUpdater::onSurfaceCreated(self.as_ShaderUpdater_mut(), gl, eglConfig) }
}
impl ShaderUpdaterDyn for ShaderUpdater {
    fn as_ShaderUpdater(&self) -> &ShaderUpdater { self }
    fn as_ShaderUpdater_mut(&mut self) -> &mut ShaderUpdater { self }
}


