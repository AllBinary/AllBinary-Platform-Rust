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
use super::WorkFlowWrapper::WorkFlowWrapper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreWorkFlowWrapper {
    pub base: WorkFlowWrapper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl StoreWorkFlowWrapper {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            base: WorkFlowWrapper::new_struct(anyType.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType))))
    }
    

    // throws Exception
    pub fn getStoreName(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helperClass: &'static str = std::any::type_name_of_val(&self.getWorkFlowObject());
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("getStoreName"), None);
            let mut result: String = (jcall!(method).invoke(self.getWorkFlowObject(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Reflection Exception");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getStoreName()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (error.clone())));
            }
        }
    }
}
impl std::ops::Deref for StoreWorkFlowWrapper {
    type Target = WorkFlowWrapper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreWorkFlowWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::workflow::WorkFlowWrapper::WorkFlowWrapperDyn for StoreWorkFlowWrapper {
    fn as_WorkFlowWrapper(&self) -> &crate::org::allbinary::logic::control::workflow::WorkFlowWrapper::WorkFlowWrapper { &self.base }
    fn as_WorkFlowWrapper_mut(&mut self) -> &mut crate::org::allbinary::logic::control::workflow::WorkFlowWrapper::WorkFlowWrapper { &mut self.base }
    fn getStoreName(&mut self) -> String { Self::getStoreName(self) }
}


