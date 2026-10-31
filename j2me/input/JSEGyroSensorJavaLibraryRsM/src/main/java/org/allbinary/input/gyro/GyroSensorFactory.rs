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
use super::AllBinaryOrientationSensor::AllBinaryOrientationSensor;
use super::NoGyroSensor::NoGyroSensor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GyroSensorFactory {
}

pub static mut GyroSensorFactory_allBinaryGyroSensor: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn>>>> = std::cell::LazyCell::new(|| None);

impl GyroSensorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init() {
        unsafe { *GyroSensorFactory_allBinaryGyroSensor = NoGyroSensor::new() };
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn>>> {
        return unsafe { GyroSensorFactory_allBinaryGyroSensor.clone() }.clone();
    }
}

