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
// package org.allbinary.input.automation.module.osgi
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::osgi::framework::Bundle::Bundle;
use crate::org::osgi::framework::ServiceFactory::ServiceFactory;
use crate::org::osgi::framework::ServiceRegistration::ServiceRegistration;
use crate::bundle::input::automation::module::InputAutomationModuleService::InputAutomationModuleService;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleServiceFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>,
}

impl ServiceFactory for InputAutomationModuleServiceFactory {
}

impl InputAutomationModuleServiceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            inputAutomationModuleInterfaceArray: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getService(&mut self, mut bundle: Option<Rc<RefCell<dyn Bundle>>>, mut registration: Option<Rc<RefCell<dyn ServiceRegistration>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getService"));
        return InputAutomationModuleService::new(self.getInputAutomationModuleInterfaceArray());
    }
    

    pub fn ungetService(&mut self, mut bundle: Option<Rc<RefCell<dyn Bundle>>>, mut registration: Option<Rc<RefCell<dyn ServiceRegistration>>>, mut service: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn getInputAutomationModuleInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>> {
        return self.inputAutomationModuleInterfaceArray.clone();
    }
    

    pub fn setInputAutomationModuleInterfaceArray(&mut self, mut inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>) {
        self.inputAutomationModuleInterfaceArray = inputAutomationModuleInterfaceArray.clone();
    }
}
pub trait InputAutomationModuleServiceFactoryDyn: crate::org::osgi::framework::ServiceFactory::ServiceFactory {
    fn as_InputAutomationModuleServiceFactory(&self) -> &InputAutomationModuleServiceFactory;
    fn as_InputAutomationModuleServiceFactory_mut(&mut self) -> &mut InputAutomationModuleServiceFactory;
    fn getService(&mut self, mut bundle: Option<Rc<RefCell<dyn Bundle>>>, mut registration: Option<Rc<RefCell<dyn ServiceRegistration>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { InputAutomationModuleServiceFactory::getService(self.as_InputAutomationModuleServiceFactory_mut(), bundle, registration) }
    fn ungetService(&mut self, mut bundle: Option<Rc<RefCell<dyn Bundle>>>, mut registration: Option<Rc<RefCell<dyn ServiceRegistration>>>, mut service: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { InputAutomationModuleServiceFactory::ungetService(self.as_InputAutomationModuleServiceFactory_mut(), bundle, registration, service) }
    fn getInputAutomationModuleInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>> { InputAutomationModuleServiceFactory::getInputAutomationModuleInterfaceArray(self.as_InputAutomationModuleServiceFactory_mut()) }
    fn setInputAutomationModuleInterfaceArray(&mut self, mut inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>) { InputAutomationModuleServiceFactory::setInputAutomationModuleInterfaceArray(self.as_InputAutomationModuleServiceFactory_mut(), inputAutomationModuleInterfaceArray) }
}
impl InputAutomationModuleServiceFactoryDyn for InputAutomationModuleServiceFactory {
    fn as_InputAutomationModuleServiceFactory(&self) -> &InputAutomationModuleServiceFactory { self }
    fn as_InputAutomationModuleServiceFactory_mut(&mut self) -> &mut InputAutomationModuleServiceFactory { self }
}


