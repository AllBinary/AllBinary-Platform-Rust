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
use crate::org::allbinary::business::context::configuration::ContextConfigurationInterface::ContextConfigurationInterface;
use crate::org::allbinary::business::user::modules::admin::configuration::AdminConfiguration::AdminConfiguration;
use crate::org::allbinary::business::user::modules::admin::configuration::AdminConfigurationInterface::AdminConfigurationInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::configuration::server::EmailServerConfigurationInterface::EmailServerConfigurationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::BasicEmailInfo::BasicEmailInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdminEmailInfo {
    pub base: BasicEmailInfo,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl AdminEmailInfo {
    

    pub fn new_struct(mut subject: String, mut textBody: String) -> Self {
        let mut this = Self {
            base: BasicEmailInfo::new_struct(subject.clone(), textBody.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        }
        this.init();
        this
    }

    pub fn new(mut subject: String, mut textBody: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(subject, textBody))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut adminConfigurationInterface: Option<Rc<RefCell<dyn AdminConfigurationInterface>>> = (AdminConfiguration::new()).clone() /* as AdminConfigurationInterface */;
        let mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>> = jcall!(adminConfigurationInterface).getContextConfigurationInterface();
        let mut emailServerConfigurationInterface: Option<Rc<RefCell<dyn EmailServerConfigurationInterface>>> = jcall!(contextConfigurationInterface).getEmailServerConfigurationInterface();
        self.setEmailServerConfigurationInterface(emailServerConfigurationInterface.clone());
    }
}
impl std::ops::Deref for AdminEmailInfo {
    type Target = BasicEmailInfo;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdminEmailInfo {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn for AdminEmailInfo {
    fn as_BasicEmailInfo(&self) -> &crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfo { &self.base }
    fn as_BasicEmailInfo_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfo { &mut self.base }
    fn init(&mut self) { Self::init(self) }
}


