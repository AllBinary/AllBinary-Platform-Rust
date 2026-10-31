//
// AllBinary Open License Version 1
// Copyright (c) 2009 AllBinary
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
// package org.allbinary.android.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil;
use crate::org::allbinary::android::AndroidInfoFactory::AndroidInfoFactory;
use crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1;
use crate::org::allbinary::android::api16::ActivityFractureUtilAPI16::ActivityFractureUtilAPI16;
use crate::org::allbinary::android::api19::ActivityFractureUtilAPI19::ActivityFractureUtilAPI19;
use crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ActivityFractureInputUtilFactory {
}


impl ActivityFractureInputUtilFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn>>> {
        let mut SDK_VERSION: i32 = jcall!(AndroidInfoFactory::getInstance()).getVersion();
        if SDK_VERSION <= 4 {
            return ActivityFractureUtilAPI1::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn>>);
        } else if SDK_VERSION <= 15 {
            return ActivityFractureUtilAPI5::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn>>);
        } else if SDK_VERSION <= 16 {
            return ActivityFractureUtilAPI16::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn>>);
        } else {
            return ActivityFractureUtilAPI19::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn>>);
        }
    }
}

