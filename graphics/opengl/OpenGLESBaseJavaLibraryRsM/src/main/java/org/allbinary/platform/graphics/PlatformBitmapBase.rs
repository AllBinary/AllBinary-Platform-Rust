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
// package org.allbinary.platform.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformBitmapBase {
    pub alpha: i32,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
}

thread_local! { pub static PlatformBitmapBase_NULL_PLATFORM_BITMAP_BASE: Option<Rc<RefCell<dyn PlatformBitmapBaseDyn>>> = PlatformBitmapBase::new().map(|reference| reference as Rc<RefCell<dyn PlatformBitmapBaseDyn>>); }

impl PlatformBitmapBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
            alpha: 0,
            basicColor: None,
        };
        this.alpha = 255;
        this.basicColor = jcall!(BasicColorFactory::getInstance()).NULL_COLOR.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getImage(&mut self) -> Option<Rc<RefCell<Image>>> {
        return crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
    }
}
pub trait PlatformBitmapBaseDyn {
    fn as_PlatformBitmapBase(&self) -> &PlatformBitmapBase;
    fn as_PlatformBitmapBase_mut(&mut self) -> &mut PlatformBitmapBase;
    fn getWidth(&mut self) -> i32 { PlatformBitmapBase::getWidth(self.as_PlatformBitmapBase_mut()) }
    fn getHeight(&mut self) -> i32 { PlatformBitmapBase::getHeight(self.as_PlatformBitmapBase_mut()) }
    fn getImage(&mut self) -> Option<Rc<RefCell<Image>>> { PlatformBitmapBase::getImage(self.as_PlatformBitmapBase_mut()) }
}
impl PlatformBitmapBaseDyn for PlatformBitmapBase {
    fn as_PlatformBitmapBase(&self) -> &PlatformBitmapBase { self }
    fn as_PlatformBitmapBase_mut(&mut self) -> &mut PlatformBitmapBase { self }
}


