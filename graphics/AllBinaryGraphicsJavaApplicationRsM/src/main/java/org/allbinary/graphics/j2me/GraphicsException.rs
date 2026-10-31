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
use crate::org::allbinary::logic::communication::log::GuiLog::GuiLog;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::lang::lang;
use super::java::java;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GraphicsException {
    pub base: Exception,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub guiLog: Option<Rc<RefCell<GuiLog>>>,
}


impl GraphicsException {
    

    pub fn new_struct3(mut msg: String, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut method: String) -> Self {
        let mut this = Self {
            base: Exception::new_struct1(msg.clone()),
            logUtil: None,
            guiLog: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.guiLog = GuiLog::getInstance();
        // super(...) is in the struct literal above
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(this.guiLog).showDialog(msg.clone());
            jcall!(this.logUtil).put(msg.clone(), crate::java_into_object(obj.clone()), method.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
        this
    }

    pub fn new3(mut msg: String, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut method: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(msg, obj, method))))
    }
    

    pub fn new_struct3(mut msg: String, mut className: String, mut method: String) -> Self {
        let mut this = Self {
            base: Exception::new_struct1(msg.clone()),
            logUtil: None,
            guiLog: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.guiLog = GuiLog::getInstance();
        // super(...) is in the struct literal above
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(this.guiLog).showDialog(msg.clone());
            jcall!(this.logUtil).put(msg.clone(), crate::java_into_object(className.clone()), method.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
        this
    }

    pub fn new3(mut msg: String, mut className: String, mut method: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(msg, className, method))))
    }
}
impl std::ops::Deref for GraphicsException {
    type Target = Exception;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GraphicsException {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



