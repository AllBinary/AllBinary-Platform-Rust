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
use crate::org::allbinary::input::AllBinarySensor::AllBinarySensor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryMagneticFieldSensor {
    pub base: AllBinarySensor,
}


impl AllBinaryMagneticFieldSensor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinarySensor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for AllBinaryMagneticFieldSensor {
    type Target = AllBinarySensor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryMagneticFieldSensor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryMagneticFieldSensorDyn: crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn {
    fn as_AllBinaryMagneticFieldSensor(&self) -> &AllBinaryMagneticFieldSensor;
    fn as_AllBinaryMagneticFieldSensor_mut(&mut self) -> &mut AllBinaryMagneticFieldSensor;
}
impl AllBinaryMagneticFieldSensorDyn for AllBinaryMagneticFieldSensor {
    fn as_AllBinaryMagneticFieldSensor(&self) -> &AllBinaryMagneticFieldSensor { self }
    fn as_AllBinaryMagneticFieldSensor_mut(&mut self) -> &mut AllBinaryMagneticFieldSensor { self }
}

impl crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn for AllBinaryMagneticFieldSensor {
    fn as_AllBinarySensor(&self) -> &crate::org::allbinary::input::AllBinarySensor::AllBinarySensor { &self.base }
    fn as_AllBinarySensor_mut(&mut self) -> &mut crate::org::allbinary::input::AllBinarySensor::AllBinarySensor { &mut self.base }
}
impl crate::org::allbinary::input::AllBinarySensorListener::AllBinarySensorListener for AllBinaryMagneticFieldSensor {
    fn onSensorChange(&mut self) { crate::org::allbinary::input::AllBinarySensor::AllBinarySensor::onSensorChange(crate::org::allbinary::input::AllBinarySensor::AllBinarySensorDyn::as_AllBinarySensor_mut(self)) }
}


