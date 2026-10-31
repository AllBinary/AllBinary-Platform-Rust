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
use crate::Calendar;
use super::*;
// use crate::java::io::IOException::IOException;
// use crate::java::util::Calendar::Calendar;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::client::AbeLicenseClient::AbeLicenseClient;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AbeLicenseInterface::AbeLicenseInterface;
use super::AbeClientInformationInterface::AbeClientInformationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbeLicenseInterfaceFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub time: i64,
    pub check: bool,
    pub checkPeriod: i32,
    pub abeLicenseInterface: Option<Rc<RefCell<dyn AbeLicenseInterface>>>,
}

thread_local! { pub static AbeLicenseInterfaceFactory_SINGLETON: Option<Rc<RefCell<AbeLicenseInterfaceFactory>>> = AbeLicenseInterfaceFactory::new(); }

impl AbeLicenseInterfaceFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AbeLicenseInterfaceFactory>>> {
        return AbeLicenseInterfaceFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            time: 0_i64,
            check: false,
            checkPeriod: 0,
            abeLicenseInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.time = 0_i64;
        this.check = false;
        this.checkPeriod = 36000000;
        this.abeLicenseInterface = None;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException
    pub fn getLicenseInstance(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Option<Rc<RefCell<dyn AbeLicenseInterface>>> {
        if self.isTimeToGetKey() {
            return self.get(abeClientInformation.clone());
        } else {
            return self.abeLicenseInterface.clone();
        }
    }
    

    // throws LicensingException
    pub fn get(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Option<Rc<RefCell<dyn AbeLicenseInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Getting Keys"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone());
            self.abeLicenseInterface = AbeNoLicense::AbeNoLicense::getInstance().map(|reference| reference as Rc<RefCell<dyn AbeLicenseInterface>>);
            let mut licenseClient: Option<Rc<RefCell<AbeLicenseClient>>> = AbeLicenseClient::new();
            self.abeLicenseInterface = jcall!(licenseClient).get(abeClientInformation.clone());
            if self.abeLicenseInterface.is_some() {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Default Key: "), { let java_receiver = self.abeLicenseInterface.clone(); let java_arguments = (jcall!(AbeClientInformationData::AbeClientInformationData::getInstance()).KEY.clone(),); let java_result = jcall!(java_receiver).getKey(java_arguments.0); java_result }), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return self.abeLicenseInterface.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Licensing IO Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", format!("{}: {:?}", "LicensingException", (String::from("License Server Connection Error"))));
            }
        }
    }
    

    pub fn isTimeToGetKey(&mut self) -> bool {
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut currentTime: i64 = jcall!(calendar).getTimeInMillis();
        if self.abeLicenseInterface.is_none() || match (&self.abeLicenseInterface, &AbeNoLicense::AbeNoLicense::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || !jcall!(self.abeLicenseInterface).hasKey() || self.isCheck() || currentTime - ((self.checkPeriod) as i64) > self.time {
            self.abeLicenseInterface = None;
            self.time = currentTime.clone();
            return true;
        } else {
            return false;
        }
    }
    

    pub fn setCheck(&mut self, mut check: bool) {
        self.check = check.clone();
    }
    

    pub fn isCheck(&mut self) -> bool {
        return self.check.clone();
    }
}

