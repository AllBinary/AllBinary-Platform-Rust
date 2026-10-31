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
// package bundle.input.automation.module.generic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::automation::module::generic::InputAutomationGenericModuleServiceFactory::InputAutomationGenericModuleServiceFactory;
use crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleBundleActivator::InputAutomationModuleBundleActivator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationGenericModuleBundleActivator {
    pub base: InputAutomationModuleBundleActivator,
}


impl InputAutomationGenericModuleBundleActivator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InputAutomationModuleBundleActivator::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut inputAutomationGenericModuleServiceFactory: Option<Rc<RefCell<InputAutomationGenericModuleServiceFactory>>> = InputAutomationGenericModuleServiceFactory::getInstance();
        { let java_arguments = (jcall!(inputAutomationGenericModuleServiceFactory).getInputAutomationModuleInterfaceArray(),); self.setInputAutomationModuleInterface(java_arguments.0) };
        self.setServiceFactory(inputAutomationGenericModuleServiceFactory.clone());
    }
}
impl std::ops::Deref for InputAutomationGenericModuleBundleActivator {
    type Target = InputAutomationModuleBundleActivator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputAutomationGenericModuleBundleActivator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleBundleActivator::InputAutomationModuleBundleActivatorDyn for InputAutomationGenericModuleBundleActivator {
    fn as_InputAutomationModuleBundleActivator(&self) -> &crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleBundleActivator::InputAutomationModuleBundleActivator { &self.base }
    fn as_InputAutomationModuleBundleActivator_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleBundleActivator::InputAutomationModuleBundleActivator { &mut self.base }
    fn init(&mut self) { Self::init(self) }
}
impl crate::org::osgi::framework::BundleActivator::BundleActivator for InputAutomationGenericModuleBundleActivator {
}


