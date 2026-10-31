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
// package org.allbinary.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
// use crate::javax::microedition::lcdui::Image::Image;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ImageCacheBaseAndroidJavaLibrary
pub struct ImageFactory {
}

thread_local! { pub static ImageFactory_instance: Option<Rc<RefCell<ImageFactory>>> = ImageFactory::new(); }

impl ImageFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageFactory>>> {
        return ImageFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn createImageUrl(&mut self, mut url: String) -> Option<Rc<RefCell<Image>>> {
        return Image::createImage(url.clone());
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        return Image::createImageWH(width.clone(), height.clone());
    }
    

    // throws Exception
    pub fn createImageFromInputStream2(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<Image>>> {
        let mut image: Option<Rc<RefCell<Image>>> = Image::createImageFromInputStream(inputStream.clone());
        return image.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createImageFromInputStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<Image>>> {
        return Image::createImageFromInputStream(inputStream.clone());
    }
}

