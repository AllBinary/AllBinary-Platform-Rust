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
// package org.allbinary.graphics.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SmallBasicColorCacheFactory {
    pub SIZE: i32,
    pub INDEX_TO_COLOR: Vec<i32>,
    pub BASIC_COLOR_ARRAY: Vec<Option<Rc<RefCell<BasicColor>>>>,
    pub colorIndex: i32,
}

thread_local! { pub static SmallBasicColorCacheFactory_instance: Option<Rc<RefCell<SmallBasicColorCacheFactory>>> = SmallBasicColorCacheFactory::new(); }

impl SmallBasicColorCacheFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<SmallBasicColorCacheFactory>>> {
        return SmallBasicColorCacheFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            SIZE: 0,
            INDEX_TO_COLOR: Vec::new(),
            BASIC_COLOR_ARRAY: Vec::new(),
            colorIndex: 0,
        };
        this.SIZE = 255;
        this.INDEX_TO_COLOR = vec![0; (this.SIZE) as usize];
        this.BASIC_COLOR_ARRAY = vec![None; (this.SIZE) as usize];
        this.colorIndex = 0;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut basicDefaultColor: Option<Rc<RefCell<BasicColor>>>) {
        self.BASIC_COLOR_ARRAY[(self.colorIndex) as usize] = basicDefaultColor.clone();
        self.INDEX_TO_COLOR[(self.colorIndex) as usize] = jcall!(basicDefaultColor).intValue();
        self.colorIndex += 1;
    }
    

    pub fn getAndOrCreate(&mut self, mut colorAsInt: i32) -> Option<Rc<RefCell<BasicColor>>> {
        let mut basicColor: Option<Rc<RefCell<BasicColor>>> = None;
        for index in (0..self.SIZE) {
            if self.INDEX_TO_COLOR[(index) as usize] == colorAsInt {
                return self.BASIC_COLOR_ARRAY[(index) as usize].clone();
            }
        }
        let mut ALPHA_MASK: i32 = 0xFF000000_u32 as i32;
        basicColor = jcall!(BasicColorFactory::BasicColorFactory::getInstance()).createInstanceAN(colorAsInt & ALPHA_MASK, colorAsInt & 0x00FFFFFF_u32 as i32, jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        self.add(basicColor.clone());
        return basicColor.clone();
    }
}

