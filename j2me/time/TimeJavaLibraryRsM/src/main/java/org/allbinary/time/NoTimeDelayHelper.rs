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
// package org.allbinary.time
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::TimeDelayHelper::TimeDelayHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoTimeDelayHelper {
    pub base: TimeDelayHelper,
}

thread_local! { pub static NoTimeDelayHelper_SINGLETON: Option<Rc<RefCell<NoTimeDelayHelper>>> = NoTimeDelayHelper::new(); }

impl NoTimeDelayHelper {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TimeDelayHelper::new_struct(0),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isTimeTNT(&mut self) -> bool {
        return true;
    }
    

    pub fn isTimeSinceTNT(&mut self, mut delay: i32) -> bool {
        return true;
    }
    

    pub fn isTime(&mut self, mut currentTime: i64) -> bool {
        return true;
    }
    

    pub fn isTimeSince(&mut self, mut delay: i32, mut currentTime: i64) -> bool {
        return true;
    }
}
impl std::ops::Deref for NoTimeDelayHelper {
    type Target = TimeDelayHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoTimeDelayHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn for NoTimeDelayHelper {
    fn as_TimeDelayHelper(&self) -> &crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper { &self.base }
    fn as_TimeDelayHelper_mut(&mut self) -> &mut crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper { &mut self.base }
    fn isTimeTNT(&mut self) -> bool { Self::isTimeTNT(self) }
    fn isTimeSinceTNT(&mut self, mut delay: i32) -> bool { Self::isTimeSinceTNT(self, delay) }
    fn isTime(&mut self, mut currentTime: i64) -> bool { Self::isTime(self, currentTime) }
    fn isTimeSince(&mut self, mut delay: i32, mut currentTime: i64) -> bool { Self::isTimeSince(self, delay, currentTime) }
}


