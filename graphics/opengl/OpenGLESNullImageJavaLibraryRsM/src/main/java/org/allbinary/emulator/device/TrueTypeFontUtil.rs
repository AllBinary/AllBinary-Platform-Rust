// 
// AllBinary Open License Version 1 
// Copyright (c) 2011 AllBinary 
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
// package org.allbinary.emulator.device
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;
use super::TrueTypeFontUtilBase::TrueTypeFontUtilBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TrueTypeFontUtil {
    pub base: TrueTypeFontUtilBase,
    pub fontImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>,
}

thread_local! { pub static TrueTypeFontUtil_instance: Option<Rc<RefCell<TrueTypeFontUtil>>> = TrueTypeFontUtil::new(); }

impl TrueTypeFontUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<TrueTypeFontUtil>>> {
        return TrueTypeFontUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TrueTypeFontUtilBase::new_struct(1),
            fontImage: None,
        };
        this.fontImage = crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_NULL_OPENGL_IMAGE.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getAsTextureSize(&mut self, mut textureSize: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCellSize(&mut self, mut cellSize: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getFontBitmap(&mut self, mut filename: String, mut fontSize: i32, mut cellSize: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Image>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn saveFontAtlasAsFile(&mut self) {
    }
    

    pub fn getFontBitmapGL(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut filename: String, mut cellSize: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Image>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getFontWidths(&mut self, mut filename: String, mut fontSize: i32) -> Vec<i32> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
impl std::ops::Deref for TrueTypeFontUtil {
    type Target = TrueTypeFontUtilBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TrueTypeFontUtil {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::emulator::device::TrueTypeFontUtilBase::TrueTypeFontUtilBaseDyn for TrueTypeFontUtil {
    fn as_TrueTypeFontUtilBase(&self) -> &crate::org::allbinary::emulator::device::TrueTypeFontUtilBase::TrueTypeFontUtilBase { &self.base }
    fn as_TrueTypeFontUtilBase_mut(&mut self) -> &mut crate::org::allbinary::emulator::device::TrueTypeFontUtilBase::TrueTypeFontUtilBase { &mut self.base }
    fn getAsTextureSize(&mut self, mut textureSize: i32) -> i32 { Self::getAsTextureSize(self, textureSize) }
    fn getCellSize(&mut self, mut cellSize: i32) -> i32 { Self::getCellSize(self, cellSize) }
}


