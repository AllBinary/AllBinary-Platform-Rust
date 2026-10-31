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
// package org.allbinary.graphics.j2me.workarea.canvas
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

pub struct IntegerDimension {
    pub width: i32,
    pub height: i32,
}


impl IntegerDimension {
    

    pub fn new_struct(mut width: i32, mut height: i32) -> Self {
        let mut this = Self {
            width: 0,
            height: 0,
        };
        this.setWidth(width.clone());
        this.setHeight(height.clone());
        this
    }

    pub fn new(mut width: i32, mut height: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(width, height))))
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return self.width.clone();
    }
    

    pub fn setWidth(&mut self, mut width: i32) {
        self.width = width.clone();
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return self.height.clone();
    }
    

    pub fn setHeight(&mut self, mut height: i32) {
        self.height = height.clone();
    }
}

