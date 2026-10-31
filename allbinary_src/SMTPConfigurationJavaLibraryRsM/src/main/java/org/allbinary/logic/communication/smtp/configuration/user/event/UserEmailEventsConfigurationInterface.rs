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
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventListenerInterface::UserEmailEventListenerInterface;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use super::UserEmailEventConfigurationInterface::UserEmailEventConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait UserEmailEventsConfigurationInterface {
    

    fn getEventConfigurationHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>>;
    

    fn addUserEmailEventConfiguration(&mut self, userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>>);
    

    // throws Exception
    fn getEventListener(&mut self, abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, userInterface: Option<Rc<RefCell<dyn UserInterface>>>) -> Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


