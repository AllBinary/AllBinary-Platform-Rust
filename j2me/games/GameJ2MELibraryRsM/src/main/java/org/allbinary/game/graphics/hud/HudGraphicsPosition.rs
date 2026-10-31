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
// package org.allbinary.game.graphics.hud
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HudGraphicsPosition {
    pub anchor: i32,
    pub point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
}

thread_local! { pub static HudGraphicsPosition_NULL_HUD_GRAPHICS_POSITION: Option<Rc<RefCell<HudGraphicsPosition>>> = HudGraphicsPosition::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), 9); }

impl HudGraphicsPosition {
    

    pub fn new_struct(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut anchor: i32) -> Self {
        let mut this = Self {
            anchor: 0,
            point: None,
        };
        this.point = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
        this.setAnchor(anchor.clone());
        this.setPoint(point.clone());
        this
    }

    pub fn new(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut anchor: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(point, anchor))))
    }
    

    pub fn getAnchor(&mut self) -> i32 {
        return self.anchor.clone();
    }
    

    pub fn setAnchor(&mut self, mut anchor: i32) {
        self.anchor = anchor.clone();
    }
    

    pub fn getPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.point.clone();
    }
    

    pub fn setPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.point = point.clone();
    }
}

