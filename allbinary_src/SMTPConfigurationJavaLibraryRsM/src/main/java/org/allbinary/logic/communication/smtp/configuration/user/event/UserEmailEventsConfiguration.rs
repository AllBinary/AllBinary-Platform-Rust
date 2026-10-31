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
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventListenerInterface::UserEmailEventListenerInterface;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::UserEmailEventsConfigurationInterface::UserEmailEventsConfigurationInterface;
use super::UserEmailEventConfigurationInterface::UserEmailEventConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventsConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub emailEventHashMap: Option<Rc<RefCell<HashMap>>>,
}

impl UserEmailEventsConfigurationInterface for UserEmailEventsConfiguration {
    fn getEventConfigurationHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::getEventConfigurationHashMap(self) }
    fn addUserEmailEventConfiguration(&mut self, mut userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>>) { Self::addUserEmailEventConfiguration(self, userEmailEventConfigurationInterface) }
    fn getEventListener(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>) -> Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>> { Self::getEventListener(self, abeClientInformation, userEmailEventNameData, userInterface) }
}

impl UserEmailEventsConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            emailEventHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.init();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            emailEventHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.init();
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn init(&mut self) {
        self.emailEventHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from(" Constructor"));
        }
    }
    

    pub fn getEventConfigurationHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.emailEventHashMap.clone();
    }
    

    pub fn addUserEmailEventConfiguration(&mut self, mut userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>>) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Adding: "), jcall!(userEmailEventConfigurationInterface).log()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addUserEmailEventConfiguration"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        { let java_receiver = self.emailEventHashMap.clone(); let java_arguments = (jcall!(userEmailEventConfigurationInterface).getName(), userEmailEventConfigurationInterface.clone(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
    }
    

    // throws Exception
    pub fn getEventListener(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>) -> Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>> {
        let mut userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>> = ({ let java_receiver = self.emailEventHashMap.clone(); let java_arguments = (jcall!(userEmailEventNameData).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as UserEmailEventConfigurationInterface */;
        return UserEmailEventListenerFactory::UserEmailEventListenerFactory::getInstance(abeClientInformation.clone(), userEmailEventConfigurationInterface.clone(), userInterface.clone());
    }
}

