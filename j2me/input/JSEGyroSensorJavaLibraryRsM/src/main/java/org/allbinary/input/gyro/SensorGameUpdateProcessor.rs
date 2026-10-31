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
use crate::org::allbinary::game::input::motion::action::NoCompleteMotionGestureInputEventListener::NoCompleteMotionGestureInputEventListener;
use crate::org::allbinary::input::AllBinarySensor::AllBinarySensor;
use crate::org::allbinary::input::NoAllBinarySensor::NoAllBinarySensor;
use crate::org::allbinary::input::accelerometer::AccelerometerSensorFactory::AccelerometerSensorFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SensorGameUpdateProcessor {
    pub inputSensor: Option<Rc<RefCell<dyn crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn>>>,
}


impl SensorGameUpdateProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            inputSensor: None,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setInputSensor(&mut self, mut inputSensor: Option<Rc<RefCell<dyn crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn>>>) {
        self.inputSensor = inputSensor.clone();
    }
    

    pub fn getInputSensor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn>>> {
        return self.inputSensor.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    // throws Exception
    pub fn sendNotifications(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn isAnySensor(&mut self) -> bool {
        if match (&self.inputSensor, &NoAllBinarySensor::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn setNoSensors(&mut self) {
        jcall!(GyroSensorFactory::GyroSensorFactory::getInstance()).setCompleteMotionGestureInputEventListener(NoCompleteMotionGestureInputEventListener::getInstance());
        jcall!(AccelerometerSensorFactory::getInstance()).setCompleteMotionGestureInputEventListener(NoCompleteMotionGestureInputEventListener::getInstance());
        { let java_arguments = (NoAllBinarySensor::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn>>),); self.setInputSensor(java_arguments.0) };
    }
}
pub trait SensorGameUpdateProcessorDyn {
    fn as_SensorGameUpdateProcessor(&self) -> &SensorGameUpdateProcessor;
    fn as_SensorGameUpdateProcessor_mut(&mut self) -> &mut SensorGameUpdateProcessor;
    fn setInputSensor(&mut self, mut inputSensor: Option<Rc<RefCell<dyn crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn>>>) { SensorGameUpdateProcessor::setInputSensor(self.as_SensorGameUpdateProcessor_mut(), inputSensor) }
    fn getInputSensor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn>>> { SensorGameUpdateProcessor::getInputSensor(self.as_SensorGameUpdateProcessor_mut()) }
    fn process(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { SensorGameUpdateProcessor::process(self.as_SensorGameUpdateProcessor_mut(), anyType) }
    fn sendNotifications(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { SensorGameUpdateProcessor::sendNotifications(self.as_SensorGameUpdateProcessor_mut(), anyType) }
    fn isAnySensor(&mut self) -> bool { SensorGameUpdateProcessor::isAnySensor(self.as_SensorGameUpdateProcessor_mut()) }
    fn setNoSensors(&mut self) { SensorGameUpdateProcessor::setNoSensors(self.as_SensorGameUpdateProcessor_mut()) }
    fn setInputSensor(&mut self, mut inputSensor: Option<Rc<RefCell<dyn crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn>>>) { SensorGameUpdateProcessor::setInputSensor(self.as_SensorGameUpdateProcessor_mut(), inputSensor) }
    fn getInputSensor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn>>> { SensorGameUpdateProcessor::getInputSensor(self.as_SensorGameUpdateProcessor_mut()) }
    fn process(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { SensorGameUpdateProcessor::process(self.as_SensorGameUpdateProcessor_mut(), anyType) }
    fn sendNotifications(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { SensorGameUpdateProcessor::sendNotifications(self.as_SensorGameUpdateProcessor_mut(), anyType) }
    fn isAnySensor(&mut self) -> bool { SensorGameUpdateProcessor::isAnySensor(self.as_SensorGameUpdateProcessor_mut()) }
    fn setNoSensors(&mut self) { SensorGameUpdateProcessor::setNoSensors(self.as_SensorGameUpdateProcessor_mut()) }
}
impl SensorGameUpdateProcessorDyn for SensorGameUpdateProcessor {
    fn as_SensorGameUpdateProcessor(&self) -> &SensorGameUpdateProcessor { self }
    fn as_SensorGameUpdateProcessor_mut(&mut self) -> &mut SensorGameUpdateProcessor { self }
}


