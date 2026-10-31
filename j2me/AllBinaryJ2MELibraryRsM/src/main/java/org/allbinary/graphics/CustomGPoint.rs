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
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomGPoint {
    pub base: GPoint,
    pub cx: i32,
    pub cy: i32,
    pub cz: i32,
}

thread_local! { pub static CustomGPoint_NULL_CUSTOM_POINT: Option<Rc<RefCell<CustomGPoint>>> = CustomGPoint::new(0, 0, 0); }

impl CustomGPoint {
    

    pub fn getInstance(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Option<Rc<RefCell<CustomGPoint>>> {
        return CustomGPoint::new(jcall!(point).getX(), jcall!(point).getY(), jcall!(point).getZ());
    }
    

    pub fn getInstance3(mut x: i32, mut y: i32) -> Option<Rc<RefCell<CustomGPoint>>> {
        return CustomGPoint::new(x.clone(), y.clone(), 3);
    }
    

    pub fn new_struct(mut x: i32, mut y: i32, mut z: i32) -> Self {
        let mut this = Self {
            base: GPoint::new_struct(x.clone(), y.clone(), z.clone()),
            cx: 0,
            cy: 0,
            cz: 0,
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(x, y, z))))
    }
    

    pub fn setX(&mut self, mut x: i32) {
        self.cx = x.clone();
    }
    

    pub fn getX(&mut self) -> i32 {
        return self.cx.clone();
    }
    

    pub fn setY(&mut self, mut y: i32) {
        self.cy = y.clone();
    }
    

    pub fn getY(&mut self) -> i32 {
        return self.cy.clone();
    }
    

    pub fn setZ(&mut self, mut z: i32) {
        self.cz = z.clone();
    }
    

    pub fn getZ(&mut self) -> i32 {
        return self.cz.clone();
    }
}
impl std::ops::Deref for CustomGPoint {
    type Target = GPoint;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomGPoint {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::GPoint::GPointDyn for CustomGPoint {
    fn as_GPoint(&self) -> &crate::org::allbinary::graphics::GPoint::GPoint { &self.base }
    fn as_GPoint_mut(&mut self) -> &mut crate::org::allbinary::graphics::GPoint::GPoint { &mut self.base }
    fn getX(&mut self) -> i32 { Self::getX(self) }
    fn getY(&mut self) -> i32 { Self::getY(self) }
    fn getZ(&mut self) -> i32 { Self::getZ(self) }
}


