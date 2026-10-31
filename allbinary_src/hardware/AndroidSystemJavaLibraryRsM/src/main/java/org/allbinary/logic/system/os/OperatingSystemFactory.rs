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
use crate::org::allbinary::logic::system::os::android::AndroidOperatingSystemFactory::AndroidOperatingSystemFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::GenericOperatingSystem::GenericOperatingSystem;
use super::SystemProperties::SystemProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OperatingSystemFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
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
            genericOperatingSystem: None,
            hasDetected: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.genericOperatingSystem = NoOperatingSystem_NO_OPERATING_SYSTEM.with(Clone::clone).clone();
        this.hasDetected = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn getOperatingSystemInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut systemProperties: Option<Rc<RefCell<SystemProperties>>> = SystemProperties::getInstance();
            let mut osName: String = jcall!(systemProperties).getName();
            if !self.hasDetected {
                self.hasDetected = true;
                if osName.find(&jcall!(OperatingSystems::OperatingSystems::getInstance()).ANDROID.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                    jcall!(self.logUtil).putF(String::from("Found a Android OS"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone());
                    self.genericOperatingSystem = jcall!(AndroidOperatingSystemFactory::getInstance()).getOperatingSystemInstance();
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Operating System Info: ")).append(jcall!(self.genericOperatingSystem).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                } else if osName.find(&jcall!(OperatingSystems::OperatingSystems::getInstance()).HARMONY.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                    jcall!(self.logUtil).putF(String::from("Found a Harmony OS"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone());
                    self.genericOperatingSystem = jcall!(AndroidOperatingSystemFactory::getInstance()).getOperatingSystemInstance();
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Operating System Info: ")).append(jcall!(self.genericOperatingSystem).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("OS Not Supported: ")).append(osName.clone()).toString())));
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            self.genericOperatingSystem = NoOperatingSystem_NO_OPERATING_SYSTEM.with(Clone::clone).clone();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
        }
        return self.genericOperatingSystem.clone();
    }
}

