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
// package org.allbinary.input.automation.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;
use crate::org::allbinary::input::automation::module::configuration::InputAutomationModuleConfiguration::InputAutomationModuleConfiguration;
use super::InputAutomationConfigurationModuleChangeEvent::InputAutomationConfigurationModuleChangeEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationConfigurationUtil {
}


impl InputAutomationConfigurationUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getChangeEvent(mut inputAutomationModuleInterface: Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>) -> Option<Rc<RefCell<InputAutomationConfigurationModuleChangeEvent>>> {
        let mut inputAutomationModuleConfiguration: Option<Rc<RefCell<InputAutomationModuleConfiguration>>> = InputAutomationModuleConfiguration::new1(inputAutomationModuleInterface.clone());
        let mut inputAutomationConfigurationModuleChangeEvent: Option<Rc<RefCell<InputAutomationConfigurationModuleChangeEvent>>> = InputAutomationConfigurationModuleChangeEvent::new(inputAutomationModuleConfiguration.clone());
        return inputAutomationConfigurationModuleChangeEvent.clone();
    }
}

