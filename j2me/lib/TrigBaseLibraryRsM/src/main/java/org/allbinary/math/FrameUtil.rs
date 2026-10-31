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
use super::AngleFactory::AngleFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FrameUtil {
}

thread_local! { pub static FrameUtil_instance: Option<Rc<RefCell<FrameUtil>>> = FrameUtil::new(); }

impl FrameUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<FrameUtil>>> {
        return FrameUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFrameForAngle(&mut self, mut angle: i16, mut angleIncrement: i32) -> i32 {
        return self.adjustAngleToFrameAngle(((angle) as i32)) / angleIncrement;
    }
    

    pub fn getFrameAngle(&mut self, mut frame: i32, mut angleIncrement: i32) -> i32 {
        let mut frameAngle: i32 = angleIncrement * frame - 90;
        return self.adjustAngleToFrameAngle(frameAngle.clone());
    }
    

    pub fn adjustAngleToFrameAngle(&mut self, mut currentAngle: i32) -> i32 {
        let mut angleFactory: Option<Rc<RefCell<AngleFactory>>> = AngleFactory::getInstance();
        if currentAngle > 359 {
            currentAngle = (currentAngle - ((jcall!(angleFactory).TOTAL_ANGLE) as i32));
            while currentAngle > 359 {
                currentAngle = (currentAngle - ((jcall!(angleFactory).TOTAL_ANGLE) as i32));
            }
        }
        if currentAngle < 0 {
            currentAngle = (currentAngle + ((jcall!(angleFactory).TOTAL_ANGLE) as i32));
            while currentAngle < 0 {
                currentAngle = (currentAngle + ((jcall!(angleFactory).TOTAL_ANGLE) as i32));
            }
        }
        return currentAngle.clone();
    }
}

