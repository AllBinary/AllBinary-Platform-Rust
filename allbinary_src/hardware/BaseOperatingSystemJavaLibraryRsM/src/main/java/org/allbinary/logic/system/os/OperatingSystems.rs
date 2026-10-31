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

pub struct OperatingSystems {
    pub LINUX: String,
    pub WINDOWS: String,
    pub SOLARIS: String,
    pub WINDOWS2000: String,
    pub WINDOWS_NT: String,
    pub WINDOWS_10: String,
    pub WINDOWS_11: String,
    pub WINDOWS_VISTA: String,
    pub WINDOWS_XP: String,
    pub ANDROID: String,
    pub HARMONY: String,
    pub unknownSpecificOSAllowed: bool,
}

thread_local! { pub static OperatingSystems_instance: Option<Rc<RefCell<OperatingSystems>>> = OperatingSystems::new(); }

impl OperatingSystems {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            LINUX: String::new(),
            WINDOWS: String::new(),
            SOLARIS: String::new(),
            WINDOWS2000: String::new(),
            WINDOWS_NT: String::new(),
            WINDOWS_10: String::new(),
            WINDOWS_11: String::new(),
            WINDOWS_VISTA: String::new(),
            WINDOWS_XP: String::new(),
            ANDROID: String::new(),
            HARMONY: String::new(),
            unknownSpecificOSAllowed: false,
        };
        this.LINUX = String::from("Linux");
        this.WINDOWS = String::from("Windows");
        this.SOLARIS = String::from("Solaris");
        this.WINDOWS2000 = String::from("Windows 2000");
        this.WINDOWS_NT = String::from("Windows NT");
        this.WINDOWS_10 = String::from("Windows 10");
        this.WINDOWS_11 = String::from("Windows 11");
        this.WINDOWS_VISTA = String::from("Windows Vista");
        this.WINDOWS_XP = String::from("Windows XP");
        this.ANDROID = this.LINUX.clone();
        this.HARMONY = String::from("HarmonyOS");
        this.unknownSpecificOSAllowed = true;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isUnknownSpecificOSAllowed(&mut self) -> bool {
        return self.unknownSpecificOSAllowed.clone();
    }
    

    pub fn setUnknownSpecificOSAllowed(&mut self, mut aUnknownSpecificOSAllowed: bool) {
        self.unknownSpecificOSAllowed = aUnknownSpecificOSAllowed.clone();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OperatingSystems>>> {
        return OperatingSystems_instance.with(Clone::clone).clone();
    }
}

