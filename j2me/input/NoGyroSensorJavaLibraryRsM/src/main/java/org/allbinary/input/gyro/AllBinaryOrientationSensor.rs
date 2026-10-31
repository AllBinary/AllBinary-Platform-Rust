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
use crate::org::allbinary::game::input::CompleteMotionGestureInputEventListenerInterface::CompleteMotionGestureInputEventListenerInterface;
use crate::org::allbinary::game::input::motion::action::NoCompleteMotionGestureInputEventListener::NoCompleteMotionGestureInputEventListener;
use crate::org::allbinary::input::AllBinarySensor::AllBinarySensor;
use crate::org::allbinary::logic::math::Orientation::Orientation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryOrientationSensor {
    pub base: AllBinarySensor,
    pub listener: Option<Rc<RefCell<dyn CompleteMotionGestureInputEventListenerInterface>>>,
}

thread_local! { pub static AllBinaryOrientationSensor_NULL_ALLBINARY_ORIENTATION_SENSOR: Option<Rc<RefCell<dyn AllBinaryOrientationSensorDyn>>> = AllBinaryOrientationSensor::new().map(|reference| reference as Rc<RefCell<dyn AllBinaryOrientationSensorDyn>>); }

impl AllBinaryOrientationSensor {
    

    pub fn setListener(&mut self, mut completeMotionGestureInputEventListenerInterface: Option<Rc<RefCell<dyn CompleteMotionGestureInputEventListenerInterface>>>) {
        self.listener = completeMotionGestureInputEventListenerInterface.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinarySensor::new_struct(),
            listener: None,
        };
        this.listener = NoCompleteMotionGestureInputEventListener::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getOrientation(&mut self) -> Option<Rc<RefCell<Orientation>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
}
impl std::ops::Deref for AllBinaryOrientationSensor {
    type Target = AllBinarySensor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryOrientationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryOrientationSensorDyn: crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn {
    fn as_AllBinaryOrientationSensor(&self) -> &AllBinaryOrientationSensor;
    fn as_AllBinaryOrientationSensor_mut(&mut self) -> &mut AllBinaryOrientationSensor;
    fn setListener(&mut self, mut completeMotionGestureInputEventListenerInterface: Option<Rc<RefCell<dyn CompleteMotionGestureInputEventListenerInterface>>>) { AllBinaryOrientationSensor::setListener(self.as_AllBinaryOrientationSensor_mut(), completeMotionGestureInputEventListenerInterface) }
    fn getOrientation(&mut self) -> Option<Rc<RefCell<Orientation>>> { AllBinaryOrientationSensor::getOrientation(self.as_AllBinaryOrientationSensor_mut()) }
}
impl AllBinaryOrientationSensorDyn for AllBinaryOrientationSensor {
    fn as_AllBinaryOrientationSensor(&self) -> &AllBinaryOrientationSensor { self }
    fn as_AllBinaryOrientationSensor_mut(&mut self) -> &mut AllBinaryOrientationSensor { self }
}

impl crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn for AllBinaryOrientationSensor {
    fn as_AllBinarySensor(&self) -> &crate::org::allbinary::input::AllBinarySensor::AllBinarySensor { &self.base }
    fn as_AllBinarySensor_mut(&mut self) -> &mut crate::org::allbinary::input::AllBinarySensor::AllBinarySensor { &mut self.base }
}
impl crate::org::allbinary::input::AllBinarySensorListener::AllBinarySensorListener for AllBinaryOrientationSensor {
    fn onSensorChange(&mut self) { crate::org::allbinary::input::AllBinarySensor::AllBinarySensor::onSensorChange(crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn::as_AllBinarySensor_mut(self)) }
}


