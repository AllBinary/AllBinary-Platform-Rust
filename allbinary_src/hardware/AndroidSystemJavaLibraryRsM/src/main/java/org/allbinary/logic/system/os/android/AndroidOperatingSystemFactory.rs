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
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::NoOperatingSystem::NoOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystems::OperatingSystems;
use crate::org::allbinary::logic::system::os::SystemProperties::SystemProperties;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AndroidOS::AndroidOS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidOperatingSystemFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
}

thread_local! { pub static AndroidOperatingSystemFactory_instance: Option<Rc<RefCell<AndroidOperatingSystemFactory>>> = AndroidOperatingSystemFactory::new(); }

impl AndroidOperatingSystemFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AndroidOperatingSystemFactory>>> {
        return AndroidOperatingSystemFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            tsUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getOperatingSystemInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut GenericOperatingSystem: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = None;
            let mut systemProperties: Option<Rc<RefCell<SystemProperties>>> = SystemProperties::getInstance();
            let mut osName: String = jcall!(systemProperties).getName();
            if { let left = &(osName); let right = &(jcall!(OperatingSystems::getInstance()).ANDROID); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
                GenericOperatingSystem = AndroidOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
            } else {
                if jcall!(OperatingSystems::getInstance()).isUnknownSpecificOSAllowed() {
                    GenericOperatingSystem = AndroidOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Specific Android OS Not Supported: "), osName))));
                }
            }
            return GenericOperatingSystem.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(String::from("Failed to get OperatingSystem returning NoOperatingSystem"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            return crate::org::allbinary::logic::system::os::NoOperatingSystem::NoOperatingSystem_NO_OPERATING_SYSTEM.with(Clone::clone).clone();
            }
        }
    }
}

