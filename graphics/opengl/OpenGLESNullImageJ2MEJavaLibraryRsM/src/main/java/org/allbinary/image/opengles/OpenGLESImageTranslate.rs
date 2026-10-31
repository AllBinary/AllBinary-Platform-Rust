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
// package org.allbinary.image.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use super::OpenGLESImage::OpenGLESImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESImageTranslate {
}

thread_local! { pub static OpenGLESImageTranslate_instance: Option<Rc<RefCell<dyn OpenGLESImageTranslateDyn>>> = OpenGLESImageTranslate::new().map(|reference| reference as Rc<RefCell<dyn OpenGLESImageTranslateDyn>>); }

impl OpenGLESImageTranslate {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn OpenGLESImageTranslateDyn>>> {
        return OpenGLESImageTranslate_instance.with(Clone::clone).clone();
    }
    

    pub fn translate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>, mut x: f32, mut y: f32) {
    }
    

    pub fn translate2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>) {
    }
}
pub trait OpenGLESImageTranslateDyn {
    fn as_OpenGLESImageTranslate(&self) -> &OpenGLESImageTranslate;
    fn as_OpenGLESImageTranslate_mut(&mut self) -> &mut OpenGLESImageTranslate;
    fn translate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>, mut x: f32, mut y: f32) { OpenGLESImageTranslate::translate(self.as_OpenGLESImageTranslate_mut(), gl, openGLESImage, x, y) }
    fn translate2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>) { OpenGLESImageTranslate::translate2(self.as_OpenGLESImageTranslate_mut(), gl, openGLESImage) }
}
impl OpenGLESImageTranslateDyn for OpenGLESImageTranslate {
    fn as_OpenGLESImageTranslate(&self) -> &OpenGLESImageTranslate { self }
    fn as_OpenGLESImageTranslate_mut(&mut self) -> &mut OpenGLESImageTranslate { self }
}


