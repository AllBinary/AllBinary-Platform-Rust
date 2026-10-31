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
// package org.allbinary.business.context.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::configuration::server::EmailServerConfiguration::EmailServerConfiguration;
use crate::org::allbinary::logic::communication::smtp::configuration::server::EmailServerConfigurationInterface::EmailServerConfigurationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ContextConfigurationInterface::ContextConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ContextConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>,
}

impl ContextConfigurationInterface for ContextConfiguration {
    fn getEmailServerConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>> { Self::getEmailServerConfigurationInterface(self) }
    fn setEmailServerConfigurationInterface(&mut self, mut emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>) { Self::setEmailServerConfigurationInterface(self, emailServerConfigurationInterface) }
}

impl ContextConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            emailServerConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("ContextConfiguration"));
        this.setEmailServerConfigurationInterface((EmailServerConfiguration::new()).clone() /* as EmailServerConfigurationInterface */);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            emailServerConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("ContextConfiguration(HashMap)"));
        this.setEmailServerConfigurationInterface((EmailServerConfiguration::new1(hashMap.clone())).clone() /* as EmailServerConfigurationInterface */);
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn getEmailServerConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>> {
        return self.emailServerConfigurationInterface.clone();
    }
    

    pub fn setEmailServerConfigurationInterface(&mut self, mut emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>) {
        self.emailServerConfigurationInterface = emailServerConfigurationInterface.clone();
    }
}

