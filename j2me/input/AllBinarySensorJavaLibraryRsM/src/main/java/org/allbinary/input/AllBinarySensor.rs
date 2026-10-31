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
// package org.allbinary.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AllBinarySensorListener::AllBinarySensorListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinarySensor {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static AllBinarySensor_NULL_ALLBINARY_SENSOR: Option<Rc<RefCell<dyn AllBinarySensorDyn>>> = AllBinarySensor::new().map(|reference| reference as Rc<RefCell<dyn AllBinarySensorDyn>>); }
pub static mut AllBinarySensor_max: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 100);
impl AllBinarySensorListener for AllBinarySensor {
    fn onSensorChange(&mut self) { Self::onSensorChange(self) }
}

impl AllBinarySensor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
    }
    

    // throws Exception
    pub fn update(&mut self) {
    }
    

    pub fn setMax(mut max: i32) {
        unsafe { *AllBinarySensor_max = max.clone() };
    }
    

    pub fn getMax() -> i32 {
        return unsafe { AllBinarySensor_max.clone() }.clone();
    }
    

    pub fn getId(&mut self) -> i32 {
        return AllBinarySensor::getMax();
    }
    

    pub fn shutdown(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("shutdown"));
    }
    

    pub fn onSensorChange(&mut self) {
        ForcedLogUtil::log(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
}
pub trait AllBinarySensorDyn: crate::org::allbinary::input::AllBinarySensorListener::AllBinarySensorListener {
    fn as_AllBinarySensor(&self) -> &AllBinarySensor;
    fn as_AllBinarySensor_mut(&mut self) -> &mut AllBinarySensor;
    fn init(&mut self) { AllBinarySensor::init(self.as_AllBinarySensor_mut()) }
    fn update(&mut self) { AllBinarySensor::update(self.as_AllBinarySensor_mut()) }
    fn getId(&mut self) -> i32 { AllBinarySensor::getId(self.as_AllBinarySensor_mut()) }
    fn shutdown(&mut self) { AllBinarySensor::shutdown(self.as_AllBinarySensor_mut()) }
}
impl AllBinarySensorDyn for AllBinarySensor {
    fn as_AllBinarySensor(&self) -> &AllBinarySensor { self }
    fn as_AllBinarySensor_mut(&mut self) -> &mut AllBinarySensor { self }
}


