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
// package org.allbinary.logic.math
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::PrimitiveLongSingleton::PrimitiveLongSingleton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PrimitiveLongUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub powerOfTen: i32,
    pub maxValue: i32,
    pub maxDigits: i32,
    pub charArray: Vec<char>,
    pub currentTotalDigits: i32,
    pub primitiveLongSingleton: Option<Rc<RefCell<PrimitiveLongSingleton>>>,
}


impl PrimitiveLongUtil {
    

    pub fn createPowerOfTen(mut powerOfTen: i32) -> Option<Rc<RefCell<PrimitiveLongUtil>>> {
        let mut primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>> = PrimitiveLongUtil::new(powerOfTen.clone(), jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
        if powerOfTen % 10 != 0 {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(primitiveLongUtil.clone()), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(format!("{}: {:?}", "Exception", (String::from("Max must be power of 10")))));
        }
        return primitiveLongUtil.clone();
    }
    

    pub fn new_struct(mut powerOfTen: i32, mut unused: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            powerOfTen: 0,
            maxValue: 0,
            maxDigits: 0,
            charArray: Vec::new(),
            currentTotalDigits: 0,
            primitiveLongSingleton: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.primitiveLongSingleton = PrimitiveLongSingleton::getInstance();
        this.maxDigits = jcall!(MathUtil::MathUtil::getInstance()).getTotalDigits(powerOfTen.clone());
        this.powerOfTen = powerOfTen.clone();
        this.maxValue = (powerOfTen * 10) - 1;
        this.charArray = vec!['\0'; (this.maxDigits) as usize];
        this
    }

    pub fn new(mut powerOfTen: i32, mut unused: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(powerOfTen, unused))))
    }
    

    pub fn getCharArray(&mut self, mut value: i32) -> Vec<char> {
        if value < 10 {
            self.setCurrentTotalDigits(1);
            return jcall!(self.primitiveLongSingleton).NUMBER_CHAR_ARRAYS[(value) as usize].clone();
        }
        if value > self.maxValue {
            self.setCurrentTotalDigits(3);
            return jcall!(self.primitiveLongSingleton).UNK.clone();
        }
        let mut index: i32 = 0;
        let mut div: i32 = self.powerOfTen.clone();
        let mut lastValue: i32 = 0;
        while div >= 1 {
            let mut digit: i32 = (value - lastValue) / div;
            if digit >= 0 && digit < 10 {
                if digit == 0 && index == 0 {
                } else {
                    self.charArray[({ let value = index; index += 1; value }) as usize] = jcall!(self.primitiveLongSingleton).NUMBER_CHAR_ARRAY[(digit) as usize].clone();
                }
                lastValue += digit * div;
            }
            div = div / 10;
        }
        self.setCurrentTotalDigits(index.clone());
        return self.charArray.clone();
    }
    

    pub fn setCurrentTotalDigits(&mut self, mut currentTotalDigits: i32) {
        self.currentTotalDigits = currentTotalDigits.clone();
    }
    

    pub fn getCurrentTotalDigits(&mut self) -> i32 {
        return self.currentTotalDigits.clone();
    }
    

    pub fn getMaxDigits(&mut self) -> i32 {
        return self.maxDigits.clone();
    }
}

