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
// use crate::java::net::URL::URL;
// use crate::javax::help::HelpSet::HelpSet;
// use crate::javax::swing::*;
use crate::org::allbinary::logic::java::help::JavaHelpUtil::JavaHelpUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbstractInputAutomationFactory {
    pub base: InputAutomationModuleFactoryInterface,
    pub name: String,
    pub jPanel: Option<Rc<RefCell<JPanel>>>,
    pub helpSet: Option<Rc<RefCell<HelpSet>>>,
}


impl AbstractInputAutomationFactory {
    

    pub fn new_struct3(mut name: String, mut jPanel: Option<Rc<RefCell<JPanel>>>, mut helpSet: Option<Rc<RefCell<HelpSet>>>) -> Self {
        let mut this = Self {
            base: InputAutomationModuleFactoryInterface::new_struct(),
            name: String::new(),
            jPanel: None,
            helpSet: None,
        };
        this.name = name.clone();
        this.jPanel = jPanel.clone();
        this.helpSet = helpSet.clone();
        this
    }

    pub fn new3(mut name: String, mut jPanel: Option<Rc<RefCell<JPanel>>>, mut helpSet: Option<Rc<RefCell<HelpSet>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(name, jPanel, helpSet))))
    }
    

    pub fn new_struct2(mut name: String, mut jPanel: Option<Rc<RefCell<JPanel>>>) -> Self {
        let mut this = Self {
            base: InputAutomationModuleFactoryInterface::new_struct(),
            name: String::new(),
            jPanel: None,
            helpSet: None,
        };
        this.name = name.clone();
        this.jPanel = jPanel.clone();
        let mut url: Option<Rc<RefCell<URL>>> = jcall!(std::any::type_name_of_val(&this)).getResource(String::from("/help/Help.hs"));
        this.helpSet = jcall!(JavaHelpUtil::getInstance()).getHelpSet(url.clone());
        this
    }

    pub fn new2(mut name: String, mut jPanel: Option<Rc<RefCell<JPanel>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(name, jPanel))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getConfigurationJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.jPanel.clone();
    }
    

    pub fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> {
        return self.helpSet.clone();
    }
}
impl std::ops::Deref for AbstractInputAutomationFactory {
    type Target = InputAutomationModuleFactoryInterface;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbstractInputAutomationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbstractInputAutomationFactoryDyn: crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn {
    fn as_AbstractInputAutomationFactory(&self) -> &AbstractInputAutomationFactory;
    fn as_AbstractInputAutomationFactory_mut(&mut self) -> &mut AbstractInputAutomationFactory;
}
impl AbstractInputAutomationFactoryDyn for AbstractInputAutomationFactory {
    fn as_AbstractInputAutomationFactory(&self) -> &AbstractInputAutomationFactory { self }
    fn as_AbstractInputAutomationFactory_mut(&mut self) -> &mut AbstractInputAutomationFactory { self }
}

impl crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn for AbstractInputAutomationFactory {
    fn as_InputAutomationModuleFactoryInterface(&self) -> &crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface { &self.base }
    fn as_InputAutomationModuleFactoryInterface_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface { &mut self.base }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getConfigurationJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> { Self::getConfigurationJPanel(self) }
    fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> { Self::getHelpSet(self) }
}


