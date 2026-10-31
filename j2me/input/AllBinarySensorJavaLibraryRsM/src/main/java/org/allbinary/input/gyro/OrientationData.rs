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
// package org.allbinary.input.gyro
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrientationData {
    pub ORIENTATION_SENSOR_INPUT: String,
    pub ORIENTATION_TYPE: String,
}

thread_local! { pub static OrientationData_SINGLETON: Option<Rc<RefCell<OrientationData>>> = OrientationData::new(); }

impl OrientationData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ORIENTATION_SENSOR_INPUT: String::new(),
            ORIENTATION_TYPE: String::new(),
        };
        this.ORIENTATION_SENSOR_INPUT = String::from("Orientation Sensor Input");
        this.ORIENTATION_TYPE = String::from("Orientation Type");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OrientationData>>> {
        return OrientationData_SINGLETON.with(Clone::clone).clone();
    }
}

