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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::BasicEmailInfo::BasicEmailInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EmailInfo {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>>,
    pub htmlAttachment: String,
    pub contentBase: String,
}


impl EmailInfo {
    

    pub fn new_struct1(mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            basicEmailInfo: None,
            htmlAttachment: String::new(),
            contentBase: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        }
        this.init(basicEmailInfo.clone(), None, None);
        this
    }

    pub fn new1(mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(basicEmailInfo))))
    }
    

    pub fn new_struct3(mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>>, mut htmlAttachment: String, mut contentBase: String) -> Self {
        let mut this = Self {
            logUtil: None,
            basicEmailInfo: None,
            htmlAttachment: String::new(),
            contentBase: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        }
        this.init(basicEmailInfo.clone(), None, None);
        this
    }

    pub fn new3(mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>>, mut htmlAttachment: String, mut contentBase: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(basicEmailInfo, htmlAttachment, contentBase))))
    }
    

    pub fn init(&mut self, mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>>, mut htmlAttachment: String, mut contentBase: String) {
        self.basicEmailInfo = basicEmailInfo.clone();
        self.htmlAttachment = htmlAttachment.clone();
        self.contentBase = contentBase.clone();
    }
    

    pub fn getBasicEmailInfo(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> {
        return self.basicEmailInfo.clone();
    }
    

    pub fn getHtmlAttachment(&mut self) -> String {
        return self.htmlAttachment.clone();
    }
    

    pub fn getContentBase(&mut self) -> String {
        return self.contentBase.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", String::from("Email Info: \n "), jcall!(self.getBasicEmailInfo()).toString());
    }
}

