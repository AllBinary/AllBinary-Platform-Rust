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
// package org.allbinary.logic.communication.smtp.configuration.user
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::configuration::user::event::UserEmailEventsConfiguration::UserEmailEventsConfiguration;
use crate::org::allbinary::logic::communication::smtp::configuration::user::event::UserEmailEventsConfigurationInterface::UserEmailEventsConfigurationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::UserEmailConfigurationInterface::UserEmailConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>>,
}

impl UserEmailConfigurationInterface for UserEmailConfiguration {
    fn getUserEmailEventsConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>> { Self::getUserEmailEventsConfigurationInterface(self) }
    fn setUserEmailEventsConfigurationInterface(&mut self, mut userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>>) { Self::setUserEmailEventsConfigurationInterface(self, userEmailEventsConfigurationInterface) }
}

impl UserEmailConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            userEmailEventsConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this.setUserEmailEventsConfigurationInterface(UserEmailEventsConfiguration::new());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            userEmailEventsConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this.setUserEmailEventsConfigurationInterface(UserEmailEventsConfiguration::new1(hashMap.clone()));
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn getUserEmailEventsConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>> {
        return self.userEmailEventsConfigurationInterface.clone();
    }
    

    pub fn setUserEmailEventsConfigurationInterface(&mut self, mut userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>>) {
        self.userEmailEventsConfigurationInterface = userEmailEventsConfigurationInterface.clone();
    }
}

