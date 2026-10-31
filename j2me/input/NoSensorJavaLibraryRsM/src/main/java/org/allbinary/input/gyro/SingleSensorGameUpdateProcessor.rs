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
use super::SensorGameUpdateProcessor::SensorGameUpdateProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SingleSensorGameUpdateProcessor {
    pub base: SensorGameUpdateProcessor,
}


impl SingleSensorGameUpdateProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SensorGameUpdateProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    // throws Exception
    pub fn sendNotifications(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}
impl std::ops::Deref for SingleSensorGameUpdateProcessor {
    type Target = SensorGameUpdateProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SingleSensorGameUpdateProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::gyro::SensorGameUpdateProcessor::SensorGameUpdateProcessorDyn for SingleSensorGameUpdateProcessor {
    fn as_SensorGameUpdateProcessor(&self) -> &crate::org::allbinary::input::gyro::SensorGameUpdateProcessor::SensorGameUpdateProcessor { &self.base }
    fn as_SensorGameUpdateProcessor_mut(&mut self) -> &mut crate::org::allbinary::input::gyro::SensorGameUpdateProcessor::SensorGameUpdateProcessor { &mut self.base }
    fn process(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::process(self, anyType) }
    fn sendNotifications(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::sendNotifications(self, anyType) }
}


