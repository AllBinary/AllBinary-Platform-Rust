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
// package org.allbinary.logic.communication.smtp.email
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData;
use crate::org::allbinary::logic::communication::smtp::event::handler::UserEmailEventHandler::UserEmailEventHandler;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::AdminUserEmailEventHandlerSingletons::AdminUserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::event::handler::factory::UserEmailEventHandlerSingletons::UserEmailEventHandlerSingletons;
use crate::org::allbinary::logic::communication::smtp::info::AdminEmailInfo::AdminEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::EmailInfo::EmailInfo;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NewPasswordEmail {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub userInterface: Option<Rc<RefCell<dyn UserInterface>>>,
    pub newPassword: String,
}


impl NewPasswordEmail {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>, mut newPassword: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abeClientInformation: None,
            userInterface: None,
            newPassword: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abeClientInformation = abeClientInformation.clone();
        this.userInterface = userInterface.clone();
        this.newPassword = newPassword.clone();
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut userInterface: Option<Rc<RefCell<dyn UserInterface>>>, mut newPassword: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, userInterface, newPassword))))
    }
    

    // throws Exception
    pub fn process(&mut self) {
        self.notifyStoreAdmin();
        self.notifyUser();
    }
    

    // throws Exception
    pub fn notifyStoreAdmin(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                jcall!(self.logUtil).putF(String::from("notifyStoreAdmin"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("notifyStoreAdmin"));
            }
            let mut emailSubject: String = format!("{}{}", String::from("New Password For User: "), jcall!(self.userInterface).getUserName());
            let mut emailBody: String = format!("{}{}", String::from("New Password: "), self.newPassword);
            let mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (AdminEmailInfo::new(emailSubject.clone(), emailBody.clone())).clone() /* as BasicEmailInfo */;
            let mut emailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(basicEmailInfo.clone());
            let mut adminUserEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(AdminUserEmailEventHandlerSingletons::getInstance()).getInstance2(self.abeClientInformation.clone(), crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_NEWPASSWORD.with(Clone::clone).clone());
            jcall!(adminUserEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_NEWPASSWORD.with(Clone::clone).clone(), emailInfo.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("emailAdmin"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn notifyUser(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                jcall!(self.logUtil).putF(String::from("Email User"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("notifyUser()"));
            }
            let mut subject: String = String::from("New Password");
            let mut body: String = format!("{}{}", String::from("New Password: "), self.newPassword);
            let mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = (AdminEmailInfo::new(subject.clone(), body.clone())).clone() /* as BasicEmailInfo */;
            let mut emailInfo: Option<Rc<RefCell<EmailInfo>>> = EmailInfo::new1(basicEmailInfo.clone());
            let mut userEmailEventHandler: Option<Rc<RefCell<UserEmailEventHandler>>> = jcall!(UserEmailEventHandlerSingletons::getInstance()).getInstance3(self.abeClientInformation.clone(), crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_NEWPASSWORD.with(Clone::clone).clone(), self.userInterface.clone());
            jcall!(userEmailEventHandler).receiveEmailInfo(crate::org::allbinary::logic::communication::smtp::event::UserEmailEventNameData::UserEmailEventNameData_NEWPASSWORD.with(Clone::clone).clone(), emailInfo.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("notifyUser"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

