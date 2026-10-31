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
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::RectangleCollisionUtil::RectangleCollisionUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RectangleCollisionPointUtil {
}


impl RectangleCollisionPointUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn allPointsInside(mut rectX1: i32, mut rectY1: i32, mut rectX2: i32, mut rectY2: i32, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> bool {
        let mut rectangleCollisionUtil: Option<Rc<RefCell<RectangleCollisionUtil>>> = RectangleCollisionUtil::getInstance();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        {
            let mut index: i32 = jcall!(list).size();
            while { index -= 1; index } >= 0 {
            point = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            if !jcall!(rectangleCollisionUtil).isInside(rectX1.clone(), rectY1.clone(), rectX2.clone(), rectY2.clone(), jcall!(point).getX(), jcall!(point).getY()) {
                return false;
            }
            }
        }
        return true;
    }
    

    pub fn allPointsInsideCell(mut rectX1: i32, mut rectY1: i32, mut rectX2: i32, mut rectY2: i32, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut xCellSize: i32, mut yCellSize: i32) -> bool {
        let mut rectangleCollisionUtil: Option<Rc<RefCell<RectangleCollisionUtil>>> = RectangleCollisionUtil::getInstance();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        {
            let mut index: i32 = jcall!(list).size();
            while { index -= 1; index } >= 0 {
            point = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            if !jcall!(rectangleCollisionUtil).isInside(rectX1.clone(), rectY1.clone(), rectX2.clone(), rectY2.clone(), jcall!(point).getX() * xCellSize, jcall!(point).getY() * yCellSize) {
                return false;
            }
            }
        }
        return true;
    }
}

