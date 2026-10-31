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
use super::Log::Log;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogUtil {
}

thread_local! { pub static LogUtil_instance: Option<Rc<RefCell<LogUtil>>> = LogUtil::new(); }

impl LogUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogUtil>>> {
        return LogUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
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
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn putF(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn putF(&mut self, mut specialMessage: String, mut className: String, mut functionName: String) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn put(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

