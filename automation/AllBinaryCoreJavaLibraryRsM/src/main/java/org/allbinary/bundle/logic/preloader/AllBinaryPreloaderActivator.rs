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
// package org.allbinary.bundle.logic.preloader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::osgi::framework::BundleActivator::BundleActivator;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::org::osgi::framework::ServiceReference::ServiceReference;
use crate::org::osgi::framework::ServiceRegistration::ServiceRegistration;
use crate::org::allbinary::bundle::logic::system::loader::CryptServiceFactory::CryptServiceFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryPreloaderActivator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

pub static mut AllBinaryPreloaderActivator_context: std::cell::LazyCell<Option<Rc<RefCell<dyn BundleContext>>>> = std::cell::LazyCell::new(|| None);
pub static AllBinaryPreloaderActivator_CRYPT_REGISTRY_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("bundle.abcs.logic.system.loader.CryptService"));
impl BundleActivator for AllBinaryPreloaderActivator {
}

impl AllBinaryPreloaderActivator {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getBundleContext() -> Option<Rc<RefCell<dyn BundleContext>>> {
        return unsafe { AllBinaryPreloaderActivator_context.clone() }.clone();
    }
    

    // throws Exception
    pub fn start(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("start"));
            unsafe { *AllBinaryPreloaderActivator_context = context.clone() };
            self.registerAsService();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("start"), crate::java_into_object(e.clone()));
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn registerAsService(&mut self) {
        let mut properties: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        let mut serviceRegistration: Option<Rc<RefCell<dyn ServiceRegistration>>> = jcall!(unsafe { AllBinaryPreloaderActivator_context.clone() }).registerService(AllBinaryPreloaderActivator_CRYPT_REGISTRY_NAME.with(Clone::clone).clone(), crate::java_into_object(CryptServiceFactory::new()), properties.clone());
        let mut serviceReference: Option<Rc<RefCell<dyn ServiceReference>>> = jcall!(serviceRegistration).getReference();
        serviceReference = { let java_value = (jcall!(unsafe { AllBinaryPreloaderActivator_context.clone() }).getServiceReference(AllBinaryPreloaderActivator_CRYPT_REGISTRY_NAME.with(Clone::clone).clone())).clone(); java_value };
        if serviceReference.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such Service Reference"))));
        }
    }
    

    // throws Exception
    pub fn stop(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("stop"));
    }
}

