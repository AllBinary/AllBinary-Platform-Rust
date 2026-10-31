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
// package org.allbinary.logic.communication.http.request.session
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::http::HttpSession::HttpSession;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData;
use crate::org::allbinary::business::user::role::BasicUserRole::BasicUserRole;
use crate::org::allbinary::business::user::role::BasicUserRoleData::BasicUserRoleData;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicWeblisketSession {
    pub session: Option<Rc<RefCell<HttpSession>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
}


impl BasicWeblisketSession {
    

    pub fn new_struct(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            session: None,
            pageContext: None,
        };
        this.pageContext = pageContext.clone();
        let mut request: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        this.session = jcall!(request).getSession(true);
        this
    }

    pub fn new(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(pageContext))))
    }
    

    pub fn setRole(&mut self, mut aRole: Option<Rc<RefCell<BasicUserRole>>>) {
        { let java_receiver = self.session.clone(); let java_arguments = (crate::org::allbinary::business::user::role::BasicUserRoleData::BasicUserRoleData_NAME.with(Clone::clone).clone(), aRole.clone(),); jcall!(java_receiver).setAttribute(java_arguments.0, java_arguments.1); };
    }
    

    pub fn getRole(&mut self) -> Option<Rc<RefCell<BasicUserRole>>> {
        return ({ let java_receiver = self.session.clone(); let java_arguments = (crate::org::allbinary::business::user::role::BasicUserRoleData::BasicUserRoleData_NAME.with(Clone::clone).clone(),); let java_result = jcall!(java_receiver).getAttribute(java_arguments.0); java_result }).clone() /* as BasicUserRole */;
    }
    

    pub fn setPassword(&mut self, mut password: String) {
        jcall!(self.session).setAttribute(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone(), password.clone());
    }
    

    pub fn setUserName(&mut self, mut userName: String) {
        jcall!(self.session).setAttribute(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone());
    }
    

    pub fn setPaymentMethod(&mut self, mut value: String) {
        { let java_receiver = self.session.clone(); let java_arguments = (jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString(), value.clone(),); jcall!(java_receiver).setAttribute(java_arguments.0, java_arguments.1); };
    }
    

    pub fn setAuthenticated(&mut self) {
        { let java_receiver = self.session.clone(); let java_arguments = (WeblisketSessionData_AUTHENTICATED.with(Clone::clone).clone(), jcall!(BooleanFactory::getInstance()).TRUE_STRING.clone(),); jcall!(java_receiver).setAttribute(java_arguments.0, java_arguments.1); };
    }
    

    pub fn setAttempts(&mut self, mut value: Option<i32>) {
        jcall!(self.session).setAttribute(WeblisketSessionData_ATTEMPTS.with(Clone::clone).clone(), value.clone());
    }
    

    pub fn setTimeout(&mut self, mut value: String) {
        jcall!(self.session).setAttribute(WeblisketSessionData_TIMEOUT.with(Clone::clone).clone(), value.clone());
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        { let java_receiver = self.session.clone(); let java_arguments = (jcall!(StoreFrontData::getInstance()).NAME.clone(), value.clone(),); jcall!(java_receiver).setAttribute(java_arguments.0, java_arguments.1); };
    }
    

    pub fn getWebAppPath(&mut self) -> String {
        return (jcall!(self.session).getAttribute(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).WEBAPPPATH.clone())).clone() /* as String */;
    }
    

    pub fn getUserName(&mut self) -> String {
        return (jcall!(self.session).getAttribute(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone())).clone() /* as String */;
    }
    

    pub fn getPassword(&mut self) -> String {
        return (jcall!(self.session).getAttribute(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone())).clone() /* as String */;
    }
    

    pub fn getPaymentMethod(&mut self) -> String {
        return ({ let java_receiver = self.session.clone(); let java_arguments = (jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString(),); let java_result = jcall!(java_receiver).getAttribute(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getAuthentication(&mut self) -> String {
        return (jcall!(self.session).getAttribute(WeblisketSessionData_AUTHENTICATED.with(Clone::clone).clone())).clone() /* as String */;
    }
    

    pub fn getAttempts(&mut self) -> Option<i32> {
        return (jcall!(self.session).getAttribute(WeblisketSessionData_ATTEMPTS.with(Clone::clone).clone())).clone() /* as Integer */;
    }
    

    pub fn getTimeout(&mut self) -> String {
        return (jcall!(self.session).getAttribute(WeblisketSessionData_TIMEOUT.with(Clone::clone).clone())).clone() /* as String */;
    }
    

    pub fn getStoreName(&mut self) -> String {
        return ({ let java_receiver = self.session.clone(); let java_arguments = (jcall!(StoreFrontData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getAttribute(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn setAuthenticated1(&mut self, mut value: bool) {
        { let java_receiver = self.session.clone(); let java_arguments = (WeblisketSessionData_AUTHENTICATED.with(Clone::clone).clone(), jcall!(BooleanFactory::getInstance()).FALSE_STRING.clone(),); jcall!(java_receiver).setAttribute(java_arguments.0, java_arguments.1); };
    }
    

    pub fn clear(&mut self) {
        jcall!(self.session).removeAttribute(WeblisketSessionData_AUTHENTICATED.with(Clone::clone).clone());
        jcall!(self.session).removeAttribute(WeblisketSessionData_TIMEOUT.with(Clone::clone).clone());
        { let java_receiver = self.session.clone(); let java_arguments = (crate::org::allbinary::business::user::role::BasicUserRoleData::BasicUserRoleData_NAME.with(Clone::clone).clone(),); jcall!(java_receiver).removeAttribute(java_arguments.0); };
        jcall!(self.session).removeAttribute(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone());
    }
    

    pub fn getCreationTime(&mut self) -> i64 {
        return jcall!(self.session).getCreationTime();
    }
    

    pub fn getLastAccessedTime(&mut self) -> i64 {
        let mut lastAccessedTime: i64 = jcall!(self.session).getLastAccessedTime();
        return lastAccessedTime.clone();
    }
}

