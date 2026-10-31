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
// package org.allbinary.android
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::os::Build::Build;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidInfoFactory {
    pub version: i32,
}

thread_local! { pub static AndroidInfoFactory_instance: Option<Rc<RefCell<AndroidInfoFactory>>> = AndroidInfoFactory::new(); }

impl AndroidInfoFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            version: 0,
        };
        this.version = jcall!(unsafe { crate::android::os::Build::Build_VERSION.clone() }).SDK.clone().parse::<i32>().unwrap();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AndroidInfoFactory>>> {
        return AndroidInfoFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getVersion(&mut self) -> i32 {
        return self.version.clone();
    }
}

