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
use super::*;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventListenerInterface::UserEmailEventListenerInterface;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use super::UserEmailEventConfigurationInterface::UserEmailEventConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventListenerFactory {
}


impl UserEmailEventListenerFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut userEmailEventConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>>, mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>) -> Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>> {
        let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (1) as usize];
        let mut classes: Vec<&'static str> = vec![""; (1) as usize];
        classes[(0) as usize] = std::any::type_name::<UserInterface>();
        params[(0) as usize] = crate::java_into_object((userInterface).clone() /* as Any */);
        return (jcall!(AbeFactory::getInstance()).getInstance4(abeClientInformation.clone(), jcall!(userEmailEventConfigurationInterface).getEventListenerClassPath(), classes.clone(), params.clone())).clone() /* as UserEmailEventListenerInterface */;
    }
}

