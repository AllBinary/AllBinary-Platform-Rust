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
use super::PlatformBitmapBase::PlatformBitmapBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformBitmapBaseFactory {
}

thread_local! { pub static PlatformBitmapBaseFactory_NULL_PLATFORM_BITMAP_BASE_FACTORY: Option<Rc<RefCell<dyn PlatformBitmapBaseFactoryDyn>>> = PlatformBitmapBaseFactory::new().map(|reference| reference as Rc<RefCell<dyn PlatformBitmapBaseFactoryDyn>>); }

impl PlatformBitmapBaseFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn createBitmap(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn>>> {
        return PlatformBitmapBase_NULL_PLATFORM_BITMAP_BASE.with(Clone::clone).clone();
    }
}
pub trait PlatformBitmapBaseFactoryDyn {
    fn as_PlatformBitmapBaseFactory(&self) -> &PlatformBitmapBaseFactory;
    fn as_PlatformBitmapBaseFactory_mut(&mut self) -> &mut PlatformBitmapBaseFactory;
    fn createBitmap(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn>>> { PlatformBitmapBaseFactory::createBitmap(self.as_PlatformBitmapBaseFactory_mut(), image) }
    fn createBitmap(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn>>> { PlatformBitmapBaseFactory::createBitmap(self.as_PlatformBitmapBaseFactory_mut(), image) }
}
impl PlatformBitmapBaseFactoryDyn for PlatformBitmapBaseFactory {
    fn as_PlatformBitmapBaseFactory(&self) -> &PlatformBitmapBaseFactory { self }
    fn as_PlatformBitmapBaseFactory_mut(&mut self) -> &mut PlatformBitmapBaseFactory { self }
}


