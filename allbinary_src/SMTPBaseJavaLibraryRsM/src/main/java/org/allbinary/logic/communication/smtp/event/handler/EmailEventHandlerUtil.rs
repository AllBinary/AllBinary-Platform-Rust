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
// package org.allbinary.logic.communication.smtp.event.handler
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::modules::configuration::UserConfigurationInterface::UserConfigurationInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::configuration::user::UserEmailConfigurationInterface::UserEmailConfigurationInterface;
use crate::org::allbinary::logic::communication::smtp::configuration::user::event::UserEmailEventsConfigurationInterface::UserEmailEventsConfigurationInterface;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventListenerInterface::UserEmailEventListenerInterface;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::communication::smtp::event::modules::log::LogUserEmailEventListenerModule::LogUserEmailEventListenerModule;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use super::UserEmailEventHandler::UserEmailEventHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EmailEventHandlerUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static EmailEventHandlerUtil_instance: Option<Rc<RefCell<EmailEventHandlerUtil>>> = EmailEventHandlerUtil::new(); }

impl EmailEventHandlerUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<EmailEventHandlerUtil>>> {
        return EmailEventHandlerUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getEventHandler(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, mut userVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<UserEmailEventHandler>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Returning UserEmailEventHandler for "));
            jcall!(stringBuffer).appendint(jcall!(userVector).size());
            jcall!(stringBuffer).append(String::from(" users."));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getEventHandler"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut userEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = UserEmailEventHandler::new();
        jcall!(userEmailEventHandler).addListener(LogUserEmailEventListenerModule::new());
        let mut size: i32 = jcall!(userVector).size();
        for index in (0..size) {
            let mut userInterface: Option<Rc<RefCell<dyn UserInterface>>> = { let java_object = jcall!(userVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn UserInterface>>>>().expect("ClassCastException").clone()) };
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = EmailEventHandlerUtil::getUserEmailEventListenerVector(abeClientInformation.clone(), userEmailEventNameData.clone(), userInterface.clone());
            jcall!(userEmailEventHandler).addListener(vector.clone());
        }
        return userEmailEventHandler.clone();
    }
    

    // throws Exception
    pub fn getUserEmailEventListenerVector(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>> = jcall!(userInterface).getUserConfigurationInterface();
        let mut userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>> = jcall!(userConfigurationInterface).getUserEmailConfigurationInterface();
        let mut userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>> = jcall!(userEmailConfigurationInterface).getUserEmailEventsConfigurationInterface();
        let mut userEmailEventListenerInterface: Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>> = jcall!(userEmailEventsConfigurationInterface).getEventListener(abeClientInformation.clone(), userEmailEventNameData.clone(), userInterface.clone());
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(userEmailEventListenerInterface.clone()));
        return vector.clone();
    }
}

