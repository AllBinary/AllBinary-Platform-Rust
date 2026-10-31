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
// package org.allbinary.logic.control.workflow
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::lang::reflect::Method::Method;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::WorkFlowInterface::WorkFlowInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WorkFlowWrapper {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}

impl WorkFlowInterface for WorkFlowWrapper {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn process(&mut self) -> Option<i32> { Self::process(self) }
}

impl WorkFlowWrapper {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            anyType: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.anyType = anyType.clone();
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType))))
    }
    

    pub fn getWorkFlowObject(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.anyType.clone();
    }
    

    // throws Exception
    pub fn getName(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("getName"), None);
            let mut result: String = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Reflection Exception");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getName()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (error.clone())));
            }
        }
    }
    

    // throws Exception
    pub fn getStoreName(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("getStoreName"), None);
            let mut result: String = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Reflection Exception");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getStoreName()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (error.clone())));
            }
        }
    }
    

    // throws Exception
    pub fn process(&mut self) -> Option<i32> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(jcall!(self.commonStrings).PROCESS.clone(), None);
            let mut result: Option<i32> = (jcall!(method).invoke(self.anyType.clone(), None)).clone() /* as Integer */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Reflection Exception");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (error.clone())));
            }
        }
    }
}
pub trait WorkFlowWrapperDyn {
    fn as_WorkFlowWrapper(&self) -> &WorkFlowWrapper;
    fn as_WorkFlowWrapper_mut(&mut self) -> &mut WorkFlowWrapper;
    fn getWorkFlowObject(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { WorkFlowWrapper::getWorkFlowObject(self.as_WorkFlowWrapper_mut()) }
    fn getName(&mut self) -> String { WorkFlowWrapper::getName(self.as_WorkFlowWrapper_mut()) }
    fn getStoreName(&mut self) -> String { WorkFlowWrapper::getStoreName(self.as_WorkFlowWrapper_mut()) }
    fn process(&mut self) -> Option<i32> { WorkFlowWrapper::process(self.as_WorkFlowWrapper_mut()) }
}
impl WorkFlowWrapperDyn for WorkFlowWrapper {
    fn as_WorkFlowWrapper(&self) -> &WorkFlowWrapper { self }
    fn as_WorkFlowWrapper_mut(&mut self) -> &mut WorkFlowWrapper { self }
}


