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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::LicenseInitInfo::LicenseInitInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseInitInfoUtil {
    pub INITFILENAME: String,
    pub ABOUT: String,
    pub PRIVACY_POLICY: String,
    pub OFFICIAL_ALLBINARY_SERVICES_SERVER: String,
}

thread_local! { pub static LicenseInitInfoUtil_instance: Option<Rc<RefCell<LicenseInitInfoUtil>>> = LicenseInitInfoUtil::new(); }

impl LicenseInitInfoUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<LicenseInitInfoUtil>>> {
        return LicenseInitInfoUtil_instance.with(Clone::clone).clone();
    }
    

    // synchronized
    pub fn setFilePath(&mut self, mut filePath: String) {
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            INITFILENAME: String::new(),
            ABOUT: String::new(),
            PRIVACY_POLICY: String::new(),
            OFFICIAL_ALLBINARY_SERVICES_SERVER: String::new(),
        };
        this.INITFILENAME = String::from("licenseinitdata.dat");
        this.ABOUT = String::from("about");
        this.PRIVACY_POLICY = String::from("privacy_policy");
        this.OFFICIAL_ALLBINARY_SERVICES_SERVER = String::from("https://services.allbinary.com/LicServ/serverssl.php");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn write(&mut self, mut initData: Option<Rc<RefCell<LicenseInitInfo>>>) {
    }
    

    // throws Exception
    // synchronized
    pub fn read(&mut self) -> Option<Rc<RefCell<LicenseInitInfo>>> {
        return self.readAgain(0);
    }
    

    // throws Exception
    // synchronized
    pub fn readAgain(&mut self, mut initializeCounter: i32) -> Option<Rc<RefCell<LicenseInitInfo>>> {
        let mut licenseInitInfo: Option<Rc<RefCell<LicenseInitInfo>>> = LicenseInitInfo::new();
        jcall!(licenseInitInfo).clearServers();
        jcall!(licenseInitInfo).addServer(self.OFFICIAL_ALLBINARY_SERVICES_SERVER.clone());
        return licenseInitInfo.clone();
    }
}

