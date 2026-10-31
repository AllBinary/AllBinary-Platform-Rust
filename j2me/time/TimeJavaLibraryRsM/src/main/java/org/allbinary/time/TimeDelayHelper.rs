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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeDelayHelper {
    pub startTime: i64,
    pub elapsedTimeAtPause: i64,
    pub delay: i32,
}


impl TimeDelayHelper {
    

    pub fn new_struct(mut delay: i32) -> Self {
        let mut this = Self {
            startTime: 0_i64,
            elapsedTimeAtPause: 0_i64,
            delay: 0,
        };
        this.startTime = ((-1) as i64);
        this.elapsedTimeAtPause = i64::MIN.clone();
        this.delay = delay.clone();
        this.setStartTimeTNT();
        this
    }

    pub fn new(mut delay: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(delay))))
    }
    

    pub fn isTimeTNT(&mut self) -> bool {
        let mut currentTime: i64 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64);
        if currentTime - self.startTime > ((self.delay) as i64) {
            self.startTime = currentTime.clone();
            return true;
        }
        return false;
    }
    

    pub fn isTimeSinceTNT(&mut self, mut delay: i32) -> bool {
        let mut currentTime: i64 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64);
        if currentTime - self.startTime > ((delay) as i64) {
            self.startTime = currentTime.clone();
            return true;
        }
        return false;
    }
    

    pub fn isTime(&mut self, mut currentTime: i64) -> bool {
        if currentTime - self.startTime > ((self.delay) as i64) {
            self.startTime = currentTime.clone();
            return true;
        }
        return false;
    }
    

    pub fn isTimeWithoutReset(&mut self, mut currentTime: i64) -> bool {
        if currentTime - self.startTime > ((self.delay) as i64) {
            return true;
        }
        return false;
    }
    

    pub fn isTimeSince(&mut self, mut delay: i32, mut currentTime: i64) -> bool {
        if currentTime - self.startTime > ((self.delay) as i64) {
            self.startTime = currentTime.clone();
            return true;
        }
        return false;
    }
    

    pub fn getElapsedTNT(&mut self) -> i64 {
        return (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64) - self.startTime;
    }
    

    pub fn getElapsed(&mut self, mut currentTime: i64) -> i64 {
        return currentTime - self.startTime;
    }
    

    pub fn isElapsed(&mut self, mut currentTime: i64, mut time: i64) -> bool {
        if self.getElapsed(currentTime.clone()) > time {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isElapsedTNT(&mut self, mut time: i64) -> bool {
        if self.getElapsedTNT() > time {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn setStartTime(&mut self, mut startTime: i64) {
        self.startTime = startTime.clone();
    }
    

    pub fn getStartTime(&mut self) -> i64 {
        return self.startTime.clone();
    }
    

    pub fn setStartTimeTNT(&mut self) {
        self.startTime = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64);
    }
    

    pub fn pause(&mut self) {
        self.elapsedTimeAtPause = self.getElapsedTNT();
        self.startTime = i64::MAX.clone();
    }
    

    pub fn resume(&mut self) -> bool {
        if self.elapsedTimeAtPause != i64::MIN {
            self.startTime = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64) + self.elapsedTimeAtPause;
            self.elapsedTimeAtPause = i64::MIN.clone();
            self.setStartTimeTNT();
            return true;
        }
        return false;
    }
    

    pub fn toStringAt(&mut self, mut currentTime: i64) -> String {
        let mut elapsed: i64 = self.getElapsed(currentTime.clone());
        if elapsed > 0_i64 {
            let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
            let mut elapsedAsString: String = (elapsed.clone()).to_string();
            return jcall!(StringMaker::new()).append(jcall!(commonLabels).START.clone()).appendlong(self.startTime.clone()).append(jcall!(commonLabels).CURRENT.clone()).appendlong(currentTime.clone()).append(jcall!(commonLabels).ELAPSED.clone()).append(elapsedAsString.clone()).toString();
        } else {
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        }
    }
    

    pub fn toString(&mut self) -> String {
        let mut currentTime: i64 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64);
        let mut elapsed: i64 = self.getElapsed(currentTime.clone());
        if elapsed > 0_i64 {
            let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
            let mut elapsedAsString: String = (elapsed.clone()).to_string();
            return jcall!(StringMaker::new()).append(jcall!(commonLabels).START.clone()).appendlong(self.startTime.clone()).append(jcall!(commonLabels).CURRENT.clone()).appendlong(currentTime.clone()).append(jcall!(commonLabels).ELAPSED.clone()).append(elapsedAsString.clone()).toString();
        } else {
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        }
    }
}
pub trait TimeDelayHelperDyn {
    fn as_TimeDelayHelper(&self) -> &TimeDelayHelper;
    fn as_TimeDelayHelper_mut(&mut self) -> &mut TimeDelayHelper;
    fn isTimeTNT(&mut self) -> bool { TimeDelayHelper::isTimeTNT(self.as_TimeDelayHelper_mut()) }
    fn isTimeSinceTNT(&mut self, mut delay: i32) -> bool { TimeDelayHelper::isTimeSinceTNT(self.as_TimeDelayHelper_mut(), delay) }
    fn isTime(&mut self, mut currentTime: i64) -> bool { TimeDelayHelper::isTime(self.as_TimeDelayHelper_mut(), currentTime) }
    fn isTimeWithoutReset(&mut self, mut currentTime: i64) -> bool { TimeDelayHelper::isTimeWithoutReset(self.as_TimeDelayHelper_mut(), currentTime) }
    fn isTimeSince(&mut self, mut delay: i32, mut currentTime: i64) -> bool { TimeDelayHelper::isTimeSince(self.as_TimeDelayHelper_mut(), delay, currentTime) }
    fn getElapsedTNT(&mut self) -> i64 { TimeDelayHelper::getElapsedTNT(self.as_TimeDelayHelper_mut()) }
    fn getElapsed(&mut self, mut currentTime: i64) -> i64 { TimeDelayHelper::getElapsed(self.as_TimeDelayHelper_mut(), currentTime) }
    fn isElapsed(&mut self, mut currentTime: i64, mut time: i64) -> bool { TimeDelayHelper::isElapsed(self.as_TimeDelayHelper_mut(), currentTime, time) }
    fn isElapsedTNT(&mut self, mut time: i64) -> bool { TimeDelayHelper::isElapsedTNT(self.as_TimeDelayHelper_mut(), time) }
    fn setStartTime(&mut self, mut startTime: i64) { TimeDelayHelper::setStartTime(self.as_TimeDelayHelper_mut(), startTime) }
    fn getStartTime(&mut self) -> i64 { TimeDelayHelper::getStartTime(self.as_TimeDelayHelper_mut()) }
    fn setStartTimeTNT(&mut self) { TimeDelayHelper::setStartTimeTNT(self.as_TimeDelayHelper_mut()) }
    fn pause(&mut self) { TimeDelayHelper::pause(self.as_TimeDelayHelper_mut()) }
    fn resume(&mut self) -> bool { TimeDelayHelper::resume(self.as_TimeDelayHelper_mut()) }
    fn toStringAt(&mut self, mut currentTime: i64) -> String { TimeDelayHelper::toStringAt(self.as_TimeDelayHelper_mut(), currentTime) }
    fn toString(&mut self) -> String { TimeDelayHelper::toString(self.as_TimeDelayHelper_mut()) }
}
impl TimeDelayHelperDyn for TimeDelayHelper {
    fn as_TimeDelayHelper(&self) -> &TimeDelayHelper { self }
    fn as_TimeDelayHelper_mut(&mut self) -> &mut TimeDelayHelper { self }
}


