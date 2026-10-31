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
// package admin.tags
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::lang::reflect::Method::Method;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
// use crate::javax::servlet::jsp::tagext::TagSupport::TagSupport;
use crate::admin::taghelpers::AuthenticationHelperFactory::AuthenticationHelperFactory;
use crate::admin::taghelpers::AuthenticationRequestHelperFactory::AuthenticationRequestHelperFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::CustomTagSupport::CustomTagSupport;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AuthenticationTag {
    pub base: CustomTagSupport,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub command: String,
    pub userName: String,
    pub password: String,
    pub newPassword: String,
    pub roles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
}


impl AuthenticationTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomTagSupport::new_struct(),
            logUtil: None,
            command: String::new(),
            userName: String::new(),
            password: String::new(),
            newPassword: String::new(),
            roles: None,
            propertiesHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        this.userName = jcall!(stringUtil).EMPTY_STRING.clone();
        this.password = jcall!(stringUtil).EMPTY_STRING.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setCommand(&mut self, mut command: String) {
        self.command = command.clone();
    }
    

    pub fn setUserName(&mut self, mut userName: String) {
        self.userName = userName.clone();
    }
    

    pub fn setPassword(&mut self, mut value: String) {
        self.password = value.clone();
    }
    

    pub fn setNewPassword(&mut self, mut value: String) {
        self.newPassword = value.clone();
    }
    

    pub fn setRoles(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.roles = values.clone();
    }
    

    // throws LicensingException
    pub fn changePassword(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationRequestHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("changePassword"), None);
            let mut result: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return result.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn newPassword(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationRequestHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("newPassword"), None);
            let mut result: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return result.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn invalidateSession(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("invalidateSession"), None);
            let mut resultInteger: Option<i32> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Integer */;
            return ((resultInteger).unwrap() as i32);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn isSessionOld(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("isSessionOld"), None);
            let mut resultBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return resultBoolean.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn isRoleValid(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationRequestHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut methodParams: Vec<&'static str> = vec![std::any::type_name_of_val(&self.userName), std::any::type_name_of_val(&self.password), std::any::type_name_of_val(&self.roles)];
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("isRoleValid"), methodParams.clone());
            let mut methodArgs: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![self.userName.clone(), self.password.clone(), self.roles.clone()];
            let mut resultBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), methodArgs.clone())).clone() /* as Boolean */;
            return resultBoolean.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn validRole(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("validRole"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn invalidRole(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("invalidRole"), None);
            let mut result: String = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as String */;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn isAuthenticationSessionValid(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut methodParams: Vec<&'static str> = vec![std::any::type_name_of_val(&self.roles)];
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("isAuthenticationSessionValid"), methodParams.clone());
            let mut methodArgs: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![self.roles.clone()];
            let mut resultBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), methodArgs.clone())).clone() /* as Boolean */;
            return resultBoolean.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn processIfNewLogin(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut helperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut methodParams: Vec<&'static str> = vec![std::any::type_name_of_val(&self.userName), std::any::type_name_of_val(&self.password)];
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(helperClass).getMethod(String::from("processIfNewLogin"), methodParams.clone());
            let mut methodArgs: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![self.userName.clone(), self.password.clone()];
            let mut resultBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), methodArgs.clone())).clone() /* as Boolean */;
            return resultBoolean.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws LicensingException
    pub fn processInvalidation(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(AuthenticationHelperFactory::new()).getInstance(self.propertiesHashMap.clone(), self.pageContext.clone());
            let mut addressHelperClass: &'static str = std::any::type_name_of_val(&anyType);
            let mut method: Option<Rc<RefCell<Method>>> = jcall!(addressHelperClass).getMethod(String::from("processInvalidation"), None);
            let mut resultBoolean: Option<bool> = (jcall!(method).invoke(anyType.clone(), None)).clone() /* as Boolean */;
            return resultBoolean.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                let mut request: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(self.pageContext).getRequest()).clone() /* as HttpServletRequest */;
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from(" Request URI: "));
                jcall!(stringBuffer).append(jcall!(request).getRequestURI());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if true {
                self.propertiesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
                if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).NEWPASSWORD.clone()) as i32) == 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                        jcall!(self.logUtil).putF(String::from("New Password"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                    }
                    if !self.newPassword() {
                        jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("New Password Request Failed.<p/>"));
                        return EVAL_BODY_INCLUDE.clone();
                    } else {
                        jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("New Password Was Sent.<p/>"));
                        return SKIP_BODY.clone();
                    }
                } else if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).CHANGEPASSWORD.clone()) as i32) == 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                        jcall!(self.logUtil).putF(String::from("Change Password"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                    }
                    if !self.changePassword() {
                        jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("Password Change Attempt Failed.<p/>"));
                        return EVAL_BODY_INCLUDE.clone();
                    } else {
                        jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("Password Changed Successfully.<p/>"));
                        return SKIP_BODY.clone();
                    }
                } else if (self.command.cmp(&crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_INVALIDATESESSION.with(Clone::clone).clone()) as i32) == 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                        jcall!(self.logUtil).putF(String::from("INVALIDATESESSION"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                    }
                    return self.invalidateSession();
                } else {
                    if self.isSessionOld() {
                        if self.processInvalidation() {
                            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                                jcall!(self.logUtil).putF(String::from("Processing Invalidation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                            }
                            jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("Please login again.<p>"));
                            if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                                return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
                            } else {
                                return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                            }
                        } else {
                            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                                jcall!(self.logUtil).putF(String::from("Preparing session for invalidation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                            }
                            if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                                return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
                            } else {
                                return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                            }
                        }
                    }
                    if (false || (self.userName.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) == 0) && (false || (self.password.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) == 0) && self.roles.is_some() && self.isAuthenticationSessionValid() {
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                            jcall!(self.logUtil).putF(String::from("Authenticated session is valid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                        }
                        if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                            return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                        } else {
                            return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
                        }
                    }
                    if true && true && self.processIfNewLogin() {
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                            jcall!(self.logUtil).putF(String::from("Processing new login."), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                        }
                        if self.isRoleValid() {
                            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                                jcall!(self.logUtil).putF(String::from("Role is valid - Processing new login"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                            }
                            jcall!(jcall!(self.base.pageContext).getOut()).print(self.validRole());
                            if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                                return EVAL_BODY_INCLUDE.clone();
                            } else {
                                return SKIP_BODY.clone();
                            }
                        } else {
                            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                                jcall!(self.logUtil).putF(String::from("Role is not valid - Processing new login"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                            }
                            jcall!(jcall!(self.base.pageContext).getOut()).print(self.invalidRole());
                            if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                                return SKIP_BODY.clone();
                            } else {
                                return EVAL_BODY_INCLUDE.clone();
                            }
                        }
                    } else {
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                            jcall!(self.logUtil).putF(String::from("Invalid Login"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone());
                        }
                        if true && (self.userName.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) != 0 && true && (self.password.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) != 0 {
                            jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("Sorry your username and/or password is invalid.<p/>"));
                        }
                    }
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Major authentication error - userName: ")).append(self.userName.clone()).append(String::from(" Password: ")).append(self.password.clone()).append(String::from(" Command: ")).append(self.command.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                        return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
                    } else {
                        return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                    }
                }
            }
            return SKIP_BODY.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagLicensingRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            }
        }
    }
}
impl std::ops::Deref for AuthenticationTag {
    type Target = CustomTagSupport;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AuthenticationTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::CustomTagSupport::CustomTagSupportDyn for AuthenticationTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base }
}


