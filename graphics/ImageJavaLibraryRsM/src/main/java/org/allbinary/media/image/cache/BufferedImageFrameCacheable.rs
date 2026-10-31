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
// package org.allbinary.media.image.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedImageFrameCacheable {
    pub frame: Option<i64>,
    pub bufferedImage: Option<Rc<RefCell<BufferedImage>>>,
}

impl CacheableInterface for BufferedImageFrameCacheable {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}

impl BufferedImageFrameCacheable {
    

    pub fn new_struct(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut frame: Option<i64>) -> Self {
        let mut this = Self {
            frame: None,
            bufferedImage: None,
        };
        this.frame = frame.clone();
        this.setBufferedImage(bufferedImage.clone());
        this
    }

    pub fn new(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut frame: Option<i64>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bufferedImage, frame))))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.frame.clone();
    }
    

    pub fn getBufferedImage(&mut self) -> Option<Rc<RefCell<BufferedImage>>> {
        return self.bufferedImage.clone();
    }
    

    pub fn setBufferedImage(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) {
        self.bufferedImage = bufferedImage.clone();
    }
    

    pub fn getFrame(&mut self) -> Option<i64> {
        return self.frame.clone();
    }
    

    pub fn setFrame(&mut self, mut frame: Option<i64>) {
        self.frame = frame.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(std::any::type_name_of_val(&self).to_string());
        jcall!(stringBuffer).append(String::from(" Frame: "));
        jcall!(stringBuffer).appendlong(self.getFrame());
        return jcall!(stringBuffer).toString();
    }
}

