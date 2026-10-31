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
use crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactory;
use crate::org::allbinary::input::automation::module::InputAutomationActionInterface::InputAutomationActionInterface;
use crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData;
use crate::org::allbinary::input::automation::module::generic::configuration::GenericModuleConfigurationJPanel::GenericModuleConfigurationJPanel;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfile::GenericProfile;
use crate::org::allbinary::input::automation::module::generic::constraints::NoImageComparatorConstraints::NoImageComparatorConstraints;
use crate::org::allbinary::input::automation::module::generic::constraints::NoMotionRectangleConstraints::NoMotionRectangleConstraints;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;
use super::GenericInputAutomationAction::GenericInputAutomationAction;
use super::GenericInputAutomationWorker::GenericInputAutomationWorker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericModuleFactory {
    pub base: AbstractInputAutomationFactory,
}

pub static mut GenericModuleFactory_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| format!("{}{}", String::from("Generic"), unsafe { crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData_MODULE_NAME_END.clone() }));

impl GenericModuleFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbstractInputAutomationFactory::new_struct2(unsafe { GenericModuleFactory_NAME.clone() }.clone(), GenericModuleConfigurationJPanel::new()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn RunnableInterface>>> {
        let mut genericProfile: Option<Rc<RefCell<GenericProfile>>> = jcall!(((self.getConfigurationJPanel()).clone() /* as GenericModuleConfigurationJPanel */)).getSelectedGenericProfile();
        let mut inputAutomationActionInterface: Option<Rc<RefCell<dyn InputAutomationActionInterface>>> = (GenericInputAutomationAction::new()).clone() /* as InputAutomationActionInterface */;
        return (GenericInputAutomationWorker::new(inputAutomationActionInterface.clone(), genericProfile.clone(), NoMotionRectangleConstraints::new(), NoImageComparatorConstraints::new())).clone() /* as RunnableInterface */;
    }
}
impl std::ops::Deref for GenericModuleFactory {
    type Target = AbstractInputAutomationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericModuleFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactoryDyn for GenericModuleFactory {
    fn as_AbstractInputAutomationFactory(&self) -> &crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactory { &self.base }
    fn as_AbstractInputAutomationFactory_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactory { &mut self.base }
}
impl crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn for GenericModuleFactory {
    fn as_InputAutomationModuleFactoryInterface(&self) -> &crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface { &self.base.base }
    fn as_InputAutomationModuleFactoryInterface_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface { &mut self.base.base }
    fn getName(&mut self) -> String { crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactory::getName(crate::org::allbinary::input::automation::module::AbstractInputAutomationFactory::AbstractInputAutomationFactoryDyn::as_AbstractInputAutomationFactory_mut(self)) }
    fn getInstance(&mut self) -> Option<Rc<RefCell<dyn RunnableInterface>>> { Self::getInstance(self) }
}


