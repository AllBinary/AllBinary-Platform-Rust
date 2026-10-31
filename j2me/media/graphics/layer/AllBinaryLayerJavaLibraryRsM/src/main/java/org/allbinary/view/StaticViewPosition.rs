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
use super::ViewPosition::ViewPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StaticViewPosition {
    pub base: ViewPosition,
}


impl StaticViewPosition {
    

    pub fn new_struct(mut x: i32, mut y: i32, mut z: i32) -> Self {
        let mut this = Self {
            base: ViewPosition::new_struct(x.clone(), y.clone(), z.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(x, y, z))))
    }
    

    pub fn getX(&mut self) -> i32 {
        return self.getRawX();
    }
    

    pub fn getY(&mut self) -> i32 {
        return self.getRawY();
    }
    

    pub fn getZ(&mut self) -> i32 {
        return self.getRawZ();
    }
}
impl std::ops::Deref for StaticViewPosition {
    type Target = ViewPosition;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StaticViewPosition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::view::ViewPosition::ViewPositionDyn for StaticViewPosition {
    fn as_ViewPosition(&self) -> &crate::org::allbinary::view::ViewPosition::ViewPosition { &self.base }
    fn as_ViewPosition_mut(&mut self) -> &mut crate::org::allbinary::view::ViewPosition::ViewPosition { &mut self.base }
}
impl crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn for StaticViewPosition {
    fn as_ViewPositionBase(&self) -> &crate::org::allbinary::view::ViewPositionBase::ViewPositionBase { &self.base.base }
    fn as_ViewPositionBase_mut(&mut self) -> &mut crate::org::allbinary::view::ViewPositionBase::ViewPositionBase { &mut self.base.base }
    fn getX2(&mut self) -> i32 { crate::org::allbinary::view::ViewPosition::ViewPosition::getX2(crate::org::allbinary::view::ViewPosition::ViewPositionDyn::as_ViewPosition_mut(self)) }
    fn getY2(&mut self) -> i32 { crate::org::allbinary::view::ViewPosition::ViewPosition::getY2(crate::org::allbinary::view::ViewPosition::ViewPositionDyn::as_ViewPosition_mut(self)) }
    fn getZ2(&mut self) -> i32 { crate::org::allbinary::view::ViewPosition::ViewPosition::getZ2(crate::org::allbinary::view::ViewPosition::ViewPositionDyn::as_ViewPosition_mut(self)) }
    fn setAllbinaryLayer(&mut self, p1: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { crate::org::allbinary::view::ViewPosition::ViewPosition::setAllbinaryLayer(crate::org::allbinary::view::ViewPosition::ViewPositionDyn::as_ViewPosition_mut(self), p1) }
}
impl crate::org::allbinary::graphics::GPoint::GPointDyn for StaticViewPosition {
    fn as_GPoint(&self) -> &crate::org::allbinary::graphics::GPoint::GPoint { &self.base.base.base }
    fn as_GPoint_mut(&mut self) -> &mut crate::org::allbinary::graphics::GPoint::GPoint { &mut self.base.base.base }
    fn getX(&mut self) -> i32 { Self::getX(self) }
    fn getY(&mut self) -> i32 { Self::getY(self) }
    fn getZ(&mut self) -> i32 { Self::getZ(self) }
}


