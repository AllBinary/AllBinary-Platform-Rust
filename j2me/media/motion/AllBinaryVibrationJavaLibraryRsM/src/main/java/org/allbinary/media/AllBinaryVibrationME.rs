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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryVibrationME {
}


impl AllBinaryVibrationME {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn vibrate(&mut self, mut duration: i32, mut r#type: i32, mut volume: i32) {
    }
}
pub trait AllBinaryVibrationMEDyn {
    fn as_AllBinaryVibrationME(&self) -> &AllBinaryVibrationME;
    fn as_AllBinaryVibrationME_mut(&mut self) -> &mut AllBinaryVibrationME;
    fn vibrate(&mut self, mut duration: i32, mut r#type: i32, mut volume: i32) { AllBinaryVibrationME::vibrate(self.as_AllBinaryVibrationME_mut(), duration, r#type, volume) }
}
impl AllBinaryVibrationMEDyn for AllBinaryVibrationME {
    fn as_AllBinaryVibrationME(&self) -> &AllBinaryVibrationME { self }
    fn as_AllBinaryVibrationME_mut(&mut self) -> &mut AllBinaryVibrationME { self }
}


