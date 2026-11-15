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
use crate::android::content::Context::Context;
use crate::android::os::Build::Build;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidSystemProperties {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

pub static AndroidSystemProperties_SINGLETON: crate::JavaStatic<Option<Rc<RefCell<AndroidSystemProperties>>>> = crate::JavaStatic::new(|| AndroidSystemProperties::new(jcall!(ResourceUtil::getInstance()).getContext()));

impl AndroidSystemProperties {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AndroidSystemProperties>>> {
        return AndroidSystemProperties_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct(mut context: Option<Rc<RefCell<Context>>>) -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new(mut context: Option<Rc<RefCell<Context>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(context))))
    }
    

    pub fn getDeviceId(&mut self) -> String {
        return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
    }
    

    pub fn getDeviceSoftwareVersion(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getLine1Number(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getNetworkCountryIso(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getNetworkOperator(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getNetworkOperatorName(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getSimCountryIso(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getSimOperator(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getSimOperatorName(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getSimSerialNumber(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getSubscriberId(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getVoiceMailAlphaTag(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getVoiceMailNumber(&mut self) -> String {
        return { let java_field = jcall!(self.commonStrings).DISABLE.clone(); java_field }.clone();
    }
    

    pub fn getNetworkType(&mut self) -> i32 {
        return -1;
    }
    

    pub fn getPhoneType(&mut self) -> i32 {
        return -1;
    }
    

    pub fn getBoard(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_BOARD.clone() }.clone();
    }
    

    pub fn getBrand(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_BRAND.clone() }.clone();
    }
    

    pub fn getDevice(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_DEVICE.clone() }.clone();
    }
    

    pub fn getFingerprint(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_FINGERPRINT.clone() }.clone();
    }
    

    pub fn getHost(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_HOST.clone() }.clone();
    }
    

    pub fn getId(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_ID.clone() }.clone();
    }
    

    pub fn getModel(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_MODEL.clone() }.clone();
    }
    

    pub fn getProduct(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_PRODUCT.clone() }.clone();
    }
    

    pub fn getTags(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_TAGS.clone() }.clone();
    }
    

    pub fn getTime(&mut self) -> i64 {
        return unsafe { crate::android::os::Build::Build_TIME.clone() }.clone();
    }
    

    pub fn getType(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_TYPE.clone() }.clone();
    }
    

    pub fn getUser(&mut self) -> String {
        return unsafe { crate::android::os::Build::Build_USER.clone() }.clone();
    }
}

