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
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use super::TagHelperFactoryInterface::TagHelperFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontsRequestHelperFactory {
}

thread_local! { pub static StoreFrontsRequestHelperFactory_CLASSNAME: String = String::from("admin.taghelpers.StoreFrontsRequestHelper"); }
thread_local! { pub static StoreFrontsRequestHelperFactory_FACTORYNAME: String = String::from("StoreFrontsRequestHelperFactory"); }
// impl TagHelperFactoryInterface for StoreFrontsRequestHelperFactory with the methods of the trait

impl StoreFrontsRequestHelperFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException
    pub fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return HelperFactory::HelperFactory::getInstance(StoreFrontsRequestHelperFactory_FACTORYNAME.with(Clone::clone).clone(), StoreFrontsRequestHelperFactory_CLASSNAME.with(Clone::clone).clone(), hashMap.clone(), pageContext.clone());
    }
}

