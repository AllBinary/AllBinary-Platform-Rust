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
// package org.allbinary.logic.system.os.windows
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::NoOperatingSystem::NoOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystems::OperatingSystems;
use crate::org::allbinary::logic::system::os::SystemProperties::SystemProperties;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::WindowsOS::WindowsOS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WindowsOperatingSystemFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static WindowsOperatingSystemFactory_instance: Option<Rc<RefCell<WindowsOperatingSystemFactory>>> = WindowsOperatingSystemFactory::new(); }

impl WindowsOperatingSystemFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<WindowsOperatingSystemFactory>>> {
        return WindowsOperatingSystemFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getOperatingSystemInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut operatingSystems: Option<Rc<RefCell<OperatingSystems>>> = OperatingSystems::getInstance();
            let mut osName: String = jcall!(SystemProperties::getInstance()).getName();
            let mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = None;
            if (osName.cmp(&jcall!(operatingSystems).WINDOWS_NT.clone()) as i32) == 0 {
                operatingSystemInterface = WindowsOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
            } else if (osName.cmp(&jcall!(operatingSystems).WINDOWS_10.clone()) as i32) == 0 {
                operatingSystemInterface = WindowsOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
            } else if (osName.cmp(&jcall!(operatingSystems).WINDOWS_11.clone()) as i32) == 0 {
                operatingSystemInterface = WindowsOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
            } else if (osName.cmp(&jcall!(operatingSystems).WINDOWS2000.clone()) as i32) == 0 {
                operatingSystemInterface = WindowsOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
            } else if (osName.cmp(&jcall!(operatingSystems).WINDOWS_XP.clone()) as i32) == 0 {
                operatingSystemInterface = WindowsOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
            } else if (osName.cmp(&jcall!(operatingSystems).WINDOWS_VISTA.clone()) as i32) == 0 {
                operatingSystemInterface = WindowsOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
            } else {
                if jcall!(operatingSystems).isUnknownSpecificOSAllowed() {
                    operatingSystemInterface = WindowsOS::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>);
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Specific Windows OS Not Supported: "), osName))));
                }
            }
            return operatingSystemInterface.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).FACTORYERROR.clone())) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            return crate::org::allbinary::logic::system::os::NoOperatingSystem::NoOperatingSystem_NO_OPERATING_SYSTEM.with(Clone::clone).clone();
            }
        }
    }
}

