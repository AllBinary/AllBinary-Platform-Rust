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
// package org.allbinary.logic.communication.smtp.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::EventObject::EventObject;
use crate::org::allbinary::logic::communication::smtp::info::EmailInfo::EmailInfo;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::UserEmailEventNameData::UserEmailEventNameData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EmailEvent {
    pub base: EventObject,
    pub emailInfo: Option<Rc<RefCell<EmailInfo>>>,
    pub userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>,
    pub attempts: i32,
}


impl EmailEvent {
    

    pub fn new_struct(mut emailEventHandler: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, mut emailInfo: Option<Rc<RefCell<EmailInfo>>>, mut attempts: i32) -> Self {
        let mut this = Self {
            base: EventObject::new_struct1(emailEventHandler.clone()),
            emailInfo: None,
            userEmailEventNameData: None,
            attempts: 0,
        };
        // super(...) is in the struct literal above
        this.userEmailEventNameData = userEmailEventNameData.clone();
        this.emailInfo = emailInfo.clone();
        this
    }

    pub fn new(mut emailEventHandler: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>>, mut emailInfo: Option<Rc<RefCell<EmailInfo>>>, mut attempts: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(emailEventHandler, userEmailEventNameData, emailInfo, attempts))))
    }
    

    pub fn getEmailInfo(&mut self) -> Option<Rc<RefCell<EmailInfo>>> {
        return self.emailInfo.clone();
    }
    

    pub fn getAttempts(&mut self) -> i32 {
        return self.attempts.clone();
    }
    

    pub fn setAttempts(&mut self, mut attempts: i32) {
        self.attempts = attempts.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("EmailEvent Log: \n"));
        jcall!(stringBuffer).append(String::from("Event Name: "));
        jcall!(stringBuffer).append(jcall!(self.userEmailEventNameData).toString());
        jcall!(stringBuffer).append(jcall!(self.getEmailInfo()).toString());
        jcall!(stringBuffer).append(String::from("\nNumber of Attempts: "));
        jcall!(stringBuffer).appendint(self.getAttempts());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for EmailEvent {
    type Target = EventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EmailEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



