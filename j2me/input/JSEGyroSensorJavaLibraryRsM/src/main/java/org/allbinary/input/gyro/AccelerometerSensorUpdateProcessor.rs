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
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::SensorFeatureFactory::SensorFeatureFactory;
use crate::org::allbinary::game::input::motion::action::NoCompleteMotionGestureInputEventListener::NoCompleteMotionGestureInputEventListener;
use crate::org::allbinary::game::input::motion::action::SingleKeyPressGameKeyFromMotionGestureEventListener::SingleKeyPressGameKeyFromMotionGestureEventListener;
use crate::org::allbinary::input::accelerometer::AccelerometerSensorFactory::AccelerometerSensorFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AccelerometerSensorUpdateProcessor {
    pub base: Processor,
}


impl AccelerometerSensorUpdateProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self) {
        let mut sensorFeatureFactory: Option<Rc<RefCell<SensorFeatureFactory>>> = SensorFeatureFactory::getInstance();
        if jcall!(Features::getInstance()).isFeature(jcall!(sensorFeatureFactory).ORIENTATION_SENSORS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(AccelerometerSensorFactory::getInstance()).setCompleteMotionGestureInputEventListener(SingleKeyPressGameKeyFromMotionGestureEventListener::new());
        } else if jcall!(Features::getInstance()).isFeature(jcall!(sensorFeatureFactory).SIMULATED_ORIENTATION_SENSORS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(AccelerometerSensorFactory::getInstance()).setCompleteMotionGestureInputEventListener(SingleKeyPressGameKeyFromMotionGestureEventListener::new());
        } else if jcall!(Features::getInstance()).isFeature(jcall!(sensorFeatureFactory).NO_ORIENTATION.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(AccelerometerSensorFactory::getInstance()).setCompleteMotionGestureInputEventListener(NoCompleteMotionGestureInputEventListener::getInstance());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Sensor Feature Not Set"))));
        }
    }
}
impl std::ops::Deref for AccelerometerSensorUpdateProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AccelerometerSensorUpdateProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for AccelerometerSensorUpdateProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self.base }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


