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
// package org.allbinary.logic.communication.http.request
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::RequestDispatcher::RequestDispatcher;
// use crate::javax::servlet::ServletException::ServletException;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::http::HttpServletResponse::HttpServletResponse;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbResponseHandler {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl AbResponseHandler {
    

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
    

    // throws JspTagException
    pub fn sendJspTagRedirect(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut e: String) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).JSPTAGERROR.clone())) {
                jcall!(logUtil).put(String::from("Jsp Tag Exception"), crate::java_into_object(String::from("AbResponseHandler")), String::from("sendJspRedirect()"), crate::java_into_object(e.clone()));
            }
            AbResponseHandler::sendRedirect(pageContext.clone(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).ERRORPAGE.clone());
        })) {
            Ok(_) => {}
            Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).LICENSINGERROR.clone())) {
                jcall!(logUtil).put(String::from("Exception in Redirect Handling"), crate::java_into_object(String::from("AbResponseHandler")), String::from("sendJspRedirect"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "JspTagException", ()));
            }
        }
    }
    

    // throws ServletException
    pub fn sendServletRedirect(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut e: String) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).JSPTAGERROR.clone())) {
                jcall!(logUtil).put(String::from("Jsp Tag Exception"), crate::java_into_object(String::from("AbResponseHandler")), String::from("sendJspRedirect()"), crate::java_into_object(e.clone()));
            }
            AbResponseHandler::sendRedirect(pageContext.clone(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).ERRORPAGE.clone());
        })) {
            Ok(_) => {}
            Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).LICENSINGERROR.clone())) {
                jcall!(logUtil).put(String::from("Exception in Redirect Handling"), crate::java_into_object(String::from("AbResponseHandler")), String::from("sendJspRedirect"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "ServletException", ()));
            }
        }
    }
    

    // throws JspTagException
    pub fn sendJspTagLicensingRedirect(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut e: String) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).JSPTAGERROR.clone())) {
                jcall!(logUtil).put(String::from("Licensing Exception"), crate::java_into_object(String::from("AbResponseHandler")), String::from("sendJspTagLicensingRedirect()"), crate::java_into_object(e.clone()));
            }
            AbResponseHandler::sendRedirect(pageContext.clone(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).LICENSEERRORPAGE.clone());
        })) {
            Ok(_) => {}
            Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).LICENSINGERROR.clone())) {
                jcall!(logUtil).put(String::from("Exception in Redirect Handling"), crate::java_into_object(String::from("AbResponseHandler")), String::from("sendJspTagLicensingRedirect"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "JspTagException", ()));
            }
        }
    }
    

    // throws ServletException
    pub fn sendServletLicensingRedirect(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut e: String) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).JSPTAGERROR.clone())) {
                jcall!(logUtil).put(String::from("Licensing Exception"), crate::java_into_object(String::from("AbResponseHandler")), String::from("sendServletLicensingRedirect()"), crate::java_into_object(e.clone()));
            }
            AbResponseHandler::sendRedirect(pageContext.clone(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).LICENSEERRORPAGE.clone());
        })) {
            Ok(_) => {}
            Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).LICENSINGERROR.clone())) {
                jcall!(logUtil).put(String::from("Exception in Redirect Handling"), crate::java_into_object(String::from("AbResponseHandler")), String::from("sendServletLicensingRedirect"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "ServletException", ()));
            }
        }
    }
    

    // throws Exception
    pub fn sendRedirect(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut page: String) {
        let mut request: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        let mut response: Option<Rc<RefCell<HttpServletResponse>>> = (jcall!(pageContext).getResponse()).clone() /* as HttpServletResponse */;
        let mut requestDispatcher: Option<Rc<RefCell<RequestDispatcher>>> = jcall!(request).getRequestDispatcher(page.clone());
        if jcall!(response).isCommitted() {
            jcall!(response).sendRedirect(page.clone());
        } else {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(requestDispatcher).forward(request.clone(), response.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(jcall!(pageContext).getOut()).print(format!("{}{}", String::from("Error: "), page));
                }
            }
        }
    }
}

