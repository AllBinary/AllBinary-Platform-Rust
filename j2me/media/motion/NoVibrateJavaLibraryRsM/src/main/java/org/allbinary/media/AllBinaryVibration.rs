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
// package org.allbinary.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use super::AllBinaryVibrationME::AllBinaryVibrationME;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryVibration {
    pub base: AllBinaryVibrationME,
}

pub static mut AllBinaryVibration_VIBRATION: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn>>>> = std::cell::LazyCell::new(|| unsafe { AllBinaryNoVibration_NO_VIBRATION.clone() }.clone());

impl AllBinaryVibration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryVibrationME::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn>>> {
        return unsafe { AllBinaryVibration_VIBRATION.clone() }.clone();
    }
    

    pub fn init() {
        if ((jcall!(jcall!(GameConfigurationCentral::getInstance()).VIBRATION).getValue()).unwrap() as i32) == 0 {
            unsafe { *AllBinaryVibration_VIBRATION = unsafe { AllBinaryNoVibration_NO_VIBRATION.clone() }.clone() };
        } else {
            unsafe { *AllBinaryVibration_VIBRATION = AllBinaryVibration::new() };
        }
    }
    

    pub fn vibrate(&mut self, mut duration: i32, mut r#type: i32, mut volume: i32) {
    }
}
impl std::ops::Deref for AllBinaryVibration {
    type Target = AllBinaryVibrationME;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryVibration {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn for AllBinaryVibration {
    fn as_AllBinaryVibrationME(&self) -> &crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationME { &self.base }
    fn as_AllBinaryVibrationME_mut(&mut self) -> &mut crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationME { &mut self.base }
    fn vibrate(&mut self, mut duration: i32, mut r#type: i32, mut volume: i32) { Self::vibrate(self, duration, r#type, volume) }
}


