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
// package org.allbinary.game.terrain
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleFactory::AngleFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTerrainInfo {
    pub angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>,
}

thread_local! { pub static BasicTerrainInfo_NULL_BASIC_TERRAIN_INFO: Option<Rc<RefCell<BasicTerrainInfo>>> = BasicTerrainInfo::new(jcall!(AngleFactory::getInstance()).NOT_ANGLE.clone()); }

impl BasicTerrainInfo {
    

    pub fn new_struct(mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) -> Self {
        let mut this = Self {
            angle: None,
        };
        this.angle = angle.clone();
        this
    }

    pub fn new(mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(angle))))
    }
    

    pub fn getAngle(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> {
        return self.angle.clone();
    }
    

    pub fn setAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        self.angle = angle.clone();
    }
}

