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
// package javax.microedition.lcdui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::PlatformImage::PlatformImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//PlatformNullImageJavaLibrary
pub struct PostLoadPlatformImage {
    pub base: PlatformImage,
}

thread_local! { pub static PostLoadPlatformImage_instance: Option<Rc<RefCell<dyn PostLoadPlatformImageDyn>>> = PostLoadPlatformImage::new().map(|reference| reference as Rc<RefCell<dyn PostLoadPlatformImageDyn>>); }

impl PostLoadPlatformImage {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PlatformImage::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn PostLoadPlatformImageDyn>>> {
        return PostLoadPlatformImage_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for PostLoadPlatformImage {
    type Target = PlatformImage;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PostLoadPlatformImage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PostLoadPlatformImageDyn: crate::javax::microedition::lcdui::PlatformImage::PlatformImageDyn {
    fn as_PostLoadPlatformImage(&self) -> &PostLoadPlatformImage;
    fn as_PostLoadPlatformImage_mut(&mut self) -> &mut PostLoadPlatformImage;
}
impl PostLoadPlatformImageDyn for PostLoadPlatformImage {
    fn as_PostLoadPlatformImage(&self) -> &PostLoadPlatformImage { self }
    fn as_PostLoadPlatformImage_mut(&mut self) -> &mut PostLoadPlatformImage { self }
}

impl crate::javax::microedition::lcdui::PlatformImage::PlatformImageDyn for PostLoadPlatformImage {
    fn as_PlatformImage(&self) -> &crate::javax::microedition::lcdui::PlatformImage::PlatformImage { &self.base }
    fn as_PlatformImage_mut(&mut self) -> &mut crate::javax::microedition::lcdui::PlatformImage::PlatformImage { &mut self.base }
}


