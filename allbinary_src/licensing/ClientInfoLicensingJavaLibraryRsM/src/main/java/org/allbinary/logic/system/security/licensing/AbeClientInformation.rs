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
use crate::org::allbinary::business::init::LicenseInitInfo::LicenseInitInfo;
use crate::org::allbinary::business::init::LicenseInitInfoUtil::LicenseInitInfoUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ClientInformation::ClientInformation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

// This is the information sent to the license server
pub struct AbeClientInformation {
    pub base: ClientInformation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl AbeClientInformation {
    

    pub fn new_struct(mut name: String, mut version: String, mut specialName: String, mut shortName: String) -> Self {
        let mut this = Self {
            base: ClientInformation::new_struct(name.clone(), version.clone(), specialName.clone(), shortName.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String, mut version: String, mut specialName: String, mut shortName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, version, specialName, shortName))))
    }
    

    pub fn init(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = (jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance(),); self.setOperatingSystemInterface(java_arguments.0) };
            let mut licenseInitInfo: Option<Rc<RefCell<LicenseInitInfo>>> = jcall!(LicenseInitInfoUtil::getInstance()).read();
            { let java_arguments = (jcall!(licenseInitInfo).getLicenseId(),); self.setLicenseId(java_arguments.0) };
            { let java_arguments = (jcall!(licenseInitInfo).getServerList(),); self.setLicenseServers(java_arguments.0) };
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if jcall!(stringValidationUtil).isEmpty(self.getLicenseId()) {
                let mut NONE: String = String::from("No License Id");
                self.setLicenseId(NONE.clone());
            }
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Special Name: ")).append(self.getSpecialName()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for AbeClientInformation {
    type Target = ClientInformation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbeClientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AbeClientInformationDyn: crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn {
    fn as_AbeClientInformation(&self) -> &AbeClientInformation;
    fn as_AbeClientInformation_mut(&mut self) -> &mut AbeClientInformation;
}
impl AbeClientInformationDyn for AbeClientInformation {
    fn as_AbeClientInformation(&self) -> &AbeClientInformation { self }
    fn as_AbeClientInformation_mut(&mut self) -> &mut AbeClientInformation { self }
}

impl crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn for AbeClientInformation {
    fn as_ClientInformation(&self) -> &crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation { &self.base }
    fn as_ClientInformation_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation { &mut self.base }
    fn init(&mut self) { Self::init(self) }
}


