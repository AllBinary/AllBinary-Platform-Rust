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
// package org.allbinary.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllMotionRecognizer {
    pub id: i32,
}

pub static mut AllMotionRecognizer_index: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);

impl AllMotionRecognizer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            id: 0,
        };
        this.id = unsafe { let value = AllMotionRecognizer_index.clone(); *AllMotionRecognizer_index += 1; value };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn processStartMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
    }
    

    // throws Exception
    pub fn processEndMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
    }
    

    // throws Exception
    pub fn processDraggedMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
    }
    

    // throws Exception
    pub fn processMovedMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
    }
    

    // throws Exception
    pub fn processScrolledMotionEvent(&mut self, mut x: i32, mut y: i32, mut deviceId: i32, mut modifiers: i32) {
    }
    

    pub fn getId(&mut self) -> i32 {
        return self.id.clone();
    }
}

