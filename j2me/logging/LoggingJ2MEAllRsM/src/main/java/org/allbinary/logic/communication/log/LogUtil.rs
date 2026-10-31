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
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::LogFormatUtil::LogFormatUtil;
use super::Log::Log;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
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
        let mut specialMessage: String = jcall!(log).getSpecialMessage();
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(log).getObject();
        let mut functionName: String = jcall!(log).getFunctionName();
        let mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(log).getThrowable();
        self.put(specialMessage.clone(), anyType.clone(), functionName.clone(), exception.clone());
    }
    

    pub fn putF(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String) {
        let mut className: String = jcall!(CommonStrings::getInstance()).EMPTY.clone();
        if true {
            className = jcall!(StringMaker::new()).append(std::any::type_name_of_val(&anyType).to_string()).append(jcall!(CommonSeps::getInstance()).COLON.clone()).append(format!("{:x}", jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(anyType.clone())))).toString().clone();
        }
        let mut message: String = jcall!(self.logFormatUtil).getS(className.clone(), functionName.clone(), specialMessage.clone());
        print!("{}", self.LOG_SUCCESS.clone());
        println!("{}", message.clone());
    }
    

    pub fn putFS(&mut self, mut specialMessage: String, mut className: String, mut functionName: String) {
        let mut message: String = jcall!(self.logFormatUtil).getS(className.clone(), functionName.clone(), specialMessage.clone());
        print!("{}", self.LOG_SUCCESS.clone());
        println!("{}", message.clone());
    }
    

    pub fn put(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut className: String = jcall!(CommonStrings::getInstance()).EMPTY.clone();
        if true {
            let mut hashCodeValue: i32 = jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(anyType.clone()));
            let mut hexString: String = format!("{:x}", hashCodeValue.clone());
            className = jcall!(StringMaker::new()).append(std::any::type_name_of_val(&anyType).to_string()).append(jcall!(CommonSeps::getInstance()).COLON.clone()).append(hexString.clone()).toString().clone();
        }
        let mut message: String = jcall!(self.logFormatUtil).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
        print!("{}", self.LOG_SUCCESS.clone());
        println!("{}", message.clone());
    }
}

