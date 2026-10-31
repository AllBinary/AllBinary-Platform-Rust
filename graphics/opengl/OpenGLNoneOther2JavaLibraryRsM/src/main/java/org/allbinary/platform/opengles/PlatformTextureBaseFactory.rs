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
// package org.allbinary.platform.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformTextureBaseFactory {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static PlatformTextureBaseFactory_NULL_PLATFORM_TEXTURE_BASE_FACTORY: Option<Rc<RefCell<dyn PlatformTextureBaseFactoryDyn>>> = PlatformTextureBaseFactory::new().map(|reference| reference as Rc<RefCell<dyn PlatformTextureBaseFactoryDyn>>); }

impl PlatformTextureBaseFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn load(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut level: i32, mut image: Option<Rc<RefCell<Image>>>, mut border: i32, mut flip: bool) {
    }
}
pub trait PlatformTextureBaseFactoryDyn {
    fn as_PlatformTextureBaseFactory(&self) -> &PlatformTextureBaseFactory;
    fn as_PlatformTextureBaseFactory_mut(&mut self) -> &mut PlatformTextureBaseFactory;
    fn load(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut level: i32, mut image: Option<Rc<RefCell<Image>>>, mut border: i32, mut flip: bool) { PlatformTextureBaseFactory::load(self.as_PlatformTextureBaseFactory_mut(), gl, target, level, image, border, flip) }
    fn load(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut level: i32, mut image: Option<Rc<RefCell<Image>>>, mut border: i32, mut flip: bool) { PlatformTextureBaseFactory::load(self.as_PlatformTextureBaseFactory_mut(), gl, target, level, image, border, flip) }
}
impl PlatformTextureBaseFactoryDyn for PlatformTextureBaseFactory {
    fn as_PlatformTextureBaseFactory(&self) -> &PlatformTextureBaseFactory { self }
    fn as_PlatformTextureBaseFactory_mut(&mut self) -> &mut PlatformTextureBaseFactory { self }
}


