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
// package org.allbinary.input.accelerometer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::SensorFeatureFactory::SensorFeatureFactory;
use crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensor;
use super::NoAccelerometerSensor::NoAccelerometerSensor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AccelerometerSensorFactory {
}

pub static mut AccelerometerSensorFactory_allBinaryAccelerometerSensor: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn>>>> = std::cell::LazyCell::new(|| crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensor_NULL_ALLBINARY_ORIENTATION_SENSOR.with(Clone::clone).clone());

impl AccelerometerSensorFactory {

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
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut sensorFeatureFactory: Option<Rc<RefCell<SensorFeatureFactory>>> = SensorFeatureFactory::getInstance();
        if jcall!(features).isFeature(jcall!(sensorFeatureFactory).ORIENTATION_SENSORS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Orientation Sensors"))));
        } else if jcall!(features).isFeature(jcall!(sensorFeatureFactory).SIMULATED_ORIENTATION_SENSORS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Simulation Sensors"))));
        } else if jcall!(features).isFeature(jcall!(sensorFeatureFactory).NO_ORIENTATION.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            unsafe { *AccelerometerSensorFactory_allBinaryAccelerometerSensor = NoAccelerometerSensor::new() };
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Such SensorFeature"))));
        }
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::AllBinaryOrientationSensor::AllBinaryOrientationSensorDyn>>> {
        return unsafe { AccelerometerSensorFactory_allBinaryAccelerometerSensor.clone() }.clone();
    }
}

