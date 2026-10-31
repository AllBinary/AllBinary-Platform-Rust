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
// package org.allbinary.logic.communication.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::logging::Level::Level;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
// use crate::java::util::logging::Logger::Logger;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::LogFormatUtil::LogFormatUtil;
use super::Log::Log;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ActualPlatform
pub struct LogUtil {
    pub logger: Option<Rc<RefCell<Logger>>>,
    pub logFormatUtil: Option<Rc<RefCell<LogFormatUtil>>>,
}

thread_local! { pub static LogUtil_instance: Option<Rc<RefCell<LogUtil>>> = LogUtil::new(); }

impl LogUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogUtil>>> {
        return LogUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logger: None,
            logFormatUtil: None,
        };
        this.logger = Logger::getLogger(jcall!(std::any::type_name::<LogUtil>()).getName());
        this.logFormatUtil = LogFormatUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        PreLogUtil::PreLogUtil::put(String::from("Loggin Initialized"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("init()"));
    }
    

    pub fn putL(&mut self, mut log: Option<Rc<RefCell<Log>>>) {
        let mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(log).getThrowable();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if exception.is_some() {
                let mut specialMessage: String = jcall!(log).getSpecialMessage();
                let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(log).getObject();
                let mut functionName: String = jcall!(log).getFunctionName();
                let mut className: String = jcall!(CommonStrings::getInstance()).EMPTY.clone();
                let mut clazz: &'static str = std::any::type_name_of_val(&anyType);
                if true {
                    className = clazz.to_string();
                }
                let mut message: String = jcall!(self.logFormatUtil).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
                jcall!(self.logger).log(unsafe { crate::java::util::logging::Level::Level_INFO.clone() }.clone(), message.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
    }
    

    pub fn put(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut className: String = jcall!(CommonStrings::getInstance()).EMPTY.clone();
            let mut clazz: &'static str = std::any::type_name_of_val(&anyType);
            if true {
                className = clazz.to_string();
            }
            let mut message: String = jcall!(self.logFormatUtil).getS(className.clone(), functionName.clone(), specialMessage.clone());
            if exception.is_some() {
                jcall!(self.logger).log(unsafe { crate::java::util::logging::Level::Level_SEVERE.clone() }.clone(), message.clone(), exception.clone());
            } else {
                jcall!(self.logger).log(unsafe { crate::java::util::logging::Level::Level_INFO.clone() }.clone(), message.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
    }
}

