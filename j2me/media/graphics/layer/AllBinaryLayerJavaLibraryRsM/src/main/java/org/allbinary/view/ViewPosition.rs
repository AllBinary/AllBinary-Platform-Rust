// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::ViewPositionBase::ViewPositionBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ViewPosition {
    pub base: ViewPositionBase,
    pub allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
}


impl ViewPosition {
    

    pub fn getInstanceD() -> Option<Rc<RefCell<dyn ViewPositionDyn>>> {
        return ViewPosition::new(0, 0, 0).map(|reference| reference as Rc<RefCell<dyn ViewPositionDyn>>);
    }
    

    pub fn new_struct(mut x: i32, mut y: i32, mut z: i32) -> Self {
        let mut this = Self {
            base: ViewPositionBase::new_struct(x.clone(), y.clone(), z.clone()),
            allbinaryLayer: None,
        };
        this.allbinaryLayer = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(x, y, z))))
    }
    

    pub fn getX(&mut self) -> i32 {
        return jcall!(self.allbinaryLayer).getXP();
    }
    

    pub fn getY(&mut self) -> i32 {
        return jcall!(self.allbinaryLayer).getYP();
    }
    

    pub fn getZ(&mut self) -> i32 {
        return jcall!(self.allbinaryLayer).getZP();
    }
    

    pub fn getX2(&mut self) -> i32 {
        return self.getX() + jcall!(self.allbinaryLayer).getWidth();
    }
    

    pub fn getY2(&mut self) -> i32 {
        return self.getY() + jcall!(self.allbinaryLayer).getHeight();
    }
    

    pub fn getZ2(&mut self) -> i32 {
        return (((((self.getZ()) as i64) + jcall!(self.allbinaryLayer).getDepth())) as i32);
    }
    

    pub fn setAllbinaryLayer(&mut self, mut allbinaryLayer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        self.allbinaryLayer = (allbinaryLayer).clone() /* as AllBinaryLayer */;
    }
}
impl std::ops::Deref for ViewPosition {
    type Target = ViewPositionBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ViewPosition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ViewPositionDyn: crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn {
    fn as_ViewPosition(&self) -> &ViewPosition;
    fn as_ViewPosition_mut(&mut self) -> &mut ViewPosition;
}
impl ViewPositionDyn for ViewPosition {
    fn as_ViewPosition(&self) -> &ViewPosition { self }
    fn as_ViewPosition_mut(&mut self) -> &mut ViewPosition { self }
}

impl crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn for ViewPosition {
    fn as_ViewPositionBase(&self) -> &crate::org::allbinary::view::ViewPositionBase::ViewPositionBase { &self.base }
    fn as_ViewPositionBase_mut(&mut self) -> &mut crate::org::allbinary::view::ViewPositionBase::ViewPositionBase { &mut self.base }
    fn getX2(&mut self) -> i32 { Self::getX2(self) }
    fn getY2(&mut self) -> i32 { Self::getY2(self) }
    fn getZ2(&mut self) -> i32 { Self::getZ2(self) }
    fn setAllbinaryLayer(&mut self, mut allbinaryLayer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::setAllbinaryLayer(self, allbinaryLayer) }
}
impl crate::org::allbinary::graphics::GPoint::GPointDyn for ViewPosition {
    fn as_GPoint(&self) -> &crate::org::allbinary::graphics::GPoint::GPoint { &self.base.base }
    fn as_GPoint_mut(&mut self) -> &mut crate::org::allbinary::graphics::GPoint::GPoint { &mut self.base.base }
    fn getX(&mut self) -> i32 { Self::getX(self) }
    fn getY(&mut self) -> i32 { Self::getY(self) }
    fn getZ(&mut self) -> i32 { Self::getZ(self) }
}


