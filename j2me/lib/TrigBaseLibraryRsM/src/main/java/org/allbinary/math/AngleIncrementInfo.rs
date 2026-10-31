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
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::jsinterop::annotations::JsProperty;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use super::AngleFactory::AngleFactory;
use super::FrameUtil::FrameUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AngleIncrementInfo {
    pub angleIncrement: i16,
    pub DOWN_FRAME: Option<i32>,
    pub UP_FRAME: Option<i32>,
    pub LEFT_FRAME: Option<i32>,
    pub RIGHT_FRAME: Option<i32>,
    pub frameUtil: Option<Rc<RefCell<FrameUtil>>>,
}

thread_local! { pub static AngleIncrementInfo_INCREMENT: String = String::from("Inc: "); }

impl AngleIncrementInfo {
    

    pub fn new_struct(mut angleIncrement: i16) -> Self {
        let mut this = Self {
            angleIncrement: 0,
            DOWN_FRAME: None,
            UP_FRAME: None,
            LEFT_FRAME: None,
            RIGHT_FRAME: None,
            frameUtil: None,
        };
        this.frameUtil = FrameUtil::getInstance();
        this.angleIncrement = angleIncrement.clone();
        let mut angleFactory: Option<Rc<RefCell<AngleFactory>>> = AngleFactory::getInstance();
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        this.DOWN_FRAME = jcall!(smallIntegerSingletonFactory).getAt(((jcall!(jcall!(angleFactory).DOWN).getValue()) as i32) / ((this.angleIncrement) as i32));
        this.UP_FRAME = jcall!(smallIntegerSingletonFactory).getAt(((jcall!(jcall!(angleFactory).UP).getValue()) as i32));
        this.LEFT_FRAME = jcall!(smallIntegerSingletonFactory).getAt(((jcall!(jcall!(angleFactory).LEFT).getValue()) as i32) / ((this.angleIncrement) as i32));
        this.RIGHT_FRAME = jcall!(smallIntegerSingletonFactory).getAt(((jcall!(jcall!(angleFactory).RIGHT).getValue()) as i32) / ((this.angleIncrement) as i32));
        this
    }

    pub fn new(mut angleIncrement: i16) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(angleIncrement))))
    }
    

    pub fn getFrameAngle(&mut self, mut frame: i32) -> i32 {
        return jcall!(self.frameUtil).getFrameAngle(frame.clone(), ((self.angleIncrement) as i32));
    }
    

    pub fn getAngleIncrement(&mut self) -> i16 {
        return self.angleIncrement.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(AngleIncrementInfo_INCREMENT.with(Clone::clone).clone()).appendshort(self.angleIncrement.clone()).toString();
    }
    

    pub fn getClosestGeneralDirection(&mut self, mut angle: i16) -> i32 {
        let mut closestDirection: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(360);
        if ((self.UP_FRAME).unwrap() as i32) - ((angle) as i32) < ((closestDirection).unwrap() as i32) - ((angle) as i32) {
            closestDirection = self.UP_FRAME.clone();
        }
        if ((self.DOWN_FRAME).unwrap() as i32) - ((angle) as i32) < ((closestDirection).unwrap() as i32) - ((angle) as i32) {
            closestDirection = self.DOWN_FRAME.clone();
        }
        if ((self.LEFT_FRAME).unwrap() as i32) - ((angle) as i32) < ((closestDirection).unwrap() as i32) - ((angle) as i32) {
            closestDirection = self.LEFT_FRAME.clone();
        }
        if ((self.RIGHT_FRAME).unwrap() as i32) - ((angle) as i32) < ((closestDirection).unwrap() as i32) - ((angle) as i32) {
            closestDirection = self.RIGHT_FRAME.clone();
        }
        return ((closestDirection).unwrap() as i32);
    }
}

