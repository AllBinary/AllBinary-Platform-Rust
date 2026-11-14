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
use crate::bundle::input::automation::module::InputAutomationModuleService::InputAutomationModuleService;
use crate::org::allbinary::input::automation::InputAutomationJFrame::InputAutomationJFrame;
use crate::org::allbinary::input::automation::configuration::InputAutomationConfigurationUtil::InputAutomationConfigurationUtil;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::osgi::OSGIServiceInterface::OSGIServiceInterface;
use crate::org::allbinary::osgi::OSGIServiceVisitorInterface::OSGIServiceVisitorInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleOSGIServiceVisitor {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

// impl OSGIServiceVisitorInterface for InputAutomationModuleOSGIServiceVisitor with the methods of the trait

impl InputAutomationModuleOSGIServiceVisitor {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return crate::java_into_object(self.visit_OSGIServiceInterface({ let java_object = (anyType).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn OSGIServiceInterface>>>>().expect("ClassCastException").clone()) }));
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn visit_OSGIServiceInterface(&mut self, mut osgiServiceInterface: Option<Rc<RefCell<dyn OSGIServiceInterface>>>) -> Option<bool> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inputAutomationModuleService: Option<Rc<RefCell<InputAutomationModuleService>>> = (osgiServiceInterface).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<InputAutomationModuleService>() }, "ClassCastException: not a InputAutomationModuleService"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<InputAutomationModuleService>) } });
            let mut inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn>>>> = jcall!(inputAutomationModuleService).getInputAutomationModuleInterfaceArray();
            for index in (0..(inputAutomationModuleInterfaceArray.len() as i32)) {
                jcall!(InputAutomationJFrame::getInstance()).onAdd(InputAutomationConfigurationUtil::getChangeEvent(inputAutomationModuleInterfaceArray[(index) as usize].clone()));
            }
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("visit"), crate::java_into_object(e.clone()));
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

