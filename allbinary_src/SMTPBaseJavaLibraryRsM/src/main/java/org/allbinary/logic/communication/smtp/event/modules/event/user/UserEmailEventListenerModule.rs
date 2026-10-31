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
// package org.allbinary.logic.communication.smtp.event.modules.event.user
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::BasicEmail::BasicEmail;
use crate::org::allbinary::logic::communication::smtp::EmailInterface::EmailInterface;
use crate::org::allbinary::logic::communication::smtp::event::EmailEvent::EmailEvent;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventListenerInterface::UserEmailEventListenerInterface;
use crate::org::allbinary::logic::communication::smtp::queue::EmailQueueFactory::EmailQueueFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventListenerModule {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub userInterface: Option<Rc<RefCell<dyn UserInterface>>>,
}

// impl UserEmailEventListenerInterface for UserEmailEventListenerModule with the methods of the trait

impl UserEmailEventListenerModule {
    

    pub fn new_struct(mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            userInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.userInterface = userInterface.clone();
        this
    }

    pub fn new(mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(userInterface))))
    }
    

    // throws Exception
    pub fn onEmailSendFailure(&mut self, mut emailEvent: Option<Rc<RefCell<EmailEvent>>>) {
        let mut to: String = jcall!(self.userInterface).getSecondaryEmail();
        let mut email: Option<Rc<RefCell<BasicEmail>>> = BasicEmail::new(jcall!(emailEvent).getEmailInfo(), to.clone());
        jcall!(EmailQueueFactory::getInstance()).offer((jcall!(email).getEmail()).clone() /* as EmailInterface */);
    }
    

    // throws Exception
    pub fn onEmailSendRequest(&mut self, mut emailEvent: Option<Rc<RefCell<EmailEvent>>>) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Add Email To Que For Sending: "), jcall!(emailEvent).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onEmailSendRequest"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut to: String = jcall!(self.userInterface).getMainEmail();
        let mut email: Option<Rc<RefCell<BasicEmail>>> = BasicEmail::new(jcall!(emailEvent).getEmailInfo(), to.clone());
        jcall!(EmailQueueFactory::getInstance()).offer((jcall!(email).getEmail()).clone() /* as EmailInterface */);
    }
}

