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
// use crate::java::io::IOException::IOException;
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::graphics::OpenGLBitmap::OpenGLBitmap;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLImageUtilBase {
}


impl OpenGLImageUtilBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn paint(&mut self, mut alphaBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut originalBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut index: i32) -> Option<Rc<RefCell<OpenGLBitmap>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn paint4(&mut self, mut colorBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut originalBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut alpha: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<OpenGLBitmap>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws IOException
    pub fn createImage(&mut self, mut name: String) -> Option<Rc<RefCell<OpenGLBitmap>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn createImageFromInputStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<OpenGLBitmap>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait OpenGLImageUtilBaseDyn {
    fn as_OpenGLImageUtilBase(&self) -> &OpenGLImageUtilBase;
    fn as_OpenGLImageUtilBase_mut(&mut self) -> &mut OpenGLImageUtilBase;
    fn paint(&mut self, mut alphaBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut originalBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut index: i32) -> Option<Rc<RefCell<OpenGLBitmap>>> { OpenGLImageUtilBase::paint(self.as_OpenGLImageUtilBase_mut(), alphaBitmap, originalBitmap, index) }
    fn paint4(&mut self, mut colorBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut originalBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut alpha: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<OpenGLBitmap>>> { OpenGLImageUtilBase::paint4(self.as_OpenGLImageUtilBase_mut(), colorBitmap, originalBitmap, alpha, basicColor) }
    fn createImage(&mut self, mut name: String) -> Option<Rc<RefCell<OpenGLBitmap>>> { OpenGLImageUtilBase::createImage(self.as_OpenGLImageUtilBase_mut(), name) }
    fn createImageFromInputStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<OpenGLBitmap>>> { OpenGLImageUtilBase::createImageFromInputStream(self.as_OpenGLImageUtilBase_mut(), inputStream) }
    fn paint(&mut self, mut alphaBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut originalBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut index: i32) -> Option<Rc<RefCell<OpenGLBitmap>>> { OpenGLImageUtilBase::paint(self.as_OpenGLImageUtilBase_mut(), alphaBitmap, originalBitmap, index) }
    fn paint4(&mut self, mut colorBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut originalBitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut alpha: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<OpenGLBitmap>>> { OpenGLImageUtilBase::paint4(self.as_OpenGLImageUtilBase_mut(), colorBitmap, originalBitmap, alpha, basicColor) }
    fn createImage(&mut self, mut name: String) -> Option<Rc<RefCell<OpenGLBitmap>>> { OpenGLImageUtilBase::createImage(self.as_OpenGLImageUtilBase_mut(), name) }
    fn createImageFromInputStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<OpenGLBitmap>>> { OpenGLImageUtilBase::createImageFromInputStream(self.as_OpenGLImageUtilBase_mut(), inputStream) }
}
impl OpenGLImageUtilBaseDyn for OpenGLImageUtilBase {
    fn as_OpenGLImageUtilBase(&self) -> &OpenGLImageUtilBase { self }
    fn as_OpenGLImageUtilBase_mut(&mut self) -> &mut OpenGLImageUtilBase { self }
}


