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
// package org.allbinary.logic.communication.smtp.configuration.server
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::EmailServerConfigurationInterface::EmailServerConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EmailServerConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub accountName: String,
    pub password: String,
    pub smtpServer: String,
}

impl EmailServerConfigurationInterface for EmailServerConfiguration {
    fn setSmtpServer(&mut self, mut value: String) { Self::setSmtpServer(self, value) }
    fn getSmtpServer(&mut self) -> String { Self::getSmtpServer(self) }
    fn getAccountName(&mut self) -> String { Self::getAccountName(self) }
    fn setAccountName(&mut self, mut accountName: String) { Self::setAccountName(self, accountName) }
    fn getPassword(&mut self) -> String { Self::getPassword(self) }
    fn setPassword(&mut self, mut password: String) { Self::setPassword(self, password) }
    fn log(&mut self) { Self::log(self) }
    fn toString(&mut self) -> String { Self::toString(self) }
}

impl EmailServerConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            accountName: String::new(),
            password: String::new(),
            smtpServer: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).EMAILLOGGING.clone(); java_field }.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF({ let java_field = jcall!(commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        }
        { let java_arguments = ({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(),); this.setSmtpServer(java_arguments.0) };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            accountName: String::new(),
            password: String::new(),
            smtpServer: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).EMAILLOGGING.clone(); java_field }.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF({ let java_field = jcall!(commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("EmailServerConfiguration(HashMap)"));
        }
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(EmailServerConfigurationData_ACCOUNT.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setAccountName(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(EmailServerConfigurationData_PASSWORD.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setPassword(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(EmailServerConfigurationData_SERVER.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setSmtpServer(java_arguments.0) };
        this.log();
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn new_struct3(mut account: String, mut password: String, mut server: String) -> Self {
        let mut this = Self {
            logUtil: None,
            accountName: String::new(),
            password: String::new(),
            smtpServer: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).EMAILLOGGING.clone(); java_field }.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF({ let java_field = jcall!(commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("EmailServerConfiguration(HashMap)"));
        }
        this.setAccountName(account.clone());
        this.setPassword(password.clone());
        this.setSmtpServer(server.clone());
        this.log();
        this
    }

    pub fn new3(mut account: String, mut password: String, mut server: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(account, password, server))))
    }
    

    pub fn setSmtpServer(&mut self, mut value: String) {
        self.smtpServer = value.clone();
    }
    

    pub fn getSmtpServer(&mut self) -> String {
        return self.smtpServer.clone();
    }
    

    pub fn getAccountName(&mut self) -> String {
        return self.accountName.clone();
    }
    

    pub fn setAccountName(&mut self, mut accountName: String) {
        self.accountName = accountName.clone();
    }
    

    pub fn getPassword(&mut self) -> String {
        return self.password.clone();
    }
    

    pub fn setPassword(&mut self, mut password: String) {
        self.password = password.clone();
    }
    

    pub fn log(&mut self) {
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).EMAILLOGGING.clone(); java_field }.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (self.toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("EmailServerConfiguration"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Email Server Configuration:\n"), String::from("Smtp Server: \n")), self.getSmtpServer()), String::from("\nAccount Name: ")), self.getAccountName()), String::from("\nPassword: ")), self.getPassword());
    }
}

