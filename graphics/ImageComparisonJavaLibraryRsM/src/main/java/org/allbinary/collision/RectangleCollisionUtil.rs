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
// package org.allbinary.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
use crate::org::allbinary::graphics::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RectangleCollisionUtil {
}


impl RectangleCollisionUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isCollision(mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool {
        if jcall!(point).getX() > (((jcall!(rectangle).x) as i32) + jcall!(rectangle).getWidth()) || jcall!(point).getY() > (((jcall!(rectangle).y) as i32) + jcall!(rectangle).getHeight()) || jcall!(point).getX() < jcall!(rectangle).x || jcall!(point).getY() < jcall!(rectangle).y {
            return false;
        } else {
            return true;
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn isCollision(mut rectangle1: Option<Rc<RefCell<Rectangle>>>, mut rectangle2: Option<Rc<RefCell<Rectangle>>>) -> bool {
        return RectangleCollisionUtil::isCollision8(jcall!(rectangle1).x.clone(), jcall!(rectangle1).y.clone(), jcall!(rectangle1).x + jcall!(rectangle1).width, jcall!(rectangle1).y + jcall!(rectangle1).height, jcall!(rectangle2).x.clone(), jcall!(rectangle2).y.clone(), jcall!(rectangle2).x + jcall!(rectangle2).width, jcall!(rectangle2).y + jcall!(rectangle2).height);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn isCollision8(mut rectX1: i32, mut rectY1: i32, mut rectX2: i32, mut rectY2: i32, mut rect2X1: i32, mut rect2Y1: i32, mut rect2X2: i32, mut rect2Y2: i32) -> bool {
        if rect2X1 >= rectX2 || rect2Y1 >= rectY2 || rect2X2 <= rectX1 || rect2Y2 <= rectY1 {
            return false;
        } else {
            return true;
        }
    }
}

