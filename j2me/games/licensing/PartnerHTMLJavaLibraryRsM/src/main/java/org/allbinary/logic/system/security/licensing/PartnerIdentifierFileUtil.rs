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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PartnerIdentifierFileUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub FILE_NAME: String,
}

thread_local! { pub static PartnerIdentifierFileUtil_instance: Option<Rc<RefCell<PartnerIdentifierFileUtil>>> = PartnerIdentifierFileUtil::new(); }

impl PartnerIdentifierFileUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            FILE_NAME: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.FILE_NAME = String::from("partner.txt");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PartnerIdentifierFileUtil>>> {
        return PartnerIdentifierFileUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn get(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
            let mut partnerString: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            return partnerString.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            return jcall!(StringUtil::getInstance()).NULL_STRING.clone();
            }
        }
    }
}

