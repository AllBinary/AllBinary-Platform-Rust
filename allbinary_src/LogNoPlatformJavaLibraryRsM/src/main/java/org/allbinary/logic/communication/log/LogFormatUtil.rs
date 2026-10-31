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

//NoPlatform
pub struct LogFormatUtil {
}

thread_local! { pub static LogFormatUtil_instance: Option<Rc<RefCell<LogFormatUtil>>> = LogFormatUtil::new(); }

impl LogFormatUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogFormatUtil>>> {
        return LogFormatUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn get(&mut self, mut className: String, mut functionName: String, mut specialMessage: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getS(&mut self, mut className: String, mut functionName: String, mut specialMessage: String) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getO(&mut self, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

