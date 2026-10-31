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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESImageProcessor {
}

thread_local! { pub static OpenGLESImageProcessor_instance: Option<Rc<RefCell<dyn OpenGLESImageProcessorDyn>>> = OpenGLESImageProcessor::new().map(|reference| reference as Rc<RefCell<dyn OpenGLESImageProcessorDyn>>); }

impl OpenGLESImageProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn OpenGLESImageProcessorDyn>>> {
        return OpenGLESImageProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn translate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut translateX: f32, mut translateY: f32) {
        jcall!(gl).glTranslatef(translateX.clone(), translateY.clone(), 0);
    }
    

    pub fn rotate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut angle: f32) {
        jcall!(gl).glRotatef(-angle, 0, 0, 1);
    }
    

    pub fn scale(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut scaleX: f32, mut scaleY: f32) {
        jcall!(gl).glScalef(scaleX.clone(), scaleY.clone(), 1);
    }
    

    pub fn colorMask(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) {
    }
}
pub trait OpenGLESImageProcessorDyn {
    fn as_OpenGLESImageProcessor(&self) -> &OpenGLESImageProcessor;
    fn as_OpenGLESImageProcessor_mut(&mut self) -> &mut OpenGLESImageProcessor;
    fn translate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut translateX: f32, mut translateY: f32) { OpenGLESImageProcessor::translate(self.as_OpenGLESImageProcessor_mut(), gl, translateX, translateY) }
    fn rotate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut angle: f32) { OpenGLESImageProcessor::rotate(self.as_OpenGLESImageProcessor_mut(), gl, angle) }
    fn scale(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut scaleX: f32, mut scaleY: f32) { OpenGLESImageProcessor::scale(self.as_OpenGLESImageProcessor_mut(), gl, scaleX, scaleY) }
    fn colorMask(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) { OpenGLESImageProcessor::colorMask(self.as_OpenGLESImageProcessor_mut(), gl, red, green, blue, alpha) }
    fn translate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut translateX: f32, mut translateY: f32) { OpenGLESImageProcessor::translate(self.as_OpenGLESImageProcessor_mut(), gl, translateX, translateY) }
    fn rotate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut angle: f32) { OpenGLESImageProcessor::rotate(self.as_OpenGLESImageProcessor_mut(), gl, angle) }
    fn scale(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut scaleX: f32, mut scaleY: f32) { OpenGLESImageProcessor::scale(self.as_OpenGLESImageProcessor_mut(), gl, scaleX, scaleY) }
    fn colorMask(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) { OpenGLESImageProcessor::colorMask(self.as_OpenGLESImageProcessor_mut(), gl, red, green, blue, alpha) }
}
impl OpenGLESImageProcessorDyn for OpenGLESImageProcessor {
    fn as_OpenGLESImageProcessor(&self) -> &OpenGLESImageProcessor { self }
    fn as_OpenGLESImageProcessor_mut(&mut self) -> &mut OpenGLESImageProcessor { self }
}


