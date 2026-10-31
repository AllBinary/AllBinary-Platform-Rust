// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.logic.communication.log.config.type
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::LogConfigType::LogConfigType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogConfigTypeFactory {
    pub NO_DESCRIPTION: String,
    pub OS: Option<Rc<RefCell<LogConfigType>>>,
    pub FACTORYERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub FILE: Option<Rc<RefCell<LogConfigType>>>,
    pub FILEERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub IDLOGGING: Option<Rc<RefCell<LogConfigType>>>,
    pub REPLACE: Option<Rc<RefCell<LogConfigType>>>,
    pub REPLACE_INFO: Option<Rc<RefCell<LogConfigType>>>,
    pub REPLACEERROR: Option<Rc<RefCell<LogConfigType>>>,
}

thread_local! { pub static LogConfigTypeFactory_instance: Option<Rc<RefCell<LogConfigTypeFactory>>> = LogConfigTypeFactory::new(); }

impl LogConfigTypeFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogConfigTypeFactory>>> {
        return LogConfigTypeFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NO_DESCRIPTION: String::new(),
            OS: None,
            FACTORYERROR: None,
            FILE: None,
            FILEERROR: None,
            IDLOGGING: None,
            REPLACE: None,
            REPLACE_INFO: None,
            REPLACEERROR: None,
        };
        this.NO_DESCRIPTION = String::from("No Description");
        this.OS = LogConfigType::new(String::from("Operating System"), this.NO_DESCRIPTION.clone());
        this.FACTORYERROR = LogConfigType::new(String::from("Factory Error"), this.NO_DESCRIPTION.clone());
        this.FILE = LogConfigType::new(String::from("File"), this.NO_DESCRIPTION.clone());
        this.FILEERROR = LogConfigType::new(String::from("File Error"), this.NO_DESCRIPTION.clone());
        this.IDLOGGING = LogConfigType::new(String::from("ID Logging"), this.NO_DESCRIPTION.clone());
        this.REPLACE = LogConfigType::new(String::from("Replace"), this.NO_DESCRIPTION.clone());
        this.REPLACE_INFO = LogConfigType::new(String::from("Replace Info"), this.NO_DESCRIPTION.clone());
        this.REPLACEERROR = LogConfigType::new(String::from("Replace Error"), this.NO_DESCRIPTION.clone());
        jcall!(unsafe { LogConfigTypes_LOGGING.clone() }).add(crate::java_into_object(this.OS.clone()));
        jcall!(unsafe { LogConfigTypes_LOGGING.clone() }).add(crate::java_into_object(this.FACTORYERROR.clone()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

