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
use crate::org::osgi::framework::BundleActivator::BundleActivator;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::org::osgi::framework::ServiceFactory::ServiceFactory;
use crate::org::osgi::framework::ServiceReference::ServiceReference;
use crate::bundle::input::automation::module::InputAutomationModuleServiceInterface::InputAutomationModuleServiceInterface;
use crate::bundle::input::automation::module::configuration::InputAutomationConfigurationModuleChangeListener::InputAutomationConfigurationModuleChangeListener;
use crate::org::allbinary::input::automation::configuration::InputAutomationConfigurationModuleChangeEvent::InputAutomationConfigurationModuleChangeEvent;
use crate::org::allbinary::input::automation::configuration::InputAutomationConfigurationUtil::InputAutomationConfigurationUtil;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::osgi::OSGIActivatorUtil::OSGIActivatorUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleBundleActivator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub inputAutomationModuleInterface: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>,
    pub serviceFactory: Option<Rc<RefCell<dyn ServiceFactory>>>,
}

impl BundleActivator for InputAutomationModuleBundleActivator {
}

impl InputAutomationModuleBundleActivator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            inputAutomationModuleInterface: Vec::new(),
            serviceFactory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.init();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getInputAutomationConfigurationModuleChangeListener(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) -> Option<Rc<RefCell<dyn InputAutomationConfigurationModuleChangeListener>>> {
        let mut serviceReference: Option<Rc<RefCell<dyn ServiceReference>>> = jcall!(context).getServiceReference(jcall!(std::any::type_name::<InputAutomationConfigurationModuleChangeListener>()).getName());
        if serviceReference.is_some() {
            let mut inputAutomationConfigurationModuleChangeListener: Option<Rc<RefCell<dyn InputAutomationConfigurationModuleChangeListener>>> = { let java_object = jcall!(context).getService(serviceReference.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn InputAutomationConfigurationModuleChangeListener>>>>().expect("ClassCastException").clone()) };
            if inputAutomationConfigurationModuleChangeListener.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Service For ServiceReference"))));
            }
            return inputAutomationConfigurationModuleChangeListener.clone();
        } else {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("No ServiceReference: "), jcall!(std::any::type_name::<InputAutomationConfigurationModuleChangeListener>()).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addModules"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return None;
        }
    }
    

    pub fn addModules(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addModules"));
            let mut inputAutomationConfigurationModuleChangeListener: Option<Rc<RefCell<dyn InputAutomationConfigurationModuleChangeListener>>> = self.getInputAutomationConfigurationModuleChangeListener(context.clone());
            if inputAutomationConfigurationModuleChangeListener.is_some() {
                for index in (0..(self.getInputAutomationModuleInterface().len() as i32)) {
                    let mut inputAutomationConfigurationModuleChangeEvent: Option<Rc<RefCell<InputAutomationConfigurationModuleChangeEvent>>> = InputAutomationConfigurationUtil::getChangeEvent(self.getInputAutomationModuleInterface()[(index) as usize].clone());
                    jcall!(inputAutomationConfigurationModuleChangeListener).onAdd(inputAutomationConfigurationModuleChangeEvent.clone());
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).putF(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addModules"));
            }
        }
    }
    

    // throws Exception
    pub fn registerAsService(&mut self, mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>) {
        OSGIActivatorUtil::registerAsService(bundleContext.clone(), self.getServiceFactory(), jcall!(std::any::type_name::<InputAutomationModuleServiceInterface>()).getName());
    }
    

    pub fn removeModules(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeModules"));
            let mut inputAutomationConfigurationModuleChangeListener: Option<Rc<RefCell<dyn InputAutomationConfigurationModuleChangeListener>>> = self.getInputAutomationConfigurationModuleChangeListener(context.clone());
            if inputAutomationConfigurationModuleChangeListener.is_some() {
                for index in (0..(self.getInputAutomationModuleInterface().len() as i32)) {
                    let mut inputAutomationConfigurationModuleChangeEvent: Option<Rc<RefCell<InputAutomationConfigurationModuleChangeEvent>>> = InputAutomationConfigurationUtil::getChangeEvent(self.getInputAutomationModuleInterface()[(index) as usize].clone());
                    jcall!(inputAutomationConfigurationModuleChangeListener).onRemove(inputAutomationConfigurationModuleChangeEvent.clone());
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).putF(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeModules"));
            }
        }
    }
    

    // throws Exception
    pub fn start(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).START.clone());
        self.addModules(context.clone());
        self.registerAsService(context.clone());
    }
    

    // throws Exception
    pub fn stop(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("stop"));
        self.removeModules(context.clone());
    }
    

    pub fn getInputAutomationModuleInterface(&mut self) -> Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>> {
        return self.inputAutomationModuleInterface.clone();
    }
    

    pub fn setInputAutomationModuleInterface(&mut self, mut inputAutomationModuleInterface: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>) {
        self.inputAutomationModuleInterface = inputAutomationModuleInterface.clone();
    }
    

    pub fn getServiceFactory(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.serviceFactory.clone();
    }
    

    pub fn setServiceFactory(&mut self, mut serviceObject: Option<Rc<RefCell<dyn ServiceFactory>>>) {
        self.serviceFactory = serviceObject.clone();
    }
}
pub trait InputAutomationModuleBundleActivatorDyn: crate::org::osgi::framework::BundleActivator::BundleActivator {
    fn as_InputAutomationModuleBundleActivator(&self) -> &InputAutomationModuleBundleActivator;
    fn as_InputAutomationModuleBundleActivator_mut(&mut self) -> &mut InputAutomationModuleBundleActivator;
    fn init(&mut self) { InputAutomationModuleBundleActivator::init(self.as_InputAutomationModuleBundleActivator_mut()) }
    fn getInputAutomationConfigurationModuleChangeListener(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) -> Option<Rc<RefCell<dyn InputAutomationConfigurationModuleChangeListener>>> { InputAutomationModuleBundleActivator::getInputAutomationConfigurationModuleChangeListener(self.as_InputAutomationModuleBundleActivator_mut(), context) }
    fn addModules(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationModuleBundleActivator::addModules(self.as_InputAutomationModuleBundleActivator_mut(), context) }
    fn registerAsService(&mut self, mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationModuleBundleActivator::registerAsService(self.as_InputAutomationModuleBundleActivator_mut(), bundleContext) }
    fn removeModules(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationModuleBundleActivator::removeModules(self.as_InputAutomationModuleBundleActivator_mut(), context) }
    fn start(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationModuleBundleActivator::start(self.as_InputAutomationModuleBundleActivator_mut(), context) }
    fn stop(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationModuleBundleActivator::stop(self.as_InputAutomationModuleBundleActivator_mut(), context) }
    fn getInputAutomationModuleInterface(&mut self) -> Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>> { InputAutomationModuleBundleActivator::getInputAutomationModuleInterface(self.as_InputAutomationModuleBundleActivator_mut()) }
    fn setInputAutomationModuleInterface(&mut self, mut inputAutomationModuleInterface: Vec<Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>>) { InputAutomationModuleBundleActivator::setInputAutomationModuleInterface(self.as_InputAutomationModuleBundleActivator_mut(), inputAutomationModuleInterface) }
    fn getServiceFactory(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { InputAutomationModuleBundleActivator::getServiceFactory(self.as_InputAutomationModuleBundleActivator_mut()) }
    fn setServiceFactory(&mut self, mut serviceObject: Option<Rc<RefCell<dyn ServiceFactory>>>) { InputAutomationModuleBundleActivator::setServiceFactory(self.as_InputAutomationModuleBundleActivator_mut(), serviceObject) }
}
impl InputAutomationModuleBundleActivatorDyn for InputAutomationModuleBundleActivator {
    fn as_InputAutomationModuleBundleActivator(&self) -> &InputAutomationModuleBundleActivator { self }
    fn as_InputAutomationModuleBundleActivator_mut(&mut self) -> &mut InputAutomationModuleBundleActivator { self }
}


