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
// package org.allbinary.business.init.db
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use super::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitDbHelperFactory {
}

thread_local! { pub static InitDbHelperFactory_CLASSNAME: String = String::from("org.allbinary.business.installer.InitDbCrypted"); }

impl InitDbHelperFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException
    pub fn getInstance2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (1) as usize];
        let mut classes: Vec<&'static str> = vec![""; (1) as usize];
        classes[(0) as usize] = std::any::type_name::<DatabaseConnectionInfoInterface>();
        params[(0) as usize] = crate::java_into_object((databaseConnectionInfoInterface).clone() /* as Any */);
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AbeFactory::getInstance()).getInstance4(abeClientInformation.clone(), InitDbHelperFactory_CLASSNAME.with(Clone::clone).clone(), classes.clone(), params.clone());
        return anyType.clone();
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AbeFactory::getInstance()).getInstance4(abeClientInformation.clone(), InitDbHelperFactory_CLASSNAME.with(Clone::clone).clone(), None, None);
        return anyType.clone();
    }
}

