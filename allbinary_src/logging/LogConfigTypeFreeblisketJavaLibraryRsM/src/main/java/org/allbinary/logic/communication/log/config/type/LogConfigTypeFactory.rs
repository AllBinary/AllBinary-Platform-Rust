// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.logic.communication.log.config.type
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::LogConfigType::LogConfigType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogConfigTypeFactory {
    pub NO_DESCRIPTION: String,
    pub AUTHENTICATION: Option<Rc<RefCell<LogConfigType>>>,
    pub AUTHENTICATIONERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub CATEGORY: Option<Rc<RefCell<LogConfigType>>>,
    pub CATEGORYERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub CRYPT: Option<Rc<RefCell<LogConfigType>>>,
    pub CRYPTERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub EMAILLOGGING: Option<Rc<RefCell<LogConfigType>>>,
    pub EMAILLOGGINGERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub ENTITYFACTORYERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub NETBEANS_MODULE: Option<Rc<RefCell<LogConfigType>>>,
    pub NETBEANS_MODULE_ERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub FACTORYERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub FILE: Option<Rc<RefCell<LogConfigType>>>,
    pub FILEERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub HTTP: Option<Rc<RefCell<LogConfigType>>>,
    pub HTTPERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub HTTPREQUEST: Option<Rc<RefCell<LogConfigType>>>,
    pub HTTPREQUESTERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub IDLOGGING: Option<Rc<RefCell<LogConfigType>>>,
    pub JSPEXTRAOUTPUT: Option<Rc<RefCell<LogConfigType>>>,
    pub JSPTAGEXTRAOUTPUT: Option<Rc<RefCell<LogConfigType>>>,
    pub JSPTAG: Option<Rc<RefCell<LogConfigType>>>,
    pub JSPTAGERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub LICENSING: Option<Rc<RefCell<LogConfigType>>>,
    pub LICENSINGERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub LOADER: Option<Rc<RefCell<LogConfigType>>>,
    pub LOADERERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub OS: Option<Rc<RefCell<LogConfigType>>>,
    pub OSERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub PAYMENT: Option<Rc<RefCell<LogConfigType>>>,
    pub PAYMENTERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub PRELOADER: Option<Rc<RefCell<LogConfigType>>>,
    pub PRELOADERERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub PRODUCTSEARCHLOGGING: Option<Rc<RefCell<LogConfigType>>>,
    pub PRODUCTSEARCHLOGGINGERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub SERVLET: Option<Rc<RefCell<LogConfigType>>>,
    pub SERVLETERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub SHIPPINGERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub SQLLOGGINGERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub SQLLOGGING: Option<Rc<RefCell<LogConfigType>>>,
    pub SQLTAGS: Option<Rc<RefCell<LogConfigType>>>,
    pub SQLTAGSERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub STYLE: Option<Rc<RefCell<LogConfigType>>>,
    pub STYLEERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub TAGHELPER: Option<Rc<RefCell<LogConfigType>>>,
    pub TAGHELPERERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub TAGHELPERFACTORY: Option<Rc<RefCell<LogConfigType>>>,
    pub TAGHELPERFACTORYERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub TAX: Option<Rc<RefCell<LogConfigType>>>,
    pub TAXERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub VALIDATION: Option<Rc<RefCell<LogConfigType>>>,
    pub VALIDATIONERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub VIEW: Option<Rc<RefCell<LogConfigType>>>,
    pub VIEWERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub VIEWFACTORYERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub WORKFLOW: Option<Rc<RefCell<LogConfigType>>>,
    pub WORKFLOWERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub XMLLOGGING: Option<Rc<RefCell<LogConfigType>>>,
    pub XMLLOGGINGERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub XSLLOGGING: Option<Rc<RefCell<LogConfigType>>>,
    pub XSLLOGGINGERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub REPLACE: Option<Rc<RefCell<LogConfigType>>>,
    pub REPLACE_INFO: Option<Rc<RefCell<LogConfigType>>>,
    pub REPLACEERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub SQLLOGGINGPOOL: Option<Rc<RefCell<LogConfigType>>>,
    pub SQLLOGGINGPOOLERROR: Option<Rc<RefCell<LogConfigType>>>,
    pub STATICPAGEGENERATIONLOGGING: Option<Rc<RefCell<LogConfigType>>>,
    pub STATICPAGEGENERATIONLOGGINGERROR: Option<Rc<RefCell<LogConfigType>>>,
}

