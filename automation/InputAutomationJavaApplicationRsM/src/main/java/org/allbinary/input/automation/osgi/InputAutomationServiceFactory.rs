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
// package org.allbinary.input.automation.osgi
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::osgi::framework::Bundle::Bundle;
use crate::org::osgi::framework::ServiceFactory::ServiceFactory;
use crate::org::osgi::framework::ServiceRegistration::ServiceRegistration;
use crate::org::allbinary::input::automation::InputAutomationJFrame::InputAutomationJFrame;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationServiceFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static InputAutomationServiceFactory_inputAutomationServiceFactory: Option<Rc<RefCell<InputAutomationServiceFactory>>> = InputAutomationServiceFactory::new(); }
impl ServiceFactory for InputAutomationServiceFactory {
}

impl InputAutomationServiceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InputAutomationServiceFactory>>> {
        return InputAutomationServiceFactory_inputAutomationServiceFactory.with(Clone::clone).clone();
    }
    

    pub fn getService(&mut self, mut bundle: Option<Rc<RefCell<dyn Bundle>>>, mut registration: Option<Rc<RefCell<dyn ServiceRegistration>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getService"));
        return InputAutomationJFrame::getInstance();
    }
    

    pub fn ungetService(&mut self, mut bundle: Option<Rc<RefCell<dyn Bundle>>>, mut registration: Option<Rc<RefCell<dyn ServiceRegistration>>>, mut service: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}

