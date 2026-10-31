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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::http::HttpSession::HttpSession;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::commerce::inventory::basket::Basket::Basket;
use crate::org::allbinary::business::user::commerce::inventory::order::Order::Order;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::business::user::role::UserRoleB::UserRoleB;
use crate::org::allbinary::business::user::role::UserRoleData::UserRoleData;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use super::WeblisketSessionInterface::WeblisketSessionInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WeblisketSession {
    pub session: Option<Rc<RefCell<HttpSession>>>,
}

impl WeblisketSessionInterface for WeblisketSession {
    fn clear(&mut self) { Self::clear(self) }
    fn getAttempts(&mut self) -> Option<i32> { Self::getAttempts(self) }
    fn getAuthentication(&mut self) -> String { Self::getAuthentication(self) }
    fn getCreationTime(&mut self) -> i64 { Self::getCreationTime(self) }
    fn getId(&mut self) -> String { Self::getId(self) }
    fn getLastAccessedTime(&mut self) -> i64 { Self::getLastAccessedTime(self) }
    fn getOrder(&mut self) -> Option<Rc<RefCell<dyn OrderInterface>>> { Self::getOrder(self) }
    fn getPassword(&mut self) -> String { Self::getPassword(self) }
    fn getPaymentMethod(&mut self) -> String { Self::getPaymentMethod(self) }
    fn getRole(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> { Self::getRole(self) }
    fn getStoreName(&mut self) -> String { Self::getStoreName(self) }
    fn getTimeout(&mut self) -> String { Self::getTimeout(self) }
    fn getUserName(&mut self) -> String { Self::getUserName(self) }
    fn getWebAppPath(&mut self) -> String { Self::getWebAppPath(self) }
    fn removeBasket(&mut self) { Self::removeBasket(self) }
    fn setAttempts(&mut self, mut value: Option<i32>) { Self::setAttempts(self, value) }
    fn setAuthenticated(&mut self) { Self::setAuthenticated(self) }
    fn setAuthenticated1(&mut self, mut value: bool) { Self::setAuthenticated1(self, value) }
    fn setPassword(&mut self, mut password: String) { Self::setPassword(self, password) }
    fn setPaymentMethod(&mut self, mut value: String) { Self::setPaymentMethod(self, value) }
    fn setRole(&mut self, mut aRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) { Self::setRole(self, aRole) }
    fn setStoreName(&mut self, mut value: String) { Self::setStoreName(self, value) }
    fn setTimeout(&mut self, mut value: String) { Self::setTimeout(self, value) }
    fn setUserName(&mut self, mut userName: String) { Self::setUserName(self, userName) }
}

impl WeblisketSession {
    

    pub fn new_struct2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self::new_struct2(hashMap.clone(), (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */);
        // this(...) is in the struct literal above
        this
    }

    pub fn new2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(hashMap, pageContext))))
    }
    

    pub fn new_struct2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            session: None,
        };
        let mut propertiesHashMap: Option<Rc<RefCell<HashMap>>> = hashMap.clone();
        let mut request: Option<Rc<RefCell<HttpServletRequest>>> = httpServletRequest.clone();
        this.session = jcall!(request).getSession(true);
        if this.getStoreName().is_none() {
            { let java_arguments = ((jcall!(propertiesHashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */,); this.setStoreName(java_arguments.0) };
        }
        this
    }

    pub fn new2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(hashMap, httpServletRequest))))
    }
    

    pub fn getId(&mut self) -> String {
        return jcall!(self.session).getId();
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
    

    pub fn setRole(&mut self, mut aRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) {
        { let java_receiver = self.session.clone(); let java_arguments = (crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone(), aRole.clone(),); jcall!(java_receiver).setAttribute(java_arguments.0, java_arguments.1); };
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
    

    // throws Exception
    pub fn getRole(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> {
        let mut userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> = ({ let java_receiver = self.session.clone(); let java_arguments = (crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone(),); let java_result = jcall!(java_receiver).getAttribute(java_arguments.0); java_result }).clone() /* as UserRole */;
        if userRole.is_some() {
            return UserRoleB::getRole(jcall!(jcall!(userRole).getBasicUserRole()).getRole());
        } else {
            return None;
        }
    }
    

    // throws Exception
    pub fn getOrder(&mut self) -> Option<Rc<RefCell<dyn OrderInterface>>> {
        let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(self.getStoreName());
        let mut basketName: String = jcall!(storeFrontInterface).getBasketName();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(basketName.clone()) {
            let mut orderInterface: Option<Rc<RefCell<dyn OrderInterface>>> = (jcall!(self.session).getAttribute(basketName.clone())).clone() /* as OrderInterface */;
            if orderInterface.is_none() {
                orderInterface = (Order::new1(Basket::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn>>)).clone() /* as OrderInterface */;
            }
            jcall!(self.session).setAttribute(basketName.clone(), orderInterface.clone());
            return orderInterface.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Basket Name Not Found"))));
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn setAuthenticated1(&mut self, mut value: bool) {
        { let java_receiver = self.session.clone(); let java_arguments = (WeblisketSessionData_AUTHENTICATED.with(Clone::clone).clone(), jcall!(BooleanFactory::getInstance()).FALSE_STRING.clone(),); jcall!(java_receiver).setAttribute(java_arguments.0, java_arguments.1); };
    }
    

    pub fn clear(&mut self) {
        jcall!(self.session).removeAttribute(WeblisketSessionData_AUTHENTICATED.with(Clone::clone).clone());
        jcall!(self.session).removeAttribute(WeblisketSessionData_TIMEOUT.with(Clone::clone).clone());
        { let java_receiver = self.session.clone(); let java_arguments = (crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone(),); jcall!(java_receiver).removeAttribute(java_arguments.0); };
        jcall!(self.session).removeAttribute(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone());
    }
    

    pub fn getCreationTime(&mut self) -> i64 {
        return jcall!(self.session).getCreationTime();
    }
    

    pub fn getLastAccessedTime(&mut self) -> i64 {
        return jcall!(self.session).getLastAccessedTime();
    }
    

    pub fn removeBasket(&mut self) {
        let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(self.getStoreName());
        let mut basketName: String = jcall!(storeFrontInterface).getBasketName();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(basketName.clone()) {
            jcall!(self.session).removeAttribute(basketName.clone());
        }
    }
}

