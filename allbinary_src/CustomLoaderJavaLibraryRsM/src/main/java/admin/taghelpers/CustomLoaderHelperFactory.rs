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
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use super::TagHelperFactoryInterface::TagHelperFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomLoaderHelperFactory {
}

thread_local! { pub static CustomLoaderHelperFactory_CLASSNAME: String = String::from("admin.taghelpers.CustomLoaderHelper"); }
thread_local! { pub static CustomLoaderHelperFactory_FACTORYNAME: String = String::from("CustomLoaderHelperFactory"); }
impl TagHelperFactoryInterface for CustomLoaderHelperFactory {
    fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getInstance2(self, hashMap, pageContext) }
}

impl CustomLoaderHelperFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException
    pub fn getInstance2(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>> = ServiceClientInformationInterfaceFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn AbeClientInformationInterface>>);
        return HelperFactory::HelperFactory::getInstance5(abeClientInformation.clone(), CustomLoaderHelperFactory_FACTORYNAME.with(Clone::clone).clone(), CustomLoaderHelperFactory_CLASSNAME.with(Clone::clone).clone(), hashMap.clone(), pageContext.clone());
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return HelperFactory::HelperFactory::getInstance(abeClientInformation.clone(), CustomLoaderHelperFactory_FACTORYNAME.with(Clone::clone).clone(), CustomLoaderHelperFactory_CLASSNAME.with(Clone::clone).clone());
    }
}

