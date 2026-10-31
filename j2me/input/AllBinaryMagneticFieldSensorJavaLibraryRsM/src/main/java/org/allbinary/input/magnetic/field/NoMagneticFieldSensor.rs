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
// package org.allbinary.input.magnetic.field
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::AllBinarySensorListener::AllBinarySensorListener;
use crate::org::allbinary::logic::math::Orientation::Orientation;
use super::AllBinaryMagneticFieldSensor::AllBinaryMagneticFieldSensor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoMagneticFieldSensor {
    pub base: AllBinaryMagneticFieldSensor,
}

impl AllBinarySensorListener for NoMagneticFieldSensor {
    fn onSensorChange(&mut self) { Self::onSensorChange(self) }
}

impl NoMagneticFieldSensor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryMagneticFieldSensor::new_struct(),
        };
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
impl std::ops::Deref for NoMagneticFieldSensor {
    type Target = AllBinaryMagneticFieldSensor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoMagneticFieldSensor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::magnetic::field::AllBinaryMagneticFieldSensor::AllBinaryMagneticFieldSensorDyn for NoMagneticFieldSensor {
    fn as_AllBinaryMagneticFieldSensor(&self) -> &crate::org::allbinary::input::magnetic::field::AllBinaryMagneticFieldSensor::AllBinaryMagneticFieldSensor { &self.base }
    fn as_AllBinaryMagneticFieldSensor_mut(&mut self) -> &mut crate::org::allbinary::input::magnetic::field::AllBinaryMagneticFieldSensor::AllBinaryMagneticFieldSensor { &mut self.base }
}
impl crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn for NoMagneticFieldSensor {
    fn as_AllBinarySensor(&self) -> &crate::org::allbinary::input::AllBinarySensor::AllBinarySensor { &self.base.base }
    fn as_AllBinarySensor_mut(&mut self) -> &mut crate::org::allbinary::input::AllBinarySensor::AllBinarySensor { &mut self.base.base }
}


