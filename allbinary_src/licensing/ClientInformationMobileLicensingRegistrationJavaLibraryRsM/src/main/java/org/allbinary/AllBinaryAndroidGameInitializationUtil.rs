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
// package org.allbinary
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::init::LicenseInitInfoUtil::LicenseInitInfoUtil;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::system::security::licensing::PartnerIdentifierFileUtil::PartnerIdentifierFileUtil;
use super::AndroidResources::AndroidResources;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryAndroidGameInitializationUtil {
}


impl AllBinaryAndroidGameInitializationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init() {
        let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
        let mut androidResources: Option<Rc<RefCell<AndroidResources>>> = AndroidResources::getInstance();
        let mut licenseInitInfoUtil: Option<Rc<RefCell<LicenseInitInfoUtil>>> = LicenseInitInfoUtil::getInstance();
        jcall!(resourceUtil).addResource(jcall!(licenseInitInfoUtil).INITFILENAME.clone(), (jcall!(jcall!(androidResources).raw).licenseinitdata.clone()).to_string());
        jcall!(resourceUtil).addResource(jcall!(PartnerIdentifierFileUtil::getInstance()).FILE_NAME.clone(), (jcall!(jcall!(androidResources).raw).partner.clone()).to_string());
    }
}

