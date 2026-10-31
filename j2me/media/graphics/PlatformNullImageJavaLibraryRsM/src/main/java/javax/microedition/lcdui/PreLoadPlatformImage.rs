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
pub struct PreLoadPlatformImage {
    pub base: PlatformImage,
}

thread_local! { pub static PreLoadPlatformImage_instance: Option<Rc<RefCell<PreLoadPlatformImage>>> = PreLoadPlatformImage::new(); }

impl PreLoadPlatformImage {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PlatformImage::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PreLoadPlatformImage>>> {
        return PreLoadPlatformImage_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for PreLoadPlatformImage {
    type Target = PlatformImage;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreLoadPlatformImage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::javax::microedition::lcdui::PlatformImage::PlatformImageDyn for PreLoadPlatformImage {
    fn as_PlatformImage(&self) -> &crate::javax::microedition::lcdui::PlatformImage::PlatformImage { &self.base }
    fn as_PlatformImage_mut(&mut self) -> &mut crate::javax::microedition::lcdui::PlatformImage::PlatformImage { &mut self.base }
}


