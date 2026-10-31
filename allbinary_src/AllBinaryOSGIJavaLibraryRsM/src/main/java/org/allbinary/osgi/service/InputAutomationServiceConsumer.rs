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
// package org.allbinary.osgi.service
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::org::osgi::framework::ServiceReference::ServiceReference;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::osgi::OSGIServiceInterface::OSGIServiceInterface;
use crate::org::allbinary::osgi::OSGIServiceVisitorInterface::OSGIServiceVisitorInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::OSGIServiceUtil::OSGIServiceUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationServiceConsumer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub osgiServiceUtil: Option<Rc<RefCell<OSGIServiceUtil>>>,
    pub registryName: String,
    pub bundleContext: Option<Rc<RefCell<dyn BundleContext>>>,
    pub osgiServiceVisitorInterface: Option<Rc<RefCell<dyn OSGIServiceVisitorInterface>>>,
}


impl InputAutomationServiceConsumer {
    

    pub fn new_struct(mut registryName: String, mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>, mut osgiServiceVisitorInterface: Option<Rc<RefCell<dyn OSGIServiceVisitorInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            osgiServiceUtil: None,
            registryName: String::new(),
            bundleContext: None,
            osgiServiceVisitorInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.osgiServiceUtil = OSGIServiceUtil::getInstance();
        this.setRegistryName(registryName.clone());
        this.setBundleContext(bundleContext.clone());
        this.setOsgiServiceVisitorInterface(osgiServiceVisitorInterface.clone());
        this
    }

    pub fn new(mut registryName: String, mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>, mut osgiServiceVisitorInterface: Option<Rc<RefCell<dyn OSGIServiceVisitorInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(registryName, bundleContext, osgiServiceVisitorInterface))))
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_receiver = self.osgiServiceUtil.clone(); let java_arguments = (self.getBundleContext(), self.getServiceReferences(),); let java_result = jcall!(java_receiver).getServicesObjectVector(java_arguments.0, java_arguments.1); java_result };
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", String::from("Processing "), jcall!(vector).size()), String::from(" Services")), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut size: i32 = jcall!(vector).size();
        let mut osgiServiceInterface: Option<Rc<RefCell<dyn OSGIServiceInterface>>> = None;
        for index in (0..size) {
            osgiServiceInterface = { let java_object = jcall!(vector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn OSGIServiceInterface>>>>().expect("ClassCastException").clone()) };
            if !jcall!(self.getOsgiServiceVisitorInterface()).visit(osgiServiceInterface.clone()) {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Unable to process service: "), osgiServiceInterface))));
            }
        }
    }
    

    // throws Exception
    pub fn getServiceReferences(&mut self) -> Vec<Option<Rc<RefCell<dyn ServiceReference>>>> {
        return jcall!(self.getBundleContext()).getServiceReferences(self.getRegistryName(), None);
    }
    

    pub fn getBundleContext(&mut self) -> Option<Rc<RefCell<dyn BundleContext>>> {
        return self.bundleContext.clone();
    }
    

    pub fn setBundleContext(&mut self, mut aBundleContext: Option<Rc<RefCell<dyn BundleContext>>>) {
        self.bundleContext = aBundleContext.clone();
    }
    

    pub fn getRegistryName(&mut self) -> String {
        return self.registryName.clone();
    }
    

    pub fn setRegistryName(&mut self, mut registryName: String) {
        self.registryName = registryName.clone();
    }
    

    pub fn getOsgiServiceVisitorInterface(&mut self) -> Option<Rc<RefCell<dyn OSGIServiceVisitorInterface>>> {
        return self.osgiServiceVisitorInterface.clone();
    }
    

    pub fn setOsgiServiceVisitorInterface(&mut self, mut osgiServiceVisitorInterface: Option<Rc<RefCell<dyn OSGIServiceVisitorInterface>>>) {
        self.osgiServiceVisitorInterface = osgiServiceVisitorInterface.clone();
    }
}
pub trait InputAutomationServiceConsumerDyn {
    fn as_InputAutomationServiceConsumer(&self) -> &InputAutomationServiceConsumer;
    fn as_InputAutomationServiceConsumer_mut(&mut self) -> &mut InputAutomationServiceConsumer;
    fn process(&mut self) { InputAutomationServiceConsumer::process(self.as_InputAutomationServiceConsumer_mut()) }
    fn getServiceReferences(&mut self) -> Vec<Option<Rc<RefCell<dyn ServiceReference>>>> { InputAutomationServiceConsumer::getServiceReferences(self.as_InputAutomationServiceConsumer_mut()) }
    fn getBundleContext(&mut self) -> Option<Rc<RefCell<dyn BundleContext>>> { InputAutomationServiceConsumer::getBundleContext(self.as_InputAutomationServiceConsumer_mut()) }
    fn setBundleContext(&mut self, mut aBundleContext: Option<Rc<RefCell<dyn BundleContext>>>) { InputAutomationServiceConsumer::setBundleContext(self.as_InputAutomationServiceConsumer_mut(), aBundleContext) }
    fn getRegistryName(&mut self) -> String { InputAutomationServiceConsumer::getRegistryName(self.as_InputAutomationServiceConsumer_mut()) }
    fn setRegistryName(&mut self, mut registryName: String) { InputAutomationServiceConsumer::setRegistryName(self.as_InputAutomationServiceConsumer_mut(), registryName) }
    fn getOsgiServiceVisitorInterface(&mut self) -> Option<Rc<RefCell<dyn OSGIServiceVisitorInterface>>> { InputAutomationServiceConsumer::getOsgiServiceVisitorInterface(self.as_InputAutomationServiceConsumer_mut()) }
    fn setOsgiServiceVisitorInterface(&mut self, mut osgiServiceVisitorInterface: Option<Rc<RefCell<dyn OSGIServiceVisitorInterface>>>) { InputAutomationServiceConsumer::setOsgiServiceVisitorInterface(self.as_InputAutomationServiceConsumer_mut(), osgiServiceVisitorInterface) }
}
impl InputAutomationServiceConsumerDyn for InputAutomationServiceConsumer {
    fn as_InputAutomationServiceConsumer(&self) -> &InputAutomationServiceConsumer { self }
    fn as_InputAutomationServiceConsumer_mut(&mut self) -> &mut InputAutomationServiceConsumer { self }
}


