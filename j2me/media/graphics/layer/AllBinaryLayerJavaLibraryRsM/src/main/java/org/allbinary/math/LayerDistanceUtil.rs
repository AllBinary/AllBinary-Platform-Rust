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
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerDistanceUtil {
    pub mathUtil: Option<Rc<RefCell<MathUtil>>>,
}

thread_local! { pub static LayerDistanceUtil_instance: Option<Rc<RefCell<LayerDistanceUtil>>> = LayerDistanceUtil::new(); }

impl LayerDistanceUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            mathUtil: None,
        };
        this.mathUtil = MathUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerDistanceUtil>>> {
        return LayerDistanceUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getDistance(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut layerInterface2: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> i32 {
        let mut dx: i32 = (jcall!(layerInterface).getXP() + jcall!(layerInterface).getHalfWidth()) - (jcall!(layerInterface2).getXP() + jcall!(layerInterface2).getHalfWidth());
        let mut dy: i32 = (jcall!(layerInterface).getYP() + jcall!(layerInterface).getHalfHeight()) - (jcall!(layerInterface2).getYP() + jcall!(layerInterface2).getHalfHeight());
        return jcall!(self.mathUtil).sqrt((dx * dx) + (dy * dy));
    }
    

    pub fn getDistance3D(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut layerInterface2: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> i32 {
        let mut dx: i32 = (jcall!(layerInterface).getXP() + jcall!(layerInterface).getHalfWidth()) - (jcall!(layerInterface2).getXP() + jcall!(layerInterface2).getHalfWidth());
        let mut dy: i32 = (jcall!(layerInterface).getYP() + jcall!(layerInterface).getHalfHeight()) - (jcall!(layerInterface2).getYP() + jcall!(layerInterface2).getHalfHeight());
        let mut dz: i32 = (jcall!(layerInterface).getZP() + jcall!(layerInterface).getHalfDepth()) - (jcall!(layerInterface2).getZP() + jcall!(layerInterface2).getHalfDepth());
        return jcall!(self.mathUtil).sqrt((dx * dx) + (dy * dy) + (dz * dz));
    }
    

    pub fn getDistanceAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> i32 {
        let mut dx: i32 = jcall!(layerInterface).getXP() - jcall!(point).getX();
        let mut dy: i32 = jcall!(layerInterface).getYP() - jcall!(point).getY();
        return jcall!(self.mathUtil).sqrt((dx * dx) + (dy * dy));
    }
}

