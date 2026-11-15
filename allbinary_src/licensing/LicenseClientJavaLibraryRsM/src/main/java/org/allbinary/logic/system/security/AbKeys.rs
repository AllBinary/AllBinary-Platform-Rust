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
// package org.allbinary.logic.system.security
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::AbeLicenseInterfaceFactory::AbeLicenseInterfaceFactory;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbKeys {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

pub static AbKeys_instance: crate::JavaStatic<Option<Rc<RefCell<AbKeys>>>> = crate::JavaStatic::new(|| AbKeys::new());

impl AbKeys {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbKeys>>> {
        return AbKeys_instance.with(Clone::clone).clone();
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
    

    // throws LicensingException
    // synchronized
    pub fn getKey(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut keyName: String) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Getting Key: "), keyName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getKey"));
            if (keyName.cmp(&String::from("DirectX 8")) as i32) == 0 || (keyName.cmp(&String::from("DirectX")) as i32) == 0 || (keyName.cmp(&String::from("Low Level")) as i32) == 0 || (keyName.cmp(&String::from("System Drivers")) as i32) == 0 {
                return Some(String::from("Temp For Input Library"));
            }
            return Some(jcall!(jcall!(AbeLicenseInterfaceFactory::getInstance()).getLicenseInstance(abeClientInformation.clone())).getKey(keyName.clone()));
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

