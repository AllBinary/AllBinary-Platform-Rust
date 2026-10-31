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
// package org.allbinary.logic.communication.log.config
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::LogConfig::LogConfig;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LoggingInitInfo {
}

pub static mut LoggingInitInfo_logConfigInfoList: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| None);
pub static mut LoggingInitInfo_hasRead: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| false);

impl LoggingInitInfo {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn write() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            unsafe { *LoggingInitInfo_hasRead = false };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // synchronized
    pub fn read() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // synchronized
    pub fn setHasRead(mut value: bool) {
        unsafe { *LoggingInitInfo_hasRead = value.clone() };
    }
    

    // throws Exception
    // synchronized
    pub fn updateIfNeeded() {
        if !unsafe { LoggingInitInfo_hasRead.clone() } {
            LoggingInitInfo::read();
            unsafe { *LoggingInitInfo_hasRead = true };
            if unsafe { LoggingInitInfo_logConfigInfoList.clone() }.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Read Failed"))));
            }
        }
    }
    

    pub fn isValid(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
    

    // synchronized
    pub fn set(mut logConfigInfoList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        unsafe { *LoggingInitInfo_logConfigInfoList = logConfigInfoList.clone() };
    }
    

    // throws Exception
    pub fn get() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        LoggingInitInfo::updateIfNeeded();
        return unsafe { LoggingInitInfo_logConfigInfoList.clone() }.clone();
    }
    

    // throws Exception
    pub fn getTypeList() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        LoggingInitInfo::updateIfNeeded();
        let mut allLogTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).size();
        for index in (0..size) {
            let mut logConfigInfo: Option<Rc<RefCell<LogConfig>>> = { let java_object = (jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LogConfig>>>>().expect("ClassCastException").clone()) };
            let mut logTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(logConfigInfo).getTypeVector();
            if jcall!(logConfigInfo).isEnabled() {
                jcall!(allLogTypeVector).addAllList(logTypeVector.clone());
            }
        }
        return allLogTypeVector.clone();
    }
    

    // throws Exception
    pub fn getNumberOfLogConfigs(&mut self) -> i32 {
        return jcall!(unsafe { LoggingInitInfo_logConfigInfoList.clone() }).size();
    }
    

    pub fn toString(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return None;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return String::from("Error");
            }
        }
    }
}

