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
// package org.allbinary.osgi
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::org::osgi::framework::NullServiceReferenceFactory::NullServiceReferenceFactory;
use crate::org::osgi::framework::ServiceReference::ServiceReference;
use crate::org::osgi::framework::ServiceRegistration::ServiceRegistration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OSGIActivatorUtil {
}


impl OSGIActivatorUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn registerAsService(mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut serviceName: String) {
        OSGIActivatorUtil::registerAsService4(bundleContext.clone(), anyType.clone(), serviceName.clone(), jcall!(StdUtil::getInstance()).createHashtable());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn registerAsService4(mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut serviceName: String, mut properties: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) {
        let mut serviceRegistration: Option<Rc<RefCell<dyn ServiceRegistration>>> = jcall!(bundleContext).registerService(serviceName.clone(), crate::java_into_object(anyType.clone()), properties.clone());
        let mut serviceReference: Option<Rc<RefCell<dyn ServiceReference>>> = jcall!(serviceRegistration).getReference();
        serviceReference = jcall!(bundleContext).getServiceReference(serviceName.clone());
        if match (&serviceReference, &jcall!(NullServiceReferenceFactory::getInstance()).NULL_SERVICE_REFERENCE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such Service Reference"))));
        }
    }
}

