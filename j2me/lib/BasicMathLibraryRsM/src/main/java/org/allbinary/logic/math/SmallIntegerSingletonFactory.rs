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
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SmallIntegerSingletonFactory {
    pub NEGATIVE_MAX: i32,
    pub POSITIVE_MAX: i32,
    pub INTEGER_ARRAY: Vec<Option<i32>>,
    pub STRING_ARRAY: Vec<String>,
    pub MIN: i32,
    pub lastMin: i32,
    pub lastNegativeMin: i32,
}

thread_local! { pub static SmallIntegerSingletonFactory_instance: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::new(); }

impl SmallIntegerSingletonFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<SmallIntegerSingletonFactory>>> {
        return SmallIntegerSingletonFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getMin(&mut self) -> i32 {
        let mut minAllowed: i32 = self.getMinAllowed();
        if self.MIN <= minAllowed {
            if J2MEUtil::isJ2ME() {
            } else if AndroidUtil::isAndroid() {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                jcall!(logUtil).putF(jcall!(StringMaker::new()).append(String::from("Android InputFactory was initialized before GameMidlet: ")).appendint(self.MIN.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getMin"));
                self.initWithRange(0x291_u32 as i32, 6);
            } else {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                jcall!(logUtil).putF(jcall!(StringMaker::new()).append(String::from("InputFactory was initialized before GameMidlet or KeyFactoryInitializer - Currently this is occurs on JS build by TouchMotionGestureFactory constructor: ")).appendint(self.MIN.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getMin"));
                self.initWithRange(0x2D0_u32 as i32, 6);
            }
        }
        if self.MIN <= minAllowed {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            jcall!(logUtil).put(String::from("This means you loaded the InputFactory before determining the platform input size requirements."), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getMin"), crate::java_into_object(format!("{}: {:?}", "Exception", ())));
        }
        return self.MIN.clone();
    }
    

    pub fn getMinAllowed(&mut self) -> i32 {
        if J2MEUtil::isJ2ME() {
            return 0;
        } else {
            if AndroidUtil::isAndroid() {
                return 0x101_u32 as i32;
            } else {
                return 23;
            }
        }
    }
    

    pub fn initWithRange(&mut self, mut value: i32, mut negativeValue: i32) {
        for index in (self.lastMin..=value - 1).rev() {
            self.INTEGER_ARRAY[(index + self.NEGATIVE_MAX) as usize] = Some((index.clone()) as i32);
        }
        for index in (self.lastNegativeMin..=negativeValue - 1).rev() {
            self.INTEGER_ARRAY[(index) as usize] = Some((-index) as i32);
        }
        if self.lastMin < value {
            self.lastMin = value.clone();
            self.MIN = value.clone();
        }
        if self.lastNegativeMin < negativeValue {
            self.lastNegativeMin = negativeValue.clone();
        }
    }
    

    pub fn init(&mut self) {
        if self.lastMin < self.POSITIVE_MAX || self.lastNegativeMin < self.NEGATIVE_MAX {
            for index in (self.lastMin..=self.POSITIVE_MAX - 1).rev() {
                self.INTEGER_ARRAY[(index + self.NEGATIVE_MAX) as usize] = Some((index.clone()) as i32);
            }
            for index in (self.lastNegativeMin..=self.NEGATIVE_MAX - 1).rev() {
                self.INTEGER_ARRAY[(index) as usize] = Some((-index) as i32);
            }
            self.lastMin = self.POSITIVE_MAX.clone();
            self.MIN = self.lastMin.clone();
            self.lastNegativeMin = self.NEGATIVE_MAX.clone();
        }
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NEGATIVE_MAX: 0,
            POSITIVE_MAX: 0,
            INTEGER_ARRAY: Vec::new(),
            STRING_ARRAY: Vec::new(),
            MIN: 0,
            lastMin: 0,
            lastNegativeMin: 0,
        };
        this.NEGATIVE_MAX = 500;
        this.POSITIVE_MAX = 0x2D1_u32 as i32;
        this.INTEGER_ARRAY = vec![None; (this.NEGATIVE_MAX + this.POSITIVE_MAX) as usize];
        this.STRING_ARRAY = vec![String::new(); (this.NEGATIVE_MAX + this.POSITIVE_MAX) as usize];
        this.MIN = 0;
        this.lastMin = 0;
        this.lastNegativeMin = 0;
        this.initWithRange(23, 0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getAt(&mut self, mut index: i32) -> Option<i32> {
        return self.INTEGER_ARRAY[(index + self.NEGATIVE_MAX) as usize].clone();
    }
    

    pub fn getAtNoThrow(&mut self, mut index: i32) -> Option<i32> {
        if index + self.NEGATIVE_MAX > (self.INTEGER_ARRAY.len() as i32) - 1 {
            return self.INTEGER_ARRAY[(-1 + self.NEGATIVE_MAX) as usize].clone();
        }
        return self.INTEGER_ARRAY[(index + self.NEGATIVE_MAX) as usize].clone();
    }
    

    pub fn createInstance(&mut self, mut index: i32) -> Option<i32> {
        let mut integer: Option<i32> = self.getAt(index.clone());
        if integer.is_none() {
            integer = Some((index.clone()) as i32);
        }
        return integer.clone();
    }
    

    pub fn getString(&mut self, mut index: i32) -> String {
        let mut i: i32 = index + self.NEGATIVE_MAX;
        if self.STRING_ARRAY[(i) as usize].is_empty() {
            self.STRING_ARRAY[(i) as usize] = (self.INTEGER_ARRAY[(i) as usize]).expect("NullPointerException").to_string();
        }
        return self.STRING_ARRAY[(i) as usize].clone();
    }
}

