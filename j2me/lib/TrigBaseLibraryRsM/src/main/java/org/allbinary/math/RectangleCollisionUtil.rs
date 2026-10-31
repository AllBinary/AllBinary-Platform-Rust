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
// package org.allbinary.math
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RectangleCollisionUtil {
}

thread_local! { pub static RectangleCollisionUtil_instance: Option<Rc<RefCell<RectangleCollisionUtil>>> = RectangleCollisionUtil::new(); }

impl RectangleCollisionUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<RectangleCollisionUtil>>> {
        return RectangleCollisionUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isCollision(&mut self, mut rectX1: i32, mut rectY1: i32, mut rectX2: i32, mut rectY2: i32, mut rect2X1: i32, mut rect2Y1: i32, mut rect2X2: i32, mut rect2Y2: i32) -> bool {
        if rect2X1 >= rectX2 || rect2Y1 >= rectY2 || rect2X2 <= rectX1 || rect2Y2 <= rectY1 {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn isInside(&mut self, mut rectX1: i32, mut rectY1: i32, mut rectX2: i32, mut rectY2: i32, mut x: i32, mut y: i32) -> bool {
        if x >= rectX2 || y >= rectY2 || x <= rectX1 || y <= rectY1 {
            return false;
        } else {
            return true;
        }
    }
}

