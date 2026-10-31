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
use crate::org::allbinary::input::AllBinarySensorListener::AllBinarySensorListener;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::Orientation::Orientation;
use super::AllBinaryOrientationSensor::AllBinaryOrientationSensor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoGyroSensor {
    pub base: AllBinaryOrientationSensor,
}

impl AllBinarySensorListener for NoGyroSensor {
    fn onSensorChange(&mut self) { Self::onSensorChange(self) }
}

impl NoGyroSensor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryOrientationSensor::new_struct(),
        };
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getOrientation(&mut self) -> Option<Rc<RefCell<Orientation>>> {
        return unsafe { crate::org::allbinary::logic::math::Orientation::Orientation_NONE.clone() }.clone();
    }
    

    pub fn onSensorChange(&mut self) {
    }
}
impl std::ops::Deref for NoGyroSensor {
    type Target = AllBinaryOrientationSensor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoGyroSensor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn for NoGyroSensor {
    fn as_AllBinaryOrientationSensor(&self) -> &crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensor { &self.base }
    fn as_AllBinaryOrientationSensor_mut(&mut self) -> &mut crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensor { &mut self.base }
    fn getOrientation(&mut self) -> Option<Rc<RefCell<Orientation>>> { Self::getOrientation(self) }
}
impl crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn for NoGyroSensor {
    fn as_AllBinarySensor(&self) -> &crate::org::allbinary::input::AllBinarySensor::AllBinarySensor { &self.base.base }
    fn as_AllBinarySensor_mut(&mut self) -> &mut crate::org::allbinary::input::AllBinarySensor::AllBinarySensor { &mut self.base.base }
}


