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
// package org.allbinary.logic.system.security.licensing.registration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::canvas::SpecialMessageUtil::SpecialMessageUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::RegistrationConfiguration::RegistrationConfiguration;
use super::XmlRpcRemoteLicenseRegistrationClient::XmlRpcRemoteLicenseRegistrationClient;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseRegistrationUtil {
}


impl LicenseRegistrationUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn process(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut registrationId: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::put(format!("{}{}", jcall!(CommonLabels::getInstance()).START_LABEL, String::from("License Registration")), String::from("LicenseRegistrationUtil"), jcall!(commonStrings).PROCESS.clone());
            let mut registrationConfiguration: Option<Rc<RefCell<RegistrationConfiguration>>> = RegistrationConfiguration::getInstance();
            jcall!(registrationConfiguration).setRegistrationCode(registrationId.clone());
            jcall!(registrationConfiguration).write();
            let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(abeClientInformation).toHashtable();
            jcall!(hashtable).put(crate::java_into_object(jcall!(RegistrationConfiguration::getInstance()).NAME.clone()), crate::java_into_object(registrationId.clone()));
            jcall!(hashtable).put(crate::java_into_object(String::from("message")), crate::java_into_object(jcall!(SpecialMessageUtil::getInstance()).get()));
            jcall!(XmlRpcRemoteLicenseRegistrationClient::new(abeClientInformation.clone())).get(hashtable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(String::from("License Registration Exception"), String::from("LicenseRegistrationUtil"), String::from("License Registration"), e.clone());
            }
        }
    }
}

