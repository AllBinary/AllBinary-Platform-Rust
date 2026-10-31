// Generated Code Do Not Modify
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::graphics::Bitmap::Bitmap;
use crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLBitmap {
    pub base: PlatformBitmapBase,
    pub bitmap: Option<Rc<RefCell<Bitmap>>>,
}


impl OpenGLBitmap {
    

    pub fn new_struct(mut bitmap: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            base: PlatformBitmapBase::new_struct(),
            bitmap: None,
        };
        this.bitmap = (bitmap).clone() /* as Bitmap */;
        this
    }

    pub fn new(mut bitmap: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bitmap))))
    }
    

    pub fn getPixels(&mut self, mut pixels: Vec<i32>, mut offset: i32, mut stride: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        jcall!(self.bitmap).getPixels(pixels.clone(), offset.clone(), stride.clone(), x.clone(), y.clone(), width.clone(), height.clone());
    }
    

    pub fn setPixels(&mut self, mut pixels: Vec<i32>, mut offset: i32, mut stride: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        jcall!(self.bitmap).setPixels(pixels.clone(), offset.clone(), stride.clone(), x.clone(), y.clone(), width.clone(), height.clone());
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return jcall!(self.bitmap).getWidth();
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return jcall!(self.bitmap).getHeight();
    }
    

    pub fn recycle(&mut self) {
        jcall!(self.bitmap).recycle();
    }
}
impl std::ops::Deref for OpenGLBitmap {
    type Target = PlatformBitmapBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLBitmap {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn for OpenGLBitmap {
    fn as_PlatformBitmapBase(&self) -> &crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBase { &self.base }
    fn as_PlatformBitmapBase_mut(&mut self) -> &mut crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBase { &mut self.base }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
    fn getHeight(&mut self) -> i32 { Self::getHeight(self) }
}


