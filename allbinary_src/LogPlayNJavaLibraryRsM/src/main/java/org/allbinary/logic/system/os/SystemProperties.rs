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
// package org.allbinary.logic.system.os
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

//ActualPlatform
pub struct SystemProperties {
    pub OS_NAME: String,
    pub OS_ARCH: String,
    pub OS_VERSION: String,
}

thread_local! { pub static SystemProperties_instance: Option<Rc<RefCell<SystemProperties>>> = SystemProperties::new(); }

impl SystemProperties {
    

    pub fn getInstance() -> Option<Rc<RefCell<SystemProperties>>> {
        return SystemProperties_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            OS_NAME: String::new(),
            OS_ARCH: String::new(),
            OS_VERSION: String::new(),
        };
        this.OS_NAME = String::from("os.name");
        this.OS_ARCH = String::from("os.arch");
        this.OS_VERSION = String::from("os.version");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        return self.OS_NAME.clone();
    }
    

    pub fn getArch(&mut self) -> String {
        return self.OS_ARCH.clone();
    }
    

    pub fn getVersion(&mut self) -> String {
        return self.OS_VERSION.clone();
    }
    

    pub fn getUserHomePath(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

