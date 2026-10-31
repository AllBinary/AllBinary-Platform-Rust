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
// package org.allbinary.input.automation.module
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::help::HelpSet::HelpSet;
// use crate::javax::swing::*;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleFactoryInterface {
}


impl InputAutomationModuleFactoryInterface {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        return None;
    }
    

    pub fn getConfigurationJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return None;
    }
    

    pub fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> {
        return None;
    }
    

    // throws Exception
    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn RunnableInterface>>> {
        return None;
    }
}
pub trait InputAutomationModuleFactoryInterfaceDyn {
    fn as_InputAutomationModuleFactoryInterface(&self) -> &InputAutomationModuleFactoryInterface;
    fn as_InputAutomationModuleFactoryInterface_mut(&mut self) -> &mut InputAutomationModuleFactoryInterface;
    fn getName(&mut self) -> String { InputAutomationModuleFactoryInterface::getName(self.as_InputAutomationModuleFactoryInterface_mut()) }
    fn getConfigurationJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> { InputAutomationModuleFactoryInterface::getConfigurationJPanel(self.as_InputAutomationModuleFactoryInterface_mut()) }
    fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> { InputAutomationModuleFactoryInterface::getHelpSet(self.as_InputAutomationModuleFactoryInterface_mut()) }
    fn getInstance(&mut self) -> Option<Rc<RefCell<dyn RunnableInterface>>> { InputAutomationModuleFactoryInterface::getInstance(self.as_InputAutomationModuleFactoryInterface_mut()) }
}
impl InputAutomationModuleFactoryInterfaceDyn for InputAutomationModuleFactoryInterface {
    fn as_InputAutomationModuleFactoryInterface(&self) -> &InputAutomationModuleFactoryInterface { self }
    fn as_InputAutomationModuleFactoryInterface_mut(&mut self) -> &mut InputAutomationModuleFactoryInterface { self }
}


