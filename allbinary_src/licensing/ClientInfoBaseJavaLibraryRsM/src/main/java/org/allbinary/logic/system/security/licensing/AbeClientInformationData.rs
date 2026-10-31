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
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbeClientInformationData {
    pub KEY: String,
    pub NAME: String,
    pub VERSION: String,
    pub SPECIALNAME: String,
    pub LICENSEID: String,
    pub LICENSE_TYPE: String,
    pub PREVIOUSLICENSEID: String,
    pub OSNAME: String,
    pub OSARCH: String,
    pub OSVERSION: String,
    pub OS: String,
    pub HARDWARE: String,
    pub LICENSESERVERS: String,
    pub NEWLICENSE: String,
    pub ISNEW: String,
    pub SPECIAL: String,
}

thread_local! { pub static AbeClientInformationData_instance: Option<Rc<RefCell<AbeClientInformationData>>> = AbeClientInformationData::new(); }

impl AbeClientInformationData {
    

    pub fn getInstance() -> Option<Rc<RefCell<AbeClientInformationData>>> {
        return AbeClientInformationData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            KEY: String::new(),
            NAME: String::new(),
            VERSION: String::new(),
            SPECIALNAME: String::new(),
            LICENSEID: String::new(),
            LICENSE_TYPE: String::new(),
            PREVIOUSLICENSEID: String::new(),
            OSNAME: String::new(),
            OSARCH: String::new(),
            OSVERSION: String::new(),
            OS: String::new(),
            HARDWARE: String::new(),
            LICENSESERVERS: String::new(),
            NEWLICENSE: String::new(),
            ISNEW: String::new(),
            SPECIAL: String::new(),
        };
        this.KEY = String::from("KEY");
        this.NAME = String::from("NAME");
        this.VERSION = String::from("VERSION");
        this.SPECIALNAME = String::from("SPECIALNAME");
        this.LICENSEID = String::from("LICENSEID");
        this.LICENSE_TYPE = String::from("LICENSETYPE");
        this.PREVIOUSLICENSEID = String::from("PREVIOUSLICENSEID");
        this.OSNAME = String::from("OSNAME");
        this.OSARCH = String::from("OSARCH");
        this.OSVERSION = String::from("OSVERSION");
        this.OS = String::from("OS");
        this.HARDWARE = String::from("HARDWARE");
        this.LICENSESERVERS = String::from("LICENSESERVERS");
        this.NEWLICENSE = String::from("NEWLICENSE");
        this.ISNEW = String::from("ISNEW");
        this.SPECIAL = String::from("SPECIAL");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

