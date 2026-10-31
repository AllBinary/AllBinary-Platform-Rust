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
use super::OpenGLESImageTranslate::OpenGLESImageTranslate;
use super::OpenGLESImage::OpenGLESImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESDeviceImageTranslate {
    pub base: OpenGLESImageTranslate,
    pub translateX: f32,
    pub translateY: f32,
    pub translateX2: f32,
    pub translateY2: f32,
}


impl OpenGLESDeviceImageTranslate {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: OpenGLESImageTranslate::new_struct(),
            translateX: 0.0_f32,
            translateY: 0.0_f32,
            translateX2: 0.0_f32,
            translateY2: 0.0_f32,
        };
        this.translateX = 0.0_f32;
        this.translateY = 0.0_f32;
        this.translateX2 = 0.0_f32;
        this.translateY2 = 0.0_f32;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn translate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>, mut x: f32, mut y: f32) {
        jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().imageProcessor).translate(gl.clone(), self.translateX + (jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties).scaleX * x), self.translateY + (jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties).scaleY * y));
    }
    

    pub fn translate2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>) {
        jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().imageProcessor).translate(gl.clone(), self.translateX2.clone(), self.translateY2.clone());
    }
}
impl std::ops::Deref for OpenGLESDeviceImageTranslate {
    type Target = OpenGLESImageTranslate;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLESDeviceImageTranslate {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::image::opengles::OpenGLESImageTranslate::OpenGLESImageTranslateDyn for OpenGLESDeviceImageTranslate {
    fn as_OpenGLESImageTranslate(&self) -> &crate::org::allbinary::image::opengles::OpenGLESImageTranslate::OpenGLESImageTranslate { &self.base }
    fn as_OpenGLESImageTranslate_mut(&mut self) -> &mut crate::org::allbinary::image::opengles::OpenGLESImageTranslate::OpenGLESImageTranslate { &mut self.base }
    fn translate(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>, mut x: f32, mut y: f32) { Self::translate(self, gl, openGLESImage, x, y) }
    fn translate2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>) { Self::translate2(self, gl, openGLESImage) }
}


