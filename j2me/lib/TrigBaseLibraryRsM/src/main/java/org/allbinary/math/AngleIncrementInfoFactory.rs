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
use super::AngleIncrementInfo::AngleIncrementInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AngleIncrementInfoFactory {
    pub angleIncrementInfo: Vec<Option<Rc<RefCell<AngleIncrementInfo>>>>,
}

thread_local! { pub static AngleIncrementInfoFactory_instance: Option<Rc<RefCell<AngleIncrementInfoFactory>>> = AngleIncrementInfoFactory::new(); }

impl AngleIncrementInfoFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            angleIncrementInfo: Vec::new(),
        };
        this.angleIncrementInfo = vec![None; (((jcall!(AngleFactory::AngleFactory::getInstance()).TOTAL_ANGLE) as i32)) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AngleIncrementInfoFactory>>> {
        return AngleIncrementInfoFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getAt(&mut self, mut angleIncrement: i16) -> Option<Rc<RefCell<AngleIncrementInfo>>> {
        let mut halfAngleIncrement: i32 = (((angleIncrement) as i32) >> 1);
        let mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>> = None;
        let mut angleIncrementInfoCanBeNull: Option<Rc<RefCell<AngleIncrementInfo>>> = self.angleIncrementInfo[(halfAngleIncrement) as usize].clone();
        if angleIncrementInfoCanBeNull.is_none() {
            angleIncrementInfo = AngleIncrementInfo::new(angleIncrement.clone());
            self.angleIncrementInfo[(halfAngleIncrement) as usize] = angleIncrementInfo.clone();
            return angleIncrementInfo.clone();
        } else {
            angleIncrementInfo = angleIncrementInfoCanBeNull.clone();
        }
        return angleIncrementInfo.clone();
    }
}

