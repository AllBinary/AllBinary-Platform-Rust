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
// package org.allbinary.input.automation.module.game.test
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::swing::*;
use crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactory;
use crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData;
use crate::org::allbinary::input::automation::module::configuration::BlankModuleConfigurationJPanel::BlankModuleConfigurationJPanel;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TestModuleFactory {
    pub base: AbstractInputAutomationFactory,
    pub blankModuleConfigurationJPanel: Option<Rc<RefCell<BlankModuleConfigurationJPanel>>>,
}

pub static mut TestModuleFactory_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| format!("{}{}", String::from("Test New Game"), unsafe { crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData_MODULE_NAME_END.clone() }));

impl TestModuleFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbstractInputAutomationFactory::new_struct2(unsafe { TestModuleFactory_NAME.clone() }.clone(), BlankModuleConfigurationJPanel::new()),
            blankModuleConfigurationJPanel: None,
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        return unsafe { TestModuleFactory_NAME.clone() }.clone();
    }
    

    pub fn getConfigurationJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.blankModuleConfigurationJPanel.clone();
    }
    

    // throws Exception
    // synchronized
    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn RunnableInterface>>> {
        return None;
    }
}
impl std::ops::Deref for TestModuleFactory {
    type Target = AbstractInputAutomationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TestModuleFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactoryDyn for TestModuleFactory {
    fn as_AbstractInputAutomationFactory(&self) -> &crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactory { &self.base }
    fn as_AbstractInputAutomationFactory_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactory { &mut self.base }
}
impl crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn for TestModuleFactory {
    fn as_InputAutomationModuleFactoryInterface(&self) -> &crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface { &self.base.base }
    fn as_InputAutomationModuleFactoryInterface_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface { &mut self.base.base }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getConfigurationJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> { Self::getConfigurationJPanel(self) }
    fn getInstance(&mut self) -> Option<Rc<RefCell<dyn RunnableInterface>>> { Self::getInstance(self) }
}


