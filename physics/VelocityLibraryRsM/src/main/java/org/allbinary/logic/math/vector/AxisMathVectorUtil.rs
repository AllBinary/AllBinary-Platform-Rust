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
// package org.allbinary.logic.math.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::math::NoDecimalTrigTable::NoDecimalTrigTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AxisMathVectorUtil {
    pub noDecimalTrigTable: Option<Rc<RefCell<NoDecimalTrigTable>>>,
}

thread_local! { pub static AxisMathVectorUtil_instance: Option<Rc<RefCell<AxisMathVectorUtil>>> = AxisMathVectorUtil::new(); }

impl AxisMathVectorUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            noDecimalTrigTable: None,
        };
        this.noDecimalTrigTable = NoDecimalTrigTable::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AxisMathVectorUtil>>> {
        return AxisMathVectorUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn calculateX(&mut self, mut magnitude: i64, mut angle: i32) -> i64 {
        return magnitude * jcall!(self.noDecimalTrigTable).cos(angle.clone()) / 10_i64;
    }
    

    pub fn calculateY(&mut self, mut magnitude: i64, mut angle: i32) -> i64 {
        return magnitude * jcall!(self.noDecimalTrigTable).sin(angle.clone()) / 10_i64;
    }
    

    pub fn calculateZ(&mut self, mut magnitude: i64, mut otherAngle: i32) -> i64 {
        return magnitude * jcall!(self.noDecimalTrigTable).sin(otherAngle.clone()) / 10_i64;
    }
}

