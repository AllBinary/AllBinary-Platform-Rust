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
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystems::OperatingSystems;
use crate::org::allbinary::logic::system::hardware::linux::LinuxHardware::LinuxHardware;
use crate::org::allbinary::logic::system::hardware::windows::WindowsHardware::WindowsHardware;
use crate::java::lang::Exception::*;
use super::HardwareInterface::HardwareInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HardwareFactory {
}

pub static HardwareFactory_instance: crate::JavaStatic<Option<Rc<RefCell<HardwareFactory>>>> = crate::JavaStatic::new(|| HardwareFactory::new());

impl HardwareFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HardwareFactory>>> {
        return HardwareFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(mut os: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>>) -> Option<Rc<RefCell<dyn HardwareInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut operatingSystems: Option<Rc<RefCell<OperatingSystems>>> = OperatingSystems::getInstance();
            if (jcall!(os).as_GenericOperatingSystem_mut().getName().cmp(&{ let java_field = jcall!(operatingSystems).LINUX.clone(); java_field }.clone()) as i32) == 0 {
                return Some((LinuxHardware::new()).clone() /* as HardwareInterface */);
            } else if jcall!(os).as_GenericOperatingSystem_mut().getName().find(&{ let java_field = jcall!(operatingSystems).WINDOWS.clone(); java_field }.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                return Some((WindowsHardware::new()).clone() /* as HardwareInterface */);
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No Hardware Imp for: "), jcall!(os).as_GenericOperatingSystem_mut().getName()))));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

