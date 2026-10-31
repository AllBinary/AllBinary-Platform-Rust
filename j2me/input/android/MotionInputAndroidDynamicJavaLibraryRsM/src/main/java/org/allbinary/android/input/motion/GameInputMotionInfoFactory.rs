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
use crate::org::allbinary::android::AndroidInfoFactory::AndroidInfoFactory;
use crate::org::allbinary::android::input::motion::api1::GameInputMotionInfoAPI1::GameInputMotionInfoAPI1;
use crate::org::allbinary::android::input::motion::api5::GameInputMotionInfoAPI5::GameInputMotionInfoAPI5;
use super::GameInputMotionInfo::GameInputMotionInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputMotionInfoFactory {
}


impl GameInputMotionInfoFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::android::input::motion::GameInputMotionInfo::GameInputMotionInfoDyn>>> {
        let mut SDK_VERSION: i32 = jcall!(AndroidInfoFactory::getInstance()).getVersion();
        if SDK_VERSION <= 4 {
            return GameInputMotionInfoAPI1::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::android::input::motion::GameInputMotionInfo::GameInputMotionInfoDyn>>);
        } else {
            return GameInputMotionInfoAPI5::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::android::input::motion::GameInputMotionInfo::GameInputMotionInfoDyn>>);
        }
    }
}

