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
// package bundle.input.automation.module
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;
use super::InputAutomationModuleServiceInterface::InputAutomationModuleServiceInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleService {
    pub inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>,
}

impl InputAutomationModuleServiceInterface for InputAutomationModuleService {
    fn getInputAutomationModuleInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>> { Self::getInputAutomationModuleInterfaceArray(self) }
}
impl crate::org::allbinary::osgi::OSGIServiceInterface::OSGIServiceInterface for InputAutomationModuleService {
}

impl InputAutomationModuleService {
    

    pub fn new_struct(mut inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>) -> Self {
        let mut this = Self {
            inputAutomationModuleInterfaceArray: Vec::new(),
        };
        this.setInputAutomationModuleInterfaceArray(inputAutomationModuleInterfaceArray.clone());
        this
    }

    pub fn new(mut inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(inputAutomationModuleInterfaceArray))))
    }
    

    pub fn getInputAutomationModuleInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>> {
        return self.inputAutomationModuleInterfaceArray.clone();
    }
    

    pub fn setInputAutomationModuleInterfaceArray(&mut self, mut inputAutomationModuleInterfaceArray: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>) {
        self.inputAutomationModuleInterfaceArray = inputAutomationModuleInterfaceArray.clone();
    }
}

