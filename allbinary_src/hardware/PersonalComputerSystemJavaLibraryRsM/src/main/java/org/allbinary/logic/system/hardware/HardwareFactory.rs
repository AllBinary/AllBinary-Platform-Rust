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
use super::HardwareInterface::HardwareInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HardwareFactory {
}

thread_local! { pub static HardwareFactory_instance: Option<Rc<RefCell<HardwareFactory>>> = HardwareFactory::new(); }

impl HardwareFactory {
    

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
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut operatingSystems: Option<Rc<RefCell<OperatingSystems>>> = OperatingSystems::getInstance();
            if jcall!(jcall!(os).getName()).compareTo(jcall!(operatingSystems).LINUX.clone()) == 0 {
                return (LinuxHardware::new()).clone() /* as HardwareInterface */;
            } else if jcall!(jcall!(os).getName()).indexOf(jcall!(operatingSystems).WINDOWS.clone()) >= 0 {
                return (WindowsHardware::new()).clone() /* as HardwareInterface */;
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No Hardware Imp for: "), jcall!(os).getName()))));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
}

