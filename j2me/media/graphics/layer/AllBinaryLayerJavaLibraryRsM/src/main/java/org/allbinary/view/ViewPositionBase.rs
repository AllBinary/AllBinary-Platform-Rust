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
// package org.allbinary.view
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ViewPositionBase {
    pub base: GPoint,
}

thread_local! { pub static ViewPositionBase_NULL_VIEW_POSITION: Option<Rc<RefCell<dyn ViewPositionBaseDyn>>> = ViewPositionBase::new(0, 0, 0).map(|reference| reference as Rc<RefCell<dyn ViewPositionBaseDyn>>); }

impl ViewPositionBase {
    

    pub fn new_struct(mut x: i32, mut y: i32, mut z: i32) -> Self {
        let mut this = Self {
            base: GPoint::new_struct(x.clone(), y.clone(), z.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(x, y, z))))
    }
    

    pub fn getX2(&mut self) -> i32 {
        return self.getX();
    }
    

    pub fn getY2(&mut self) -> i32 {
        return self.getY();
    }
    

    pub fn getZ2(&mut self) -> i32 {
        return self.getZ();
    }
    

    pub fn setAllbinaryLayer(&mut self, mut allbinaryLayer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}
impl std::ops::Deref for ViewPositionBase {
    type Target = GPoint;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ViewPositionBase {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ViewPositionBaseDyn: crate::org::allbinary::graphics::GPoint::GPointDyn {
    fn as_ViewPositionBase(&self) -> &ViewPositionBase;
    fn as_ViewPositionBase_mut(&mut self) -> &mut ViewPositionBase;
    fn getX2(&mut self) -> i32 { ViewPositionBase::getX2(self.as_ViewPositionBase_mut()) }
    fn getY2(&mut self) -> i32 { ViewPositionBase::getY2(self.as_ViewPositionBase_mut()) }
    fn getZ2(&mut self) -> i32 { ViewPositionBase::getZ2(self.as_ViewPositionBase_mut()) }
    fn setAllbinaryLayer(&mut self, mut allbinaryLayer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { ViewPositionBase::setAllbinaryLayer(self.as_ViewPositionBase_mut(), allbinaryLayer) }
}
impl ViewPositionBaseDyn for ViewPositionBase {
    fn as_ViewPositionBase(&self) -> &ViewPositionBase { self }
    fn as_ViewPositionBase_mut(&mut self) -> &mut ViewPositionBase { self }
}

impl crate::org::allbinary::graphics::GPoint::GPointDyn for ViewPositionBase {
    fn as_GPoint(&self) -> &crate::org::allbinary::graphics::GPoint::GPoint { &self.base }
    fn as_GPoint_mut(&mut self) -> &mut crate::org::allbinary::graphics::GPoint::GPoint { &mut self.base }
}


