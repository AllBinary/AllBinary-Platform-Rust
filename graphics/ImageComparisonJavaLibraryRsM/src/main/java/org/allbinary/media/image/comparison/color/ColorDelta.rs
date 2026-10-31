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
// package org.allbinary.media.image.comparison.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorDelta {
    pub rgb1: i32,
    pub rgb2: i32,
    pub key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}

impl CacheableInterface for ColorDelta {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}

impl ColorDelta {
    

    pub fn new_struct(mut rgb1: i32, mut rgb2: i32) -> Self {
        let mut this = Self {
            rgb1: 0,
            rgb2: 0,
            key: None,
        };
        this.setRgb1(rgb1.clone());
        this.setRgb2(rgb2.clone());
        this.key = ColorDelta::getKey2(rgb1.clone(), rgb2.clone());
        this
    }

    pub fn new(mut rgb1: i32, mut rgb2: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(rgb1, rgb2))))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.key.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getKey2(mut rgb1: i32, mut rgb2: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(StringMaker::new()).append((rgb1.clone()).to_string()).append(jcall!(CommonSeps::getInstance()).UNDERSCORE.clone()).append((rgb2.clone()).to_string()).toString();
    }
    

    pub fn getRgb1(&mut self) -> i32 {
        return self.rgb1.clone();
    }
    

    pub fn setRgb1(&mut self, mut rgb1: i32) {
        self.rgb1 = rgb1.clone();
    }
    

    pub fn getRgb2(&mut self) -> i32 {
        return self.rgb2.clone();
    }
    

    pub fn setRgb2(&mut self, mut rgb2: i32) {
        self.rgb2 = rgb2.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(String::from("ColorDelta: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.getKey()))).append(String::from(" RGB1: ")).appendint(self.getRgb1()).append(String::from(" RGB2: ")).appendint(self.getRgb2()).toString();
    }
}

