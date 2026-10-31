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
// package org.allbinary.logic.communication.smtp.configuration.user.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::UserEmailEventConfigurationInterface::UserEmailEventConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub name: String,
    pub eventListenerClassPath: String,
}

impl UserEmailEventConfigurationInterface for UserEmailEventConfiguration {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn setName(&mut self, mut name: String) { Self::setName(self, name) }
    fn getEventListenerClassPath(&mut self) -> String { Self::getEventListenerClassPath(self) }
    fn setEventListenerClassPath(&mut self, mut eventListenerClassPath: String) { Self::setEventListenerClassPath(self, eventListenerClassPath) }
    fn log(&mut self) -> String { Self::log(self) }
}

impl UserEmailEventConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            name: String::new(),
            eventListenerClassPath: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        { let java_arguments = (jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); this.setName(java_arguments.0) };
        { let java_arguments = (jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); this.setEventListenerClassPath(java_arguments.0) };
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            jcall!(this.logUtil).putF(String::from("New"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from(" Constructor"));
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            name: String::new(),
            eventListenerClassPath: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        { let java_arguments = ((jcall!(hashMap).get(UserEmailEventConfigurationData_NAME.with(Clone::clone).clone())).clone() /* as String */,); this.setName(java_arguments.0) };
        { let java_arguments = ((jcall!(hashMap).get(UserEmailEventConfigurationData_LISTENER_CLASSPATH.with(Clone::clone).clone())).clone() /* as String */,); this.setEventListenerClassPath(java_arguments.0) };
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Log: "), this.log()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from(" Constructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn getEventListenerClassPath(&mut self) -> String {
        return self.eventListenerClassPath.clone();
    }
    

    pub fn setEventListenerClassPath(&mut self, mut eventListenerClassPath: String) {
        self.eventListenerClassPath = eventListenerClassPath.clone();
    }
    

    pub fn log(&mut self) -> String {
        return format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("\nUser Email Event Log Info:"), String::from("\nName: ")), self.getName()), String::from("\nEventListenerClassPath: ")), self.getEventListenerClassPath());
    }
}

