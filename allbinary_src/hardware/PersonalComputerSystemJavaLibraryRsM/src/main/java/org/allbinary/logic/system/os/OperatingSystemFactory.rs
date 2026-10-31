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
// package org.allbinary.logic.system.os
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::Log::Log;
use crate::org::allbinary::logic::communication::log::LogFactory::LogFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::os::linux::LinuxOperatingSystemFactory::LinuxOperatingSystemFactory;
use crate::org::allbinary::logic::system::os::solaris::SolarisOperatingSystemFactory::SolarisOperatingSystemFactory;
use crate::org::allbinary::logic::system::os::windows::WindowsOperatingSystemFactory::WindowsOperatingSystemFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::GenericOperatingSystem::GenericOperatingSystem;
use super::OperatingSystems::OperatingSystems;
use super::SystemProperties::SystemProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OperatingSystemFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub genericOperatingSystem: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>>,
    pub hasDetected: bool,
}

thread_local! { pub static OperatingSystemFactory_instance: Option<Rc<RefCell<OperatingSystemFactory>>> = OperatingSystemFactory::new(); }

impl OperatingSystemFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<OperatingSystemFactory>>> {
        return OperatingSystemFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            genericOperatingSystem: None,
            hasDetected: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.genericOperatingSystem = NoOperatingSystem_NO_OPERATING_SYSTEM.with(Clone::clone).clone();
        this.hasDetected = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn getOperatingSystemInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut operatingSystems: Option<Rc<RefCell<OperatingSystems>>> = OperatingSystems::getInstance();
            let mut systemProperties: Option<Rc<RefCell<SystemProperties>>> = SystemProperties::getInstance();
            let mut osName: String = jcall!(systemProperties).getName();
            let mut osArch: String = jcall!(systemProperties).getArch();
            let mut osVersion: String = jcall!(systemProperties).getVersion();
            if !self.hasDetected {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("osName: "), osName), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
                self.hasDetected = true;
                if osName.find(&jcall!(operatingSystems).LINUX.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                    if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).FACTORYERROR.clone())) {
                        jcall!(self.logUtil).putF(String::from("Found a Linux OS"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
                    }
                    self.genericOperatingSystem = jcall!(LinuxOperatingSystemFactory::getInstance()).getOperatingSystemInstance();
                } else if osName.find(&jcall!(operatingSystems).WINDOWS.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                    if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).FACTORYERROR.clone())) {
                        jcall!(self.logUtil).putF(String::from("Found a Windows OS"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
                    }
                    self.genericOperatingSystem = jcall!(WindowsOperatingSystemFactory::getInstance()).getOperatingSystemInstance();
                } else if osName.find(&jcall!(operatingSystems).SOLARIS.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                    if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).FACTORYERROR.clone())) {
                        jcall!(self.logUtil).putF(String::from("Found a Solaris OS"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
                    }
                    self.genericOperatingSystem = jcall!(SolarisOperatingSystemFactory::getInstance()).getOperatingSystemInstance();
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("OS Not Supported: ")).append(osName.clone()).toString())));
                }
                let mut log: Option<Rc<RefCell<Log>>> = LogFactory::getInstanceF(jcall!(StringMaker::new()).append(String::from("OperatingSystem Info: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.genericOperatingSystem.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
                println!("{}", jcall!(log).toString());
                jcall!(self.logUtil).putL(log.clone());
            }
            return self.genericOperatingSystem.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).FACTORYERROR.clone())) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            return NoOperatingSystem_NO_OPERATING_SYSTEM.with(Clone::clone).clone();
            }
        }
    }
}

