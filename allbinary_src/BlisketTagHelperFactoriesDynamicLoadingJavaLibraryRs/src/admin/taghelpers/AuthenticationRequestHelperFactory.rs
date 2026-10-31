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
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use super::TagHelperFactoryInterface::TagHelperFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AuthenticationRequestHelperFactory {
}

thread_local! { pub static AuthenticationRequestHelperFactory_CLASSNAME: String = String::from("admin.taghelpers.AuthenticationRequestHelper"); }
thread_local! { pub static AuthenticationRequestHelperFactory_FACTORYNAME: String = String::from("AuthenticationRequestHelperFactory"); }
// impl TagHelperFactoryInterface for AuthenticationRequestHelperFactory with the methods of the trait

impl AuthenticationRequestHelperFactory {
    

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
        return HelperFactory::HelperFactory::getInstance(AuthenticationRequestHelperFactory_FACTORYNAME.with(Clone::clone).clone(), AuthenticationRequestHelperFactory_CLASSNAME.with(Clone::clone).clone(), hashMap.clone(), pageContext.clone());
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return HelperFactory::HelperFactory::getInstance(AuthenticationRequestHelperFactory_FACTORYNAME.with(Clone::clone).clone(), AuthenticationRequestHelperFactory_CLASSNAME.with(Clone::clone).clone(), hashMap.clone(), httpServletRequest.clone());
    }
}

