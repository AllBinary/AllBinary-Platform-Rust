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
use crate::org::allbinary::logic::java::exception::ExceptionUtil::ExceptionUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::time::TimeStampUtil::TimeStampUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ActualPlatform
pub struct LogFormatUtil {
    pub timeStampUtil: Option<Rc<RefCell<TimeStampUtil>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub NONE: String,
    pub LOG_ERROR: String,
    pub EMPTY: String,
    pub STACK_TRACE: String,
    pub TIME: String,
    pub CLASS_NAME: String,
    pub FUNCTION_CALL: String,
    pub SPECIAL_MESSAGE: String,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub exceptionUtil: Option<Rc<RefCell<ExceptionUtil>>>,
    pub nullUtil: Option<Rc<RefCell<NullUtil>>>,
}

pub static LogFormatUtil_instance: crate::JavaStatic<Option<Rc<RefCell<LogFormatUtil>>>> = crate::JavaStatic::new(|| LogFormatUtil::new());

impl LogFormatUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LogFormatUtil>>> {
        return LogFormatUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            timeStampUtil: None,
            commonSeps: None,
            NONE: String::new(),
            LOG_ERROR: String::new(),
            EMPTY: String::new(),
            STACK_TRACE: String::new(),
            TIME: String::new(),
            CLASS_NAME: String::new(),
            FUNCTION_CALL: String::new(),
            SPECIAL_MESSAGE: String::new(),
            stringUtil: None,
            exceptionUtil: None,
            nullUtil: None,
        };
        this.timeStampUtil = TimeStampUtil::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.NONE = String::from("None");
        this.LOG_ERROR = String::from("\nLog-Error: ");
        this.EMPTY = String::from("Empty");
        this.STACK_TRACE = String::from("\nStack Trace: ");
        this.TIME = String::from("Time: ");
        this.CLASS_NAME = { let java_field = jcall!(this.commonSeps).SPACE.clone(); java_field }.clone();
        this.FUNCTION_CALL = { let java_field = jcall!(CommonLabels::getInstance()).COLON_SEP.clone(); java_field }.clone();
        this.SPECIAL_MESSAGE = String::from("> ");
        this.stringUtil = StringUtil::getInstance();
        this.exceptionUtil = ExceptionUtil::getInstance();
        this.nullUtil = NullUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn get(&mut self, mut className: String, mut functionName: String, mut specialMessage: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = self.getF(className.clone(), functionName.clone());
        jcall!(stringBuffer).append(self.getO(exception.clone()));
        jcall!(stringBuffer).append(self.SPECIAL_MESSAGE.clone());
        jcall!(stringBuffer).append(specialMessage.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).NEW_LINE.clone(); java_field }.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getS(&mut self, mut className: String, mut functionName: String, mut specialMessage: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = self.getF(className.clone(), functionName.clone());
        jcall!(stringBuffer).append(self.SPECIAL_MESSAGE.clone());
        jcall!(stringBuffer).append(specialMessage.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).NEW_LINE.clone(); java_field }.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getF(&mut self, mut className: String, mut functionName: String) -> Option<Rc<RefCell<StringMaker>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.TIME.clone());
        jcall!(stringBuffer).append(jcall!(self.timeStampUtil).getAsString());
        jcall!(stringBuffer).append(self.CLASS_NAME.clone());
        jcall!(stringBuffer).append(className.clone());
        jcall!(stringBuffer).append(self.FUNCTION_CALL.clone());
        jcall!(stringBuffer).append(functionName.clone());
        return stringBuffer.clone();
    }
    

    pub fn getO(&mut self, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        if !match (&exception, &{ let java_field = jcall!(self.nullUtil).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.LOG_ERROR.clone());
            let mut exceptionAsString: String = { let java_object = (exception).clone(); let value = jcall!(java_object); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) };
            jcall!(stringBuffer).append(exceptionAsString.clone());
            jcall!(stringBuffer).append(self.STACK_TRACE.clone());
            if !match (&exception, &{ let java_field = jcall!(self.nullUtil).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(stringBuffer).append(jcall!(self.exceptionUtil).getStackTrace({ let reference = exception.as_ref().expect("null reference").borrow(); reference.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", reference.as_ref().type_id())) }));
            } else {
                jcall!(stringBuffer).append({ let java_field = jcall!(self.stringUtil).NULL_STRING.clone(); java_field }.clone());
            }
            return jcall!(stringBuffer).toString();
        } else {
            return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        }
    }
}

