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
// package org.allbinary.input.automation.robot.osgi
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::osgi::framework::BundleActivator::BundleActivator;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::org::osgi::framework::ServiceFactory::ServiceFactory;
use crate::org::osgi::framework::ServiceReference::ServiceReference;
use crate::bundle::input::automation::robot::InputAutomationRobotChangeListener::InputAutomationRobotChangeListener;
use crate::bundle::input::automation::robot::InputAutomationRobotServiceInterface::InputAutomationRobotServiceInterface;
use crate::org::allbinary::input::automation::robot::InputRobotInterface::InputRobotInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::osgi::OSGIActivatorUtil::OSGIActivatorUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::InputAutomationRobotChangeEvent::InputAutomationRobotChangeEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationRobotBundleActivator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub inputRobotInterface: Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>>,
    pub serviceFactory: Option<Rc<RefCell<dyn ServiceFactory>>>,
}

impl BundleActivator for InputAutomationRobotBundleActivator {
}

impl InputAutomationRobotBundleActivator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            inputRobotInterface: Vec::new(),
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
    pub fn registerAsService(&mut self, mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>) {
        OSGIActivatorUtil::registerAsService(bundleContext.clone(), self.getServiceFactory(), jcall!(std::any::type_name::<InputAutomationRobotServiceInterface>()).getName());
    }
    

    // throws Exception
    pub fn getInputAutomationRobotChangeListener(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) -> Option<Rc<RefCell<dyn InputAutomationRobotChangeListener>>> {
        let mut serviceReference: Option<Rc<RefCell<dyn ServiceReference>>> = jcall!(context).getServiceReference(jcall!(std::any::type_name::<InputAutomationRobotChangeListener>()).getName());
        if serviceReference.is_some() {
            let mut inputAutomationRobotChangeListener: Option<Rc<RefCell<dyn InputAutomationRobotChangeListener>>> = { let java_object = jcall!(context).getService(serviceReference.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn InputAutomationRobotChangeListener>>>>().expect("ClassCastException").clone()) };
            if inputAutomationRobotChangeListener.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Service For ServiceReference"))));
            }
            return inputAutomationRobotChangeListener.clone();
        } else {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("No ServiceReference: "), jcall!(std::any::type_name::<InputAutomationRobotChangeListener>()).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInputAutomationRobotChangeListener"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return None;
        }
    }
    

    pub fn addRobots(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addRobots"));
            let mut inputAutomationRobotChangeListener: Option<Rc<RefCell<dyn InputAutomationRobotChangeListener>>> = self.getInputAutomationRobotChangeListener(context.clone());
            if inputAutomationRobotChangeListener.is_some() {
                for index in (0..(self.getInputRobotInterface().len() as i32)) {
                    let mut inputAutomationRobotChangeEvent: Option<Rc<RefCell<InputAutomationRobotChangeEvent>>> = InputAutomationRobotUtil::InputAutomationRobotUtil::getChangeEvent(self.getInputRobotInterface()[(index) as usize].clone());
                    jcall!(inputAutomationRobotChangeListener).onAdd(inputAutomationRobotChangeEvent.clone());
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
    

    pub fn removeRobots(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeRobots"));
            let mut inputAutomationRobotChangeListener: Option<Rc<RefCell<dyn InputAutomationRobotChangeListener>>> = self.getInputAutomationRobotChangeListener(context.clone());
            if inputAutomationRobotChangeListener.is_some() {
                for index in (0..(self.getInputRobotInterface().len() as i32)) {
                    let mut inputAutomationRobotChangeEvent: Option<Rc<RefCell<InputAutomationRobotChangeEvent>>> = InputAutomationRobotUtil::InputAutomationRobotUtil::getChangeEvent(self.getInputRobotInterface()[(index) as usize].clone());
                    jcall!(inputAutomationRobotChangeListener).onRemove(inputAutomationRobotChangeEvent.clone());
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).putF(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeRobots"));
            }
        }
    }
    

    // throws Exception
    pub fn start(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).START.clone());
        self.addRobots(context.clone());
        self.registerAsService(context.clone());
    }
    

    // throws Exception
    pub fn stop(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        jcall!(self.logUtil).putF(String::from("Stop"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).START.clone());
        self.removeRobots(context.clone());
    }
    

    pub fn getServiceFactory(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.serviceFactory.clone();
    }
    

    pub fn setServiceFactory(&mut self, mut serviceObject: Option<Rc<RefCell<dyn ServiceFactory>>>) {
        self.serviceFactory = serviceObject.clone();
    }
    

    pub fn getInputRobotInterface(&mut self) -> Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>> {
        return self.inputRobotInterface.clone();
    }
    

    pub fn setInputRobotInterface(&mut self, mut inputRobotInterface: Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>>) {
        self.inputRobotInterface = inputRobotInterface.clone();
    }
}
pub trait InputAutomationRobotBundleActivatorDyn: crate::org::osgi::framework::BundleActivator::BundleActivator {
    fn as_InputAutomationRobotBundleActivator(&self) -> &InputAutomationRobotBundleActivator;
    fn as_InputAutomationRobotBundleActivator_mut(&mut self) -> &mut InputAutomationRobotBundleActivator;
    fn init(&mut self) { InputAutomationRobotBundleActivator::init(self.as_InputAutomationRobotBundleActivator_mut()) }
    fn registerAsService(&mut self, mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationRobotBundleActivator::registerAsService(self.as_InputAutomationRobotBundleActivator_mut(), bundleContext) }
    fn getInputAutomationRobotChangeListener(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) -> Option<Rc<RefCell<dyn InputAutomationRobotChangeListener>>> { InputAutomationRobotBundleActivator::getInputAutomationRobotChangeListener(self.as_InputAutomationRobotBundleActivator_mut(), context) }
    fn addRobots(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationRobotBundleActivator::addRobots(self.as_InputAutomationRobotBundleActivator_mut(), context) }
    fn removeRobots(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationRobotBundleActivator::removeRobots(self.as_InputAutomationRobotBundleActivator_mut(), context) }
    fn start(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationRobotBundleActivator::start(self.as_InputAutomationRobotBundleActivator_mut(), context) }
    fn stop(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationRobotBundleActivator::stop(self.as_InputAutomationRobotBundleActivator_mut(), context) }
    fn getServiceFactory(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { InputAutomationRobotBundleActivator::getServiceFactory(self.as_InputAutomationRobotBundleActivator_mut()) }
    fn setServiceFactory(&mut self, mut serviceObject: Option<Rc<RefCell<dyn ServiceFactory>>>) { InputAutomationRobotBundleActivator::setServiceFactory(self.as_InputAutomationRobotBundleActivator_mut(), serviceObject) }
    fn getInputRobotInterface(&mut self) -> Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>> { InputAutomationRobotBundleActivator::getInputRobotInterface(self.as_InputAutomationRobotBundleActivator_mut()) }
    fn setInputRobotInterface(&mut self, mut inputRobotInterface: Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>>) { InputAutomationRobotBundleActivator::setInputRobotInterface(self.as_InputAutomationRobotBundleActivator_mut(), inputRobotInterface) }
}
impl InputAutomationRobotBundleActivatorDyn for InputAutomationRobotBundleActivator {
    fn as_InputAutomationRobotBundleActivator(&self) -> &InputAutomationRobotBundleActivator { self }
    fn as_InputAutomationRobotBundleActivator_mut(&mut self) -> &mut InputAutomationRobotBundleActivator { self }
}


