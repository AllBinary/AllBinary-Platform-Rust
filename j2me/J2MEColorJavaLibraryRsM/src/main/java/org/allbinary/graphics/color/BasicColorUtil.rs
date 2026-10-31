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
use super::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicColorUtil {
    pub ZERO_ARRAY: Vec<Option<Rc<RefCell<BasicColor>>>>,
    pub ALPHA: i32,
    pub isAlpha: bool,
    pub ffOpaque: bool,
}

thread_local! { pub static BasicColorUtil_instance: Option<Rc<RefCell<BasicColorUtil>>> = BasicColorUtil::new(); }

impl BasicColorUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ZERO_ARRAY: Vec::new(),
            ALPHA: 0,
            isAlpha: false,
            ffOpaque: false,
        };
        this.ZERO_ARRAY = vec![None; (0) as usize];
        this.ALPHA = 0xFF000000_u32 as i32;
        this.isAlpha = true;
        this.ffOpaque = true;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicColorUtil>>> {
        return BasicColorUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getRGB(&mut self, mut red: i32, mut green: i32, mut blue: i32) -> i32 {
        let mut value: i32 = 0;
        if self.isAlpha {
            if self.ffOpaque {
                value = self.ALPHA.clone();
            }
        }
        value = (red << 16) | value;
        value = (green << 8) | value;
        value = blue | value;
        return value.clone();
    }
    

    pub fn getARGB(&mut self, mut alphaValue: i32, mut r: i32, mut g: i32, mut b: i32) -> i32 {
        return ((alphaValue << 24) & self.ALPHA) + ((r << 16) & 0x00FF0000_u32 as i32) + ((g << 8) & 0x0000FF00_u32 as i32) + (b & 0x000000FF_u32 as i32);
    }
    

    pub fn get(&mut self, mut alphaValue: i32, mut color: i32) -> i32 {
        return ((alphaValue << 24) & self.ALPHA) + color;
    }
    

    pub fn invert(&mut self, mut color: i32) -> i32 {
        return (0xFFFFFF_u32 as i32 - color) | self.ALPHA;
    }
}

