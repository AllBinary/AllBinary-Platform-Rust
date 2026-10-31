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
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::org::osgi::framework::ServiceReference::ServiceReference;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::osgi::OSGIServiceInterface::OSGIServiceInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OSGIServiceUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static OSGIServiceUtil_instance: Option<Rc<RefCell<OSGIServiceUtil>>> = OSGIServiceUtil::new(); }

impl OSGIServiceUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<OSGIServiceUtil>>> {
        return OSGIServiceUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getServicesObjectVector(&mut self, mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>, mut serviceReferences: Vec<Option<Rc<RefCell<dyn ServiceReference>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        if serviceReferences.is_some() {
            let mut size: i32 = (serviceReferences.len() as i32).clone();
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Service References: "), size), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getServicesObjectVector"));
            for index in (0..size) {
                let mut serviceReference: Option<Rc<RefCell<dyn ServiceReference>>> = serviceReferences[(index) as usize].clone();
                if serviceReference.is_some() {
                    let mut osgiServiceInterface: Option<Rc<RefCell<dyn OSGIServiceInterface>>> = { let java_object = jcall!(bundleContext).getService(serviceReference.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn OSGIServiceInterface>>>>().expect("ClassCastException").clone()) };
                    if osgiServiceInterface.is_none() {
                        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Service For Reference"))));
                    }
                    jcall!(vector).add(crate::java_into_object(osgiServiceInterface.clone()));
                }
            }
        }
        return vector.clone();
    }
}

