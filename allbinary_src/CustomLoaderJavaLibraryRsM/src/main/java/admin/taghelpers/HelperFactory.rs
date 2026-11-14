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
use super::*;
use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
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
    

    // throws LicensingException
    pub fn getInstance5(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut factoryName: String, mut className: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (2) as usize];
            let mut classes: Vec<&'static str> = vec![""; (2) as usize];
            let mut abeFactory: Option<Rc<RefCell<AbeFactory>>> = AbeFactory::getInstance();
            classes[(0) as usize] = std::any::type_name_of_val(&hashMap);
            classes[(1) as usize] = jcall!(abeFactory).getClass(abeClientInformation.clone(), String::from("javax.servlet.http.HttpServletRequest"));
            params[(0) as usize] = crate::java_into_object(crate::java_into_object((hashMap).clone()));
            params[(1) as usize] = crate::java_into_object(crate::java_into_object((httpServletRequest).clone()));
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = crate::java_into_object(jcall!(abeFactory).getInstance4(abeClientInformation.clone(), className.clone(), classes.clone(), params.clone()));
            return Some(anyType.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut error: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failed To Get Instance Args: HashMap="), jcall!(hashMap).toString()), String::from(" HttpServletRequest=")), httpServletRequest);
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                jcall!(logUtil).put(error.clone(), crate::java_into_object(format!("{}{}", factoryName, String::from("->HelperFactory"))), String::from("getInstance(String, String, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance5_AbeClientInformationInterface_String_String_HashMap_PageContext(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut factoryName: String, mut className: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (2) as usize];
            let mut classes: Vec<&'static str> = vec![""; (2) as usize];
            let mut abeFactory: Option<Rc<RefCell<AbeFactory>>> = AbeFactory::getInstance();
            classes[(0) as usize] = std::any::type_name_of_val(&hashMap);
            classes[(1) as usize] = jcall!(abeFactory).getClass(abeClientInformation.clone(), String::from("javax.servlet.jsp.PageContext"));
            params[(0) as usize] = crate::java_into_object(crate::java_into_object((hashMap).clone()));
            params[(1) as usize] = crate::java_into_object(crate::java_into_object((pageContext).clone()));
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = crate::java_into_object(jcall!(abeFactory).getInstance4(abeClientInformation.clone(), className.clone(), classes.clone(), params.clone()));
            return Some(anyType.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                let mut error: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failed To Get Instance Args: HashMap="), jcall!(hashMap).toString()), String::from(" PageContext=")), pageContext);
                jcall!(logUtil).put(error.clone(), crate::java_into_object(format!("{}{}", factoryName, String::from("->HelperFactory"))), String::from("getInstance(String, String, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance6(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut factoryName: String, mut className: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut specialhashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (3) as usize];
            let mut classes: Vec<&'static str> = vec![""; (3) as usize];
            let mut abeFactory: Option<Rc<RefCell<AbeFactory>>> = AbeFactory::getInstance();
            classes[(0) as usize] = std::any::type_name_of_val(&hashMap);
            classes[(1) as usize] = std::any::type_name_of_val(&specialhashMap);
            classes[(2) as usize] = jcall!(abeFactory).getClass(abeClientInformation.clone(), String::from("javax.servlet.jsp.PageContext"));
            params[(0) as usize] = crate::java_into_object(crate::java_into_object((hashMap).clone()));
            params[(1) as usize] = crate::java_into_object(crate::java_into_object((specialhashMap).clone()));
            params[(2) as usize] = crate::java_into_object(crate::java_into_object((pageContext).clone()));
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = crate::java_into_object(jcall!(abeFactory).getInstance4(abeClientInformation.clone(), className.clone(), classes.clone(), params.clone()));
            return Some(anyType.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                let mut error: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Failed To Get Instance Args: HashMap="), jcall!(hashMap).toString()), String::from(" PageContext=")), pageContext);
                jcall!(logUtil).put(error.clone(), crate::java_into_object(format!("{}{}", factoryName, String::from("->HelperFactory"))), String::from("getInstance(String, String, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut factoryName: String, mut className: String) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = crate::java_into_object(jcall!(AbeFactory::getInstance()).getInstance2(abeClientInformation.clone(), className.clone()));
            return Some(anyType.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(format!("{}{}", factoryName, String::from("->HelperFactory"))), String::from("getInstance(String, String, HashMap, PageContext)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

