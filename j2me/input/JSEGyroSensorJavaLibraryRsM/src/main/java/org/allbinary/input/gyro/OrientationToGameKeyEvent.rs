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
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::SensorFeatureFactory::SensorFeatureFactory;
use crate::org::allbinary::logic::math::Orientation::Orientation;
use super::AllBinaryOrientationSensor::AllBinaryOrientationSensor;
use super::NoGyroSensor::NoGyroSensor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrientationToGameKeyEvent {
    pub allBinaryGyroSensor: Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn>>>,
    pub MIN: i32,
}


impl OrientationToGameKeyEvent {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            allBinaryGyroSensor: None,
            MIN: 0,
        };
        this.MIN = 15;
        if jcall!(Features::getInstance()).isFeature(jcall!(SensorFeatureFactory::getInstance()).NO_ORIENTATION.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            this.allBinaryGyroSensor = NoGyroSensor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn>>);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Such Feature"))));
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setAllBinaryGyroSensor(&mut self, mut allBinaryGyroSensor: Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn>>>) {
        self.allBinaryGyroSensor = allBinaryGyroSensor.clone();
    }
    

    pub fn getAllBinaryGyroSensor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn>>> {
        return self.allBinaryGyroSensor.clone();
    }
    

    // throws Exception
    pub fn process(&mut self) {
    }
    

    // throws Exception
    pub fn updateGameKeyEvents(&mut self, mut orientation: Option<Rc<RefCell<Orientation>>>, mut yaw: i32, mut pitch: i32, mut roll: i32) {
    }
}

