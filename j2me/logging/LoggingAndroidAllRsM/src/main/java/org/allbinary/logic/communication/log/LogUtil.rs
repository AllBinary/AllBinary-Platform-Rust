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
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
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
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub logFormatUtil: Option<Rc<RefCell<LogFormatUtil>>>,
    pub LABEL: String,
}

thread_local! { pub static LogUtil_instance: Option<Rc<RefCell<LogUtil>>> = LogUtil::new(); }

impl LogUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogUtil>>> {
        return LogUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonSeps: None,
            logFormatUtil: None,
            LABEL: String::new(),
        };
        this.commonSeps = CommonSeps::getInstance();
        this.logFormatUtil = LogFormatUtil::getInstance();
        this.LABEL = String::from("org.allbinary");
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
        { let java_arguments = (specialMessage.clone(), anyType.clone(), functionName.clone(), jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(),); self.put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) };
    }
    

    pub fn putFS(&mut self, mut specialMessage: String, mut string: String, mut functionName: String) {
        { let java_arguments = (specialMessage.clone(), string.clone(), functionName.clone(), jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(),); self.putS(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) };
    }
    

    pub fn put(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut className: String = self.LABEL.clone();
        className = jcall!(StringMaker::new()).append(std::any::type_name_of_val(&anyType).to_string()).append(jcall!(self.commonSeps).COLON.clone()).append(format!("{:x}", jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(anyType.clone())))).toString().clone();
        let mut message: String = jcall!(self.logFormatUtil).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
        jcall!(jcall!(unsafe { android_util.clone() }).Log).i(self.LABEL.clone(), message.clone());
    }
    

    pub fn putS(&mut self, mut specialMessage: String, mut string: String, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut className: String = self.LABEL.clone();
        className = jcall!(StringMaker::new()).append(string.clone()).append(jcall!(self.commonSeps).COLON.clone()).append(format!("{:x}", jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(string.clone())))).toString().clone();
        let mut message: String = jcall!(self.logFormatUtil).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
        jcall!(jcall!(unsafe { android_util.clone() }).Log).i(self.LABEL.clone(), message.clone());
    }
}

