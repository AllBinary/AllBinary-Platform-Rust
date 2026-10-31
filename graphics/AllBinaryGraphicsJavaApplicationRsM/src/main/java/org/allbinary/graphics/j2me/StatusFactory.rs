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
// package org.allbinary.graphics.j2me
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StatusFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub SETSTATUS: String,
    pub STATUS_LABEL: String,
}

thread_local! { pub static StatusFactory_instance: Option<Rc<RefCell<StatusFactory>>> = StatusFactory::new(); }

impl StatusFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            SETSTATUS: String::new(),
            STATUS_LABEL: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.SETSTATUS = String::from("setStatus");
        this.STATUS_LABEL = String::from("Status: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<StatusFactory>>> {
        return StatusFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn setStatusNoLog(&mut self, mut status: String) {
        jcall!(unsafe { MyFrame_statusJLabel.clone() }).setText(status.clone());
    }
    

    pub fn setStatus(&mut self, mut status: String) {
        self.setStatusNoLog(status.clone());
        jcall!(self.logUtil).putF(format!("{}{}", self.STATUS_LABEL, status), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SETSTATUS.clone());
    }
}

