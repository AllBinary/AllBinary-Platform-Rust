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
// package org.allbinary.input.automation.module.generic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;
use crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleServiceFactory::InputAutomationModuleServiceFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::GenericModuleFactory::GenericModuleFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationGenericModuleServiceFactory {
    pub base: InputAutomationModuleServiceFactory,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

pub static mut InputAutomationGenericModuleServiceFactory_inputAutomationGenericModuleServiceFactory: std::cell::LazyCell<Option<Rc<RefCell<InputAutomationGenericModuleServiceFactory>>>> = std::cell::LazyCell::new(|| InputAutomationGenericModuleServiceFactory::new());

impl InputAutomationGenericModuleServiceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InputAutomationModuleServiceFactory::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>> = vec![None; (1) as usize];
            inputAutomationModuleInterfaceArray[(0) as usize] = GenericModuleFactory::new();
            this.setInputAutomationModuleInterfaceArray(inputAutomationModuleInterfaceArray.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InputAutomationGenericModuleServiceFactory>>> {
        return unsafe { InputAutomationGenericModuleServiceFactory_inputAutomationGenericModuleServiceFactory.clone() }.clone();
    }
}
impl std::ops::Deref for InputAutomationGenericModuleServiceFactory {
    type Target = InputAutomationModuleServiceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputAutomationGenericModuleServiceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleServiceFactory::InputAutomationModuleServiceFactoryDyn for InputAutomationGenericModuleServiceFactory {
    fn as_InputAutomationModuleServiceFactory(&self) -> &crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleServiceFactory::InputAutomationModuleServiceFactory { &self.base }
    fn as_InputAutomationModuleServiceFactory_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleServiceFactory::InputAutomationModuleServiceFactory { &mut self.base }
}
impl crate::org::osgi::framework::ServiceFactory::ServiceFactory for InputAutomationGenericModuleServiceFactory {
}


