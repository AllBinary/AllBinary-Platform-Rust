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
// package org.allbinary.media.image.io
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::javax::imageio::IIOImage::IIOImage;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IIOImageCacheable {
    pub key: String,
    pub iioImage: Option<Rc<RefCell<IIOImage>>>,
}

impl CacheableInterface for IIOImageCacheable {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}

impl IIOImageCacheable {
    

    pub fn new_struct(mut key: String) -> Self {
        let mut this = Self {
            key: String::new(),
            iioImage: None,
        };
        this.key = key.clone();
        this
    }

    pub fn new(mut key: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(key))))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.key.clone();
    }
    

    pub fn setBufferedImage(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) {
        self.setIioImage(IIOImage::new(bufferedImage.clone(), None, None));
    }
    

    pub fn getIioImage(&mut self) -> Option<Rc<RefCell<IIOImage>>> {
        return self.iioImage.clone();
    }
    

    pub fn setIioImage(&mut self, mut iioImage: Option<Rc<RefCell<IIOImage>>>) {
        self.iioImage = iioImage.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(std::any::type_name_of_val(&self).to_string());
        jcall!(stringBuffer).append(String::from(" Reusable IIOImage"));
        return jcall!(stringBuffer).toString();
    }
}

