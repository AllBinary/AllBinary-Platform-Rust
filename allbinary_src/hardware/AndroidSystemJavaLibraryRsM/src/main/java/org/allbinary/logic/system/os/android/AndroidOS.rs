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
// package org.allbinary.logic.system.os.android
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::AndroidInfoFactory::AndroidInfoFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::AndroidSystemProperties::AndroidSystemProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidOS {
    pub base: GenericOperatingSystem,
    pub stringBuffer: Option<Rc<RefCell<StringMaker>>>,
    pub DEVICE_ID: String,
    pub DEVICE_SOFTWARE_VERSION: String,
    pub LINE_1_NUMBER: String,
    pub OUYA: String,
}


impl AndroidOS {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GenericOperatingSystem::new_struct(),
            stringBuffer: None,
            DEVICE_ID: String::new(),
            DEVICE_SOFTWARE_VERSION: String::new(),
            LINE_1_NUMBER: String::new(),
            OUYA: String::new(),
        };
        this.stringBuffer = StringMaker::new();
        this.DEVICE_ID = String::from("DeviceId");
        this.DEVICE_SOFTWARE_VERSION = String::from("DeviceSoftwareVersion");
        this.LINE_1_NUMBER = String::from("Line1Number");
        this.OUYA = String::from("ouya");
        let mut SDK_VERSION: i32 = jcall!(AndroidInfoFactory::getInstance()).getVersion();
        if SDK_VERSION > 10 {
            this.scalable = true;
        }
        let mut properties: Option<Rc<RefCell<AndroidSystemProperties>>> = AndroidSystemProperties::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        jcall!(this.stringBuffer).append(this.DEVICE_ID.clone());
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getDeviceId(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(this.DEVICE_SOFTWARE_VERSION.clone());
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getDeviceSoftwareVersion(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(this.LINE_1_NUMBER.clone());
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getLine1Number(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("NetworkCountryIso"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getNetworkCountryIso(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("NetworkOperator"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getNetworkOperator(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("NetworkOperatorName"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getNetworkOperatorName(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("NetworkType"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getNetworkType(),); jcall!(java_receiver).appendint(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("PhoneType"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getPhoneType(),); jcall!(java_receiver).appendint(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("SimCountryIso"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getSimCountryIso(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("SimOperator"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getSimOperator(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("SimOperatorName"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getSimOperatorName(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("SimSerialNumber"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getSimSerialNumber(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("SubscriberId"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getSubscriberId(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("VoiceMailAlphaTag"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getVoiceMailAlphaTag(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        jcall!(this.stringBuffer).append(String::from("VoiceMailNumber"));
        jcall!(this.stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        { let java_receiver = this.stringBuffer.clone(); let java_arguments = (jcall!(properties).getVoiceMailNumber(),); jcall!(java_receiver).append(java_arguments.0); };
        jcall!(this.stringBuffer).append(jcall!(commonStrings).SPACE.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isOverScan(&mut self) -> bool {
        let mut properties: Option<Rc<RefCell<AndroidSystemProperties>>> = AndroidSystemProperties::getInstance();
        if jcall!(jcall!(properties).getDevice()).toLowerCase().find(&self.OUYA.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return true;
        }
        return false;
    }
    

    pub fn getOverScanXPercent(&mut self) -> i32 {
        return 90;
    }
    

    pub fn getOverScanYPercent(&mut self) -> i32 {
        return 90;
    }
    

    pub fn toString(&mut self) -> String {
        let mut osBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(osBuffer).append(self.base.toString());
        jcall!(osBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(osBuffer).append(String::from("Other System Info: \n"));
        jcall!(osBuffer).append(jcall!(self.stringBuffer).toString());
        return jcall!(osBuffer).toString();
    }
}
impl std::ops::Deref for AndroidOS {
    type Target = GenericOperatingSystem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidOS {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn for AndroidOS {
    fn as_GenericOperatingSystem(&self) -> &crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem { &self.base }
    fn as_GenericOperatingSystem_mut(&mut self) -> &mut crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem { &mut self.base }
}
impl crate::org::allbinary::logic::system::os::OperatingSystemInterface::OperatingSystemInterface for AndroidOS {
    fn getName(&mut self) -> String { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getName(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn getVersion(&mut self) -> String { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getVersion(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn getArch(&mut self) -> String { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getArch(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn getCharacterSet(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::logic::java::character::CharacterSet::CharacterSet>>> { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getCharacterSet(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn isOverScan(&mut self) -> bool { Self::isOverScan(self) }
    fn getOverScanXPercent(&mut self) -> i32 { Self::getOverScanXPercent(self) }
    fn getOverScanYPercent(&mut self) -> i32 { Self::getOverScanYPercent(self) }
    fn isScalable(&mut self) -> bool { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::isScalable(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn isAutoHide(&mut self) -> bool { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::isAutoHide(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn toString(&mut self) -> String { Self::toString(self) }
}


