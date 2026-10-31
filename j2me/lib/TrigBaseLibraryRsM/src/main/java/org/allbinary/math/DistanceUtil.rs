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
use crate::org::allbinary::logic::math::MathUtil::MathUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DistanceUtil {
    pub mathUtil: Option<Rc<RefCell<MathUtil>>>,
}

thread_local! { pub static DistanceUtil_instance: Option<Rc<RefCell<DistanceUtil>>> = DistanceUtil::new(); }

impl DistanceUtil {

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
    

    pub fn getInstance() -> Option<Rc<RefCell<DistanceUtil>>> {
        return DistanceUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getDistance(&mut self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) -> i32 {
        let mut dx: i32 = x1 - x2;
        let mut dy: i32 = y1 - y2;
        return jcall!(self.mathUtil).sqrt((dx * dx) + (dy * dy));
    }
}