thread_local! { pub static LogConfigTypeFactory_instance: Option<Rc<RefCell<LogConfigTypeFactory>>> = LogConfigTypeFactory::new(); }

impl LogConfigTypeFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogConfigTypeFactory>>> {
        return LogConfigTypeFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NO_DESCRIPTION: String::new(),
            AUTHENTICATION: None,
            AUTHENTICATIONERROR: None,
            CATEGORY: None,
            CATEGORYERROR: None,
            CRYPT: None,
            CRYPTERROR: None,
            EMAILLOGGING: None,
            EMAILLOGGINGERROR: None,
            ENTITYFACTORYERROR: None,
            NETBEANS_MODULE: None,
            NETBEANS_MODULE_ERROR: None,
            FACTORYERROR: None,
            FILE: None,
            FILEERROR: None,
            HTTP: None,
            HTTPERROR: None,
            HTTPREQUEST: None,
            HTTPREQUESTERROR: None,
            IDLOGGING: None,
            JSPEXTRAOUTPUT: None,
            JSPTAGEXTRAOUTPUT: None,
            JSPTAG: None,
            JSPTAGERROR: None,
            LICENSING: None,
            LICENSINGERROR: None,
            LOADER: None,
            LOADERERROR: None,
            OS: None,
            OSERROR: None,
            PAYMENT: None,
            PAYMENTERROR: None,
            PRELOADER: None,
            PRELOADERERROR: None,
            PRODUCTSEARCHLOGGING: None,
            PRODUCTSEARCHLOGGINGERROR: None,
            SERVLET: None,
            SERVLETERROR: None,
            SHIPPINGERROR: None,
            SQLLOGGINGERROR: None,
            SQLLOGGING: None,
            SQLTAGS: None,
            SQLTAGSERROR: None,
            STYLE: None,
            STYLEERROR: None,
            TAGHELPER: None,
            TAGHELPERERROR: None,
            TAGHELPERFACTORY: None,
            TAGHELPERFACTORYERROR: None,
            TAX: None,
            TAXERROR: None,
            VALIDATION: None,
            VALIDATIONERROR: None,
            VIEW: None,
            VIEWERROR: None,
            VIEWFACTORYERROR: None,
            WORKFLOW: None,
            WORKFLOWERROR: None,
            XMLLOGGING: None,
            XMLLOGGINGERROR: None,
            XSLLOGGING: None,
            XSLLOGGINGERROR: None,
            REPLACE: None,
            REPLACE_INFO: None,
            REPLACEERROR: None,
            SQLLOGGINGPOOL: None,
            SQLLOGGINGPOOLERROR: None,
            STATICPAGEGENERATIONLOGGING: None,
            STATICPAGEGENERATIONLOGGINGERROR: None,
        };
        this.NO_DESCRIPTION = String::from("No Description");
        this.AUTHENTICATION = LogConfigType::new(String::from("Authentication"), this.NO_DESCRIPTION.clone());
        this.AUTHENTICATIONERROR = LogConfigType::new(String::from("Authentication Error"), this.NO_DESCRIPTION.clone());
        this.CATEGORY = LogConfigType::new(String::from("Category"), this.NO_DESCRIPTION.clone());
        this.CATEGORYERROR = LogConfigType::new(String::from("Category Error"), this.NO_DESCRIPTION.clone());
        this.CRYPT = LogConfigType::new(String::from("Crypt"), this.NO_DESCRIPTION.clone());
        this.CRYPTERROR = LogConfigType::new(String::from("Crypt Error"), this.NO_DESCRIPTION.clone());
        this.EMAILLOGGING = LogConfigType::new(String::from("Email"), this.NO_DESCRIPTION.clone());
        this.EMAILLOGGINGERROR = LogConfigType::new(String::from("Email Error"), this.NO_DESCRIPTION.clone());
        this.ENTITYFACTORYERROR = LogConfigType::new(String::from("Entity Factory Error"), this.NO_DESCRIPTION.clone());
        this.NETBEANS_MODULE = LogConfigType::new(String::from("NetBeans Module"), this.NO_DESCRIPTION.clone());
        this.NETBEANS_MODULE_ERROR = LogConfigType::new(String::from("NetBeans Module Error"), this.NO_DESCRIPTION.clone());
        this.FACTORYERROR = LogConfigType::new(String::from("General Factory Error"), this.NO_DESCRIPTION.clone());
        this.FILE = LogConfigType::new(String::from("File"), this.NO_DESCRIPTION.clone());
        this.FILEERROR = LogConfigType::new(String::from("File Error"), this.NO_DESCRIPTION.clone());
        this.HTTP = LogConfigType::new(String::from("HTTP"), this.NO_DESCRIPTION.clone());
        this.HTTPERROR = LogConfigType::new(String::from("HTTP Error"), this.NO_DESCRIPTION.clone());
        this.HTTPREQUEST = LogConfigType::new(String::from("Http Request"), this.NO_DESCRIPTION.clone());
        this.HTTPREQUESTERROR = LogConfigType::new(String::from("Http Request Error"), this.NO_DESCRIPTION.clone());
        this.IDLOGGING = LogConfigType::new(String::from("ID Logging"), this.NO_DESCRIPTION.clone());
        this.JSPEXTRAOUTPUT = LogConfigType::new(String::from("JSP Extra Output"), this.NO_DESCRIPTION.clone());
        this.JSPTAGEXTRAOUTPUT = LogConfigType::new(String::from("JSP Tag Extra Output"), this.NO_DESCRIPTION.clone());
        this.JSPTAG = LogConfigType::new(String::from("JSP Tag"), this.NO_DESCRIPTION.clone());
        this.JSPTAGERROR = LogConfigType::new(String::from("JSP Tag Error"), this.NO_DESCRIPTION.clone());
        this.LICENSING = LogConfigType::new(String::from("Licensing"), this.NO_DESCRIPTION.clone());
        this.LICENSINGERROR = LogConfigType::new(String::from("Licensing Error"), this.NO_DESCRIPTION.clone());
        this.LOADER = LogConfigType::new(String::from("Loader"), this.NO_DESCRIPTION.clone());
        this.LOADERERROR = LogConfigType::new(String::from("Loader Error"), this.NO_DESCRIPTION.clone());
        this.OS = LogConfigType::new(String::from("OS"), this.NO_DESCRIPTION.clone());
        this.OSERROR = LogConfigType::new(String::from("OS Error"), this.NO_DESCRIPTION.clone());
        this.PAYMENT = LogConfigType::new(String::from("Payment"), this.NO_DESCRIPTION.clone());
        this.PAYMENTERROR = LogConfigType::new(String::from("Payment Error"), this.NO_DESCRIPTION.clone());
        this.PRELOADER = LogConfigType::new(String::from("PreLoader"), this.NO_DESCRIPTION.clone());
        this.PRELOADERERROR = LogConfigType::new(String::from("PreLoader Error"), this.NO_DESCRIPTION.clone());
        this.PRODUCTSEARCHLOGGING = LogConfigType::new(String::from("Product Search Logging"), this.NO_DESCRIPTION.clone());
        this.PRODUCTSEARCHLOGGINGERROR = LogConfigType::new(String::from("Product Search Logging Error"), this.NO_DESCRIPTION.clone());
        this.SERVLET = LogConfigType::new(String::from("Servlet"), this.NO_DESCRIPTION.clone());
        this.SERVLETERROR = LogConfigType::new(String::from("Servlet Error"), this.NO_DESCRIPTION.clone());
        this.SHIPPINGERROR = LogConfigType::new(String::from("Shipping Error"), this.NO_DESCRIPTION.clone());
        this.SQLLOGGINGERROR = LogConfigType::new(String::from("SQL Error"), this.NO_DESCRIPTION.clone());
        this.SQLLOGGING = LogConfigType::new(String::from("SQL Logging"), this.NO_DESCRIPTION.clone());
        this.SQLTAGS = LogConfigType::new(String::from("SQL Tag Logging"), this.NO_DESCRIPTION.clone());
        this.SQLTAGSERROR = LogConfigType::new(String::from("SQL Tag Error"), this.NO_DESCRIPTION.clone());
        this.STYLE = LogConfigType::new(String::from("Style"), this.NO_DESCRIPTION.clone());
        this.STYLEERROR = LogConfigType::new(String::from("Style Error"), this.NO_DESCRIPTION.clone());
        this.TAGHELPER = LogConfigType::new(String::from("Tag Helper"), this.NO_DESCRIPTION.clone());
        this.TAGHELPERERROR = LogConfigType::new(String::from("Tag Helper Error"), this.NO_DESCRIPTION.clone());
        this.TAGHELPERFACTORY = LogConfigType::new(String::from("Tag Helper Factory"), this.NO_DESCRIPTION.clone());
        this.TAGHELPERFACTORYERROR = LogConfigType::new(String::from("Tag Helper Factory Error"), this.NO_DESCRIPTION.clone());
        this.TAX = LogConfigType::new(String::from("Tax"), this.NO_DESCRIPTION.clone());
        this.TAXERROR = LogConfigType::new(String::from("Tax Error"), this.NO_DESCRIPTION.clone());
        this.VALIDATION = LogConfigType::new(String::from("Validation"), this.NO_DESCRIPTION.clone());
        this.VALIDATIONERROR = LogConfigType::new(String::from("Validation Error"), this.NO_DESCRIPTION.clone());
        this.VIEW = LogConfigType::new(String::from("View"), this.NO_DESCRIPTION.clone());
        this.VIEWERROR = LogConfigType::new(String::from("View Error"), this.NO_DESCRIPTION.clone());
        this.VIEWFACTORYERROR = LogConfigType::new(String::from("View Factory Error"), this.NO_DESCRIPTION.clone());
        this.WORKFLOW = LogConfigType::new(String::from("Workflow"), this.NO_DESCRIPTION.clone());
        this.WORKFLOWERROR = LogConfigType::new(String::from("Workflow Error"), this.NO_DESCRIPTION.clone());
        this.XMLLOGGING = LogConfigType::new(String::from("XML Logging"), this.NO_DESCRIPTION.clone());
        this.XMLLOGGINGERROR = LogConfigType::new(String::from("XML Logging Error"), this.NO_DESCRIPTION.clone());
        this.XSLLOGGING = LogConfigType::new(String::from("XSL Logging"), this.NO_DESCRIPTION.clone());
        this.XSLLOGGINGERROR = LogConfigType::new(String::from("XSL Logging Error"), this.NO_DESCRIPTION.clone());
        this.REPLACE = LogConfigType::new(String::from("Replace"), this.NO_DESCRIPTION.clone());
        this.REPLACE_INFO = LogConfigType::new(String::from("Replace Info"), this.NO_DESCRIPTION.clone());
        this.REPLACEERROR = LogConfigType::new(String::from("Replace Error"), this.NO_DESCRIPTION.clone());
        this.SQLLOGGINGPOOL = LogConfigType::new(String::from("SQL Pool"), this.NO_DESCRIPTION.clone());
        this.SQLLOGGINGPOOLERROR = LogConfigType::new(String::from("SQL Pool Error"), this.NO_DESCRIPTION.clone());
        this.STATICPAGEGENERATIONLOGGING = LogConfigType::new(String::from("Static Page Generation Logging"), this.NO_DESCRIPTION.clone());
        this.STATICPAGEGENERATIONLOGGINGERROR = LogConfigType::new(String::from("Static Page Generation Logging Error"), this.NO_DESCRIPTION.clone());
        PreLogUtil::put(String::from("FreeBlisket"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(CommonStrings::getInstance()).CONSTRUCTOR.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

