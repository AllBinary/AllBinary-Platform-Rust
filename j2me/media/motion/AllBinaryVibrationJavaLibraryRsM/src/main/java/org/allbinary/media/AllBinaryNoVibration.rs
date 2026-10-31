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
use super::AllBinaryVibrationME::AllBinaryVibrationME;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryNoVibration {
    pub base: AllBinaryVibrationME,
}

pub static mut AllBinaryNoVibration_NO_VIBRATION: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn>>>> = std::cell::LazyCell::new(|| AllBinaryNoVibration::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn>>));

impl AllBinaryNoVibration {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryVibrationME::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn vibrate(&mut self, mut duration: i32, mut r#type: i32, mut volume: i32) {
    }
}
impl std::ops::Deref for AllBinaryNoVibration {
    type Target = AllBinaryVibrationME;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryNoVibration {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn for AllBinaryNoVibration {
    fn as_AllBinaryVibrationME(&self) -> &crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationME { &self.base }
    fn as_AllBinaryVibrationME_mut(&mut self) -> &mut crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationME { &mut self.base }
    fn vibrate(&mut self, mut duration: i32, mut r#type: i32, mut volume: i32) { Self::vibrate(self, duration, r#type, volume) }
}


