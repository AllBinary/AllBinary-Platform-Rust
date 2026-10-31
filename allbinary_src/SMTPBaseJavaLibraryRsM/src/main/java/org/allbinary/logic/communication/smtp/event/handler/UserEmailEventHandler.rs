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
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::smtp::event::EmailEvent::EmailEvent;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventListenerInterface::UserEmailEventListenerInterface;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::communication::smtp::info::EmailInfo::EmailInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventHandler {
    pub emailVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub emailInfo: Option<Rc<RefCell<EmailInfo>>>,
    pub userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>,
}


impl UserEmailEventHandler {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            emailVector: None,
            emailInfo: None,
            userEmailEventNameData: None,
        };
        this.emailVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn receiveEmailInfo(&mut self, mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, mut emailInfo: Option<Rc<RefCell<EmailInfo>>>) {
        self.userEmailEventNameData = userEmailEventNameData.clone();
        self.emailInfo = emailInfo.clone();
        self.fireEmailEvent();
    }
    

    // synchronized
    pub fn addListener(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut userEmailEventListenerInterface: Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>> = { let java_object = jcall!(vector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>>>().expect("ClassCastException").clone()) };
            self.addListener(userEmailEventListenerInterface.clone());
        }
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn addListener(&mut self, mut emailEventListenerInterface: Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>>) {
        jcall!(self.emailVector).add(crate::java_into_object(emailEventListenerInterface.clone()));
    }
    

    // synchronized
    pub fn removeListener(&mut self, mut emailEventListenerInterface: Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>>) {
        jcall!(self.emailVector).remove(crate::java_into_object(emailEventListenerInterface.clone()));
    }
    

    // throws Exception
    // synchronized
    pub fn fireEmailEvent(&mut self) {
        let mut emailEvent: Option<Rc<RefCell<EmailEvent>>> = EmailEvent::new(None /* Java this: a &mut self method has no Rc of itself */, self.userEmailEventNameData.clone(), self.emailInfo.clone(), 0);
        let mut size: i32 = jcall!(self.emailVector).size();
        for index in (0..size) {
            let mut emailEventListenerInterface: Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>> = { let java_object = jcall!(self.emailVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn UserEmailEventListenerInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(emailEventListenerInterface).onEmailSendRequest(emailEvent.clone());
        }
    }
}

