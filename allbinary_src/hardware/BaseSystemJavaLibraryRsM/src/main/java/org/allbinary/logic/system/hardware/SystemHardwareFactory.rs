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
// package org.allbinary.logic.system.hardware
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::HardwareInterface::HardwareInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SystemHardwareFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>,
}

thread_local! { pub static SystemHardwareFactory_instance: Option<Rc<RefCell<SystemHardwareFactory>>> = SystemHardwareFactory::new(); }

impl SystemHardwareFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<SystemHardwareFactory>>> {
        return SystemHardwareFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            hardwareInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.hardwareInterface = NoHardware::NoHardware::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>>) -> Option<Rc<RefCell<dyn HardwareInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if match (&self.hardwareInterface, &NoHardware::NoHardware::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.hardwareInterface = jcall!(HardwareFactory::HardwareFactory::getInstance()).getInstance1(operatingSystemInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::os::OperatingSystemInterface::OperatingSystemInterface>>));
            }
            jcall!(self.logUtil).putF(String::from("Found Hardware"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        return self.hardwareInterface.clone();
    }
    

    pub fn static_toString() -> String {
        let mut systemHardwareFactory: Option<Rc<RefCell<SystemHardwareFactory>>> = SystemHardwareFactory::getInstance();
        let mut osBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(osBuffer).append(String::from("Hardware Info: \n"));
        if jcall!(systemHardwareFactory).hardwareInterface.is_some() {
            jcall!(osBuffer).append(jcall!(jcall!(systemHardwareFactory).hardwareInterface).toString());
        } else {
            jcall!(osBuffer).append(jcall!(StringUtil::getInstance()).NULL_STRING.clone());
        }
        return jcall!(osBuffer).toString();
    }
}

