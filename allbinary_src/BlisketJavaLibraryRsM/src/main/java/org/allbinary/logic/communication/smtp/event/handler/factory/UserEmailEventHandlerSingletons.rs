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
// package org.allbinary.logic.communication.smtp.event.handler.factory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::communication::smtp::event::handler::EmailEventHandlerUtil::EmailEventHandlerUtil;
use crate::org::allbinary::logic::communication::smtp::event::handler::UserEmailEventHandler::UserEmailEventHandler;
use crate::org::allbinary::logic::communication::smtp::event::modules::log::LogUserEmailEventListenerModule::LogUserEmailEventListenerModule;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventHandlerSingletons {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub userEmailEventHandlerHashMap: Option<Rc<RefCell<HashMap>>>,
}

thread_local! { pub static UserEmailEventHandlerSingletons_instance: Option<Rc<RefCell<UserEmailEventHandlerSingletons>>> = UserEmailEventHandlerSingletons::new(); }

impl UserEmailEventHandlerSingletons {
    

    pub fn getInstance() -> Option<Rc<RefCell<UserEmailEventHandlerSingletons>>> {
        return UserEmailEventHandlerSingletons_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            userEmailEventHandlerHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.userEmailEventHandlerHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>) -> Option<Rc<RefCell<UserEmailEventHandler>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
        }
        let mut userEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = (jcall!(self.userEmailEventHandlerHashMap).get(userEmailEventNameData.clone())).clone() /* as UserEmailEventHandler */;
        if userEmailEventHandler.is_none() {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                jcall!(self.logUtil).putF(String::from("Creating New Named UserEmailEventHandler"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
            }
            let mut newUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = UserEmailEventHandler::new();
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = EmailEventHandlerUtil::getUserEmailEventListenerVector(abeClientInformation.clone(), userEmailEventNameData.clone(), userInterface.clone());
            jcall!(newUserEmailEventHandler).addListener(vector.clone());
            jcall!(newUserEmailEventHandler).addListener(LogUserEmailEventListenerModule::new());
            jcall!(self.userEmailEventHandlerHashMap).put(userEmailEventNameData.clone(), newUserEmailEventHandler.clone());
            return newUserEmailEventHandler.clone();
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                jcall!(self.logUtil).putF(String::from("Returning existing UserEmailEventHandler"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
            }
            return userEmailEventHandler.clone();
        }
    }
}

