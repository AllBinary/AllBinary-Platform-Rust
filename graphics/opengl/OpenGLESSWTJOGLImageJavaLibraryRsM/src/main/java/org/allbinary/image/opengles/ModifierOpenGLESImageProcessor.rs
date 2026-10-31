// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.image.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use super::OpenGLESImageProcessor::OpenGLESImageProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ModifierOpenGLESImageProcessor {
    pub base: OpenGLESImageProcessor,
}

thread_local! { pub static ModifierOpenGLESImageProcessor_instanceC: Option<Rc<RefCell<ModifierOpenGLESImageProcessor>>> = ModifierOpenGLESImageProcessor::new(); }

impl ModifierOpenGLESImageProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: OpenGLESImageProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ModifierOpenGLESImageProcessor>>> {
        return ModifierOpenGLESImageProcessor_instanceC.with(Clone::clone).clone();
    }
    

    pub fn colorMask(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) {
        jcall!(gl).glColor4f(red.clone(), green.clone(), blue.clone(), alpha.clone());
    }
}
impl std::ops::Deref for ModifierOpenGLESImageProcessor {
    type Target = OpenGLESImageProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ModifierOpenGLESImageProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessorDyn for ModifierOpenGLESImageProcessor {
    fn as_OpenGLESImageProcessor(&self) -> &crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessor { &self.base }
    fn as_OpenGLESImageProcessor_mut(&mut self) -> &mut crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessor { &mut self.base }
    fn colorMask(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) { Self::colorMask(self, gl, red, green, blue, alpha) }
    fn colorMask(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) { Self::colorMask(self, gl, red, green, blue, alpha) }
}


