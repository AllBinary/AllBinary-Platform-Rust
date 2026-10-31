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
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::SensorFeature::SensorFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SensorFeatureFactory {
    pub NO_ORIENTATION: Option<Rc<RefCell<SensorFeature>>>,
    pub ORIENTATION_SENSORS: Option<Rc<RefCell<SensorFeature>>>,
    pub SIMULATED_ORIENTATION_SENSORS: Option<Rc<RefCell<SensorFeature>>>,
    pub YAW: Option<Rc<RefCell<SensorFeature>>>,
    pub YAW_MINUS_RIGHT_PLUS_LEFT: Option<Rc<RefCell<SensorFeature>>>,
    pub YAW_MINUS_LEFT_PLUS_RIGHT: Option<Rc<RefCell<SensorFeature>>>,
    pub PITCH: Option<Rc<RefCell<SensorFeature>>>,
    pub ROLL: Option<Rc<RefCell<SensorFeature>>>,
}

thread_local! { pub static SensorFeatureFactory_instance: Option<Rc<RefCell<SensorFeatureFactory>>> = SensorFeatureFactory::new(); }

impl SensorFeatureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<SensorFeatureFactory>>> {
        return SensorFeatureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NO_ORIENTATION: None,
            ORIENTATION_SENSORS: None,
            SIMULATED_ORIENTATION_SENSORS: None,
            YAW: None,
            YAW_MINUS_RIGHT_PLUS_LEFT: None,
            YAW_MINUS_LEFT_PLUS_RIGHT: None,
            PITCH: None,
            ROLL: None,
        };
        this.NO_ORIENTATION = SensorFeature::new(String::from("No Orientation"));
        this.ORIENTATION_SENSORS = SensorFeature::new(String::from("Orientation Sensors"));
        this.SIMULATED_ORIENTATION_SENSORS = SensorFeature::new(String::from("Simulated Orientation Sensors"));
        this.YAW = SensorFeature::new(String::from("Yaw"));
        this.YAW_MINUS_RIGHT_PLUS_LEFT = SensorFeature::new(String::from("Yaw -Right +Left"));
        this.YAW_MINUS_LEFT_PLUS_RIGHT = SensorFeature::new(String::from("Yaw -Left +Right "));
        this.PITCH = SensorFeature::new(String::from("Pitch"));
        this.ROLL = SensorFeature::new(String::from("Roll"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

