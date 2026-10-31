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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::playn::core::PlayN::PlayN;
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
    pub logFormatUtil: Option<Rc<RefCell<LogFormatUtil>>>,
    pub LOG_SUCCESS: String,
}

thread_local! { pub static LogUtil_instance: Option<Rc<RefCell<LogUtil>>> = LogUtil::new(); }

impl LogUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogUtil>>> {
        return LogUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logFormatUtil: None,
            LOG_SUCCESS: String::new(),
        };
        this.logFormatUtil = LogFormatUtil::getInstance();
        this.LOG_SUCCESS = String::from("org.allbinary: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn putL(&mut self, mut log: Option<Rc<RefCell<Log>>>) {
        if log.is_none() {
            return;
        }
        let mut specialMessage: String = jcall!(log).getSpecialMessage();
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(log).getObject();
        let mut functionName: String = jcall!(log).getFunctionName();
        let mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(log).getThrowable();
        self.put(specialMessage.clone(), anyType.clone(), functionName.clone(), exception.clone());
    }
    

    pub fn putF(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String) {
        let mut className: String = PreLogUtil::PreLogUtil::getClassName(anyType.clone());
        if false {
            className = jcall!(CommonStrings::getInstance()).EMPTY.clone();
        }
        let mut message: String = jcall!(self.logFormatUtil).getS(className.clone(), functionName.clone(), specialMessage.clone());
        jcall!(PlayN::log()).debug(format!("{}{}", self.LOG_SUCCESS, message));
    }
    

    pub fn putFS(&mut self, mut specialMessage: String, mut className: String, mut functionName: String) {
        let mut message: String = jcall!(self.logFormatUtil).getS(className.clone(), functionName.clone(), specialMessage.clone());
        jcall!(PlayN::log()).debug(format!("{}{}", self.LOG_SUCCESS, message));
    }
    

    pub fn put(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut className: String = PreLogUtil::PreLogUtil::getClassName(anyType.clone());
        if false {
            className = jcall!(CommonStrings::getInstance()).EMPTY.clone();
        }
        let mut message: String = jcall!(self.logFormatUtil).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
        if exception.is_some() {
            jcall!(PlayN::log()).error(format!("{}{}", self.LOG_SUCCESS, message), { let reference = exception.as_ref().expect("null reference").borrow(); reference.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", reference.as_ref().type_id())) });
        } else {
            jcall!(PlayN::log()).debug(format!("{}{}", self.LOG_SUCCESS, message));
        }
    }
}

