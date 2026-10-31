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
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::bundle::input::automation::module::InputAutomationModuleServiceInterface::InputAutomationModuleServiceInterface;
use crate::org::allbinary::osgi::service::InputAutomationServiceConsumer::InputAutomationServiceConsumer;
use super::InputAutomationModuleOSGIServiceVisitor::InputAutomationModuleOSGIServiceVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleServiceConsumer {
    pub base: InputAutomationServiceConsumer,
}


impl InputAutomationModuleServiceConsumer {
    

    pub fn new_struct(mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>) -> Self {
        let mut this = Self {
            base: InputAutomationServiceConsumer::new_struct(jcall!(std::any::type_name::<InputAutomationModuleServiceInterface>()).getName(), bundleContext.clone(), InputAutomationModuleOSGIServiceVisitor::new()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bundleContext))))
    }
}
impl std::ops::Deref for InputAutomationModuleServiceConsumer {
    type Target = InputAutomationServiceConsumer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputAutomationModuleServiceConsumer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::osgi::service::InputAutomationServiceConsumer::InputAutomationServiceConsumerDyn for InputAutomationModuleServiceConsumer {
    fn as_InputAutomationServiceConsumer(&self) -> &crate::org::allbinary::osgi::service::InputAutomationServiceConsumer::InputAutomationServiceConsumer { &self.base }
    fn as_InputAutomationServiceConsumer_mut(&mut self) -> &mut crate::org::allbinary::osgi::service::InputAutomationServiceConsumer::InputAutomationServiceConsumer { &mut self.base }
}


