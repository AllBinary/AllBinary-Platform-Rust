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
// package org.allbinary.logic.communication.smtp.event.modules.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::event::EmailEvent::EmailEvent;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventListenerInterface::UserEmailEventListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogUserEmailEventListenerModule {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

// impl UserEmailEventListenerInterface for LogUserEmailEventListenerModule with the methods of the trait

impl LogUserEmailEventListenerModule {
    

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
    pub fn onEmailSendRequest(&mut self, mut userEmailInfoEvent: Option<Rc<RefCell<EmailEvent>>>) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            let mut message: String = format!("{}{}", String::from("Logging EmailEvent - Temporary Listener for debugging.\n\n"), jcall!(userEmailInfoEvent).toString());
            jcall!(self.logUtil).putF(message.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onEmailSendRequest"));
        }
    }
}

