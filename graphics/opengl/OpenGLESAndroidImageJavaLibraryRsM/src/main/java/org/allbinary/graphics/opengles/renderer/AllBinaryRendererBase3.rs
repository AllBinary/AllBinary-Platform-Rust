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
// package org.allbinary.graphics.opengles.renderer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryRendererBase3 {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl AllBinaryRendererBase3 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            list: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn update(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
        while jcall!(self.list).size() > 0 {
            let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = { let java_object = (jcall!(self.list).removeAt(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>>().expect("ClassCastException").clone()) };
            if openGLESImage.is_some() {
                jcall!(openGLESImage).set(gl.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>));
            }
        }
    }
    

    pub fn add(&mut self, mut image: Option<Rc<RefCell<Image>>>) {
        jcall!(self.list).add(crate::java_into_object(image.clone()));
    }
}
pub trait AllBinaryRendererBase3Dyn {
    fn as_AllBinaryRendererBase3(&self) -> &AllBinaryRendererBase3;
    fn as_AllBinaryRendererBase3_mut(&mut self) -> &mut AllBinaryRendererBase3;
    fn update(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { AllBinaryRendererBase3::update(self.as_AllBinaryRendererBase3_mut(), gl) }
    fn add(&mut self, mut image: Option<Rc<RefCell<Image>>>) { AllBinaryRendererBase3::add(self.as_AllBinaryRendererBase3_mut(), image) }
    fn update(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { AllBinaryRendererBase3::update(self.as_AllBinaryRendererBase3_mut(), gl) }
    fn add(&mut self, mut image: Option<Rc<RefCell<Image>>>) { AllBinaryRendererBase3::add(self.as_AllBinaryRendererBase3_mut(), image) }
}
impl AllBinaryRendererBase3Dyn for AllBinaryRendererBase3 {
    fn as_AllBinaryRendererBase3(&self) -> &AllBinaryRendererBase3 { self }
    fn as_AllBinaryRendererBase3_mut(&mut self) -> &mut AllBinaryRendererBase3 { self }
}


