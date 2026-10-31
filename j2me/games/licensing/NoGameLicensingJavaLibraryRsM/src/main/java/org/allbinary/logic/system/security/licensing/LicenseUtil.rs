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
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseUtil {
}

thread_local! { pub static LicenseUtil_instance: Option<Rc<RefCell<LicenseUtil>>> = LicenseUtil::new(); }

impl LicenseUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LicenseUtil>>> {
        return LicenseUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return false;
    }
    

    // synchronized
    pub fn setRunning(&mut self, mut running: bool) {
    }
    

    pub fn shouldWait(&mut self, mut commandListener: Option<Rc<RefCell<CommandListener>>>) -> bool {
        return self.isRunning();
    }
}

