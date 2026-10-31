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
use crate::android::app::Activity::Activity;
use crate::android::os::Build::Build;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidUtil {
}

thread_local! { pub static AndroidUtil_NULL_ACTIVITY: Option<Rc<RefCell<Activity>>> = Activity::new(); }
thread_local! { pub static AndroidUtil_version: i32 = jcall!(unsafe { crate::android::os::Build::Build_VERSION.clone() }).SDK.clone().parse::<i32>().unwrap(); }

impl AndroidUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isMemoryRestrictive() -> bool {
        if AndroidUtil_version.with(Clone::clone) < 30 {
            return true;
        }
        return false;
    }
    

    pub fn isAndroid() -> bool {
        return true;
    }
}

