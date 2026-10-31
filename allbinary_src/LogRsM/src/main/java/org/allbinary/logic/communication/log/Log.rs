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
use crate::org::allbinary::logic::communication::log::LogFormatUtil::LogFormatUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Log {
    pub logFormatUtil: Option<Rc<RefCell<LogFormatUtil>>>,
    pub specialMessage: String,
    pub anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub functionName: String,
    pub exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}


impl Log {
    

    pub fn new_struct(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            logFormatUtil: None,
            specialMessage: String::new(),
            anyType: None,
            functionName: String::new(),
            exception: None,
        };
        this.logFormatUtil = LogFormatUtil::getInstance();
        this.specialMessage = specialMessage.clone();
        this.anyType = anyType.clone();
        this.functionName = functionName.clone();
        this.exception = exception.clone();
        this
    }

    pub fn new(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(specialMessage, anyType, functionName, exception))))
    }
    

    pub fn getSpecialMessage(&mut self) -> String {
        return self.specialMessage.clone();
    }
    

    pub fn getObject(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.anyType.clone();
    }
    

    pub fn getFunctionName(&mut self) -> String {
        return self.functionName.clone();
    }
    

    pub fn getThrowable(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.exception.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut className: String = jcall!(CommonStrings::getInstance()).EMPTY.clone();
        let mut clazz: &'static str = std::any::type_name_of_val(&self.anyType);
        if true {
            className = clazz.to_string();
        }
        return jcall!(self.logFormatUtil).get(className.clone(), self.functionName.clone(), self.specialMessage.clone(), crate::java_into_object(self.exception.clone()));
    }
}

