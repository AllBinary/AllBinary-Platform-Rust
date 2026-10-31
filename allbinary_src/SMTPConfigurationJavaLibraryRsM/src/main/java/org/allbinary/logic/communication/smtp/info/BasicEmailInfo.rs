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
// package org.allbinary.logic.communication.smtp.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::configuration::server::EmailServerConfigurationInterface::EmailServerConfigurationInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicEmailInfo {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub subject: String,
    pub textBody: String,
    pub emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>,
}


impl BasicEmailInfo {
    

    pub fn new_struct(mut subject: String, mut textBody: String) -> Self {
        let mut this = Self {
            logUtil: None,
            subject: String::new(),
            textBody: String::new(),
            emailServerConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        }
        this.subject = subject.clone();
        this.textBody = textBody.clone();
        this
    }

    pub fn new(mut subject: String, mut textBody: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(subject, textBody))))
    }
    

    pub fn getSubject(&mut self) -> String {
        return self.subject.clone();
    }
    

    pub fn getTextBody(&mut self) -> String {
        return self.textBody.clone();
    }
    

    pub fn getEmailServerConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>> {
        return self.emailServerConfigurationInterface.clone();
    }
    

    pub fn setEmailServerConfigurationInterface(&mut self, mut emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>) {
        self.emailServerConfigurationInterface = emailServerConfigurationInterface.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.getEmailServerConfigurationInterface()).toString());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from("Subject: \n"));
        jcall!(stringBuffer).append(self.getSubject());
        jcall!(stringBuffer).append(String::from("\nText Body: \n"));
        jcall!(stringBuffer).append(self.getTextBody());
        return jcall!(stringBuffer).toString();
    }
    

    // throws Exception
    pub fn init(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait BasicEmailInfoDyn {
    fn as_BasicEmailInfo(&self) -> &BasicEmailInfo;
    fn as_BasicEmailInfo_mut(&mut self) -> &mut BasicEmailInfo;
    fn getSubject(&mut self) -> String { BasicEmailInfo::getSubject(self.as_BasicEmailInfo_mut()) }
    fn getTextBody(&mut self) -> String { BasicEmailInfo::getTextBody(self.as_BasicEmailInfo_mut()) }
    fn getEmailServerConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>> { BasicEmailInfo::getEmailServerConfigurationInterface(self.as_BasicEmailInfo_mut()) }
    fn setEmailServerConfigurationInterface(&mut self, mut emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>>) { BasicEmailInfo::setEmailServerConfigurationInterface(self.as_BasicEmailInfo_mut(), emailServerConfigurationInterface) }
    fn toString(&mut self) -> String { BasicEmailInfo::toString(self.as_BasicEmailInfo_mut()) }
    fn init(&mut self) { BasicEmailInfo::init(self.as_BasicEmailInfo_mut()) }
}
impl BasicEmailInfoDyn for BasicEmailInfo {
    fn as_BasicEmailInfo(&self) -> &BasicEmailInfo { self }
    fn as_BasicEmailInfo_mut(&mut self) -> &mut BasicEmailInfo { self }
}


