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
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HelperFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl HelperFactory {
    

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
    

    // throws LicensingException
    pub fn getInstance5(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut factoryName: String, mut className: String, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (2) as usize];
            let mut classes: Vec<&'static str> = vec![""; (2) as usize];
            let mut abeFactory: Option<Rc<RefCell<AbeFactory>>> = AbeFactory::getInstance();
            classes[(0) as usize] = std::any::type_name_of_val(&hashMap);
            classes[(1) as usize] = jcall!(abeFactory).getClass(abeClientInformation.clone(), String::from("javax.servlet.http.HttpServletRequest"));
            params[(0) as usize] = crate::java_into_object((hashMap).clone() /* as Any */);
            params[(1) as usize] = crate::java_into_object((httpServletRequest).clone() /* as Any */);
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(abeFactory).getInstance4(abeClientInformation.clone(), className.clone(), classes.clone(), params.clone());
            return anyType.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failed To Get Instance Args: HashMap="), jcall!(hashMap).toString()), String::from(" HttpServletRequest=")), httpServletRequest);
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                jcall!(logUtil).put(error.clone(), crate::java_into_object(format!("{}{}", factoryName, String::from("->HelperFactory"))), String::from("getInstance(String, String, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance5(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut factoryName: String, mut className: String, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (2) as usize];
            let mut classes: Vec<&'static str> = vec![""; (2) as usize];
            let mut abeFactory: Option<Rc<RefCell<AbeFactory>>> = AbeFactory::getInstance();
            classes[(0) as usize] = std::any::type_name_of_val(&hashMap);
            classes[(1) as usize] = jcall!(abeFactory).getClass(abeClientInformation.clone(), String::from("javax.servlet.jsp.PageContext"));
            params[(0) as usize] = crate::java_into_object((hashMap).clone() /* as Any */);
            params[(1) as usize] = crate::java_into_object((pageContext).clone() /* as Any */);
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(abeFactory).getInstance4(abeClientInformation.clone(), className.clone(), classes.clone(), params.clone());
            return anyType.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                let mut error: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failed To Get Instance Args: HashMap="), jcall!(hashMap).toString()), String::from(" PageContext=")), pageContext);
                jcall!(logUtil).put(error.clone(), crate::java_into_object(format!("{}{}", factoryName, String::from("->HelperFactory"))), String::from("getInstance(String, String, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance6(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut factoryName: String, mut className: String, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut specialhashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (3) as usize];
            let mut classes: Vec<&'static str> = vec![""; (3) as usize];
            let mut abeFactory: Option<Rc<RefCell<AbeFactory>>> = AbeFactory::getInstance();
            classes[(0) as usize] = std::any::type_name_of_val(&hashMap);
            classes[(1) as usize] = std::any::type_name_of_val(&specialhashMap);
            classes[(2) as usize] = jcall!(abeFactory).getClass(abeClientInformation.clone(), String::from("javax.servlet.jsp.PageContext"));
            params[(0) as usize] = crate::java_into_object((hashMap).clone() /* as Any */);
            params[(1) as usize] = crate::java_into_object((specialhashMap).clone() /* as Any */);
            params[(2) as usize] = crate::java_into_object((pageContext).clone() /* as Any */);
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(abeFactory).getInstance4(abeClientInformation.clone(), className.clone(), classes.clone(), params.clone());
            return anyType.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                let mut error: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failed To Get Instance Args: HashMap="), jcall!(hashMap).toString()), String::from(" PageContext=")), pageContext);
                jcall!(logUtil).put(error.clone(), crate::java_into_object(format!("{}{}", factoryName, String::from("->HelperFactory"))), String::from("getInstance(String, String, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut factoryName: String, mut className: String) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AbeFactory::getInstance()).getInstance2(abeClientInformation.clone(), className.clone());
            return anyType.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERFACTORYERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(format!("{}{}", factoryName, String::from("->HelperFactory"))), String::from("getInstance(String, String, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

