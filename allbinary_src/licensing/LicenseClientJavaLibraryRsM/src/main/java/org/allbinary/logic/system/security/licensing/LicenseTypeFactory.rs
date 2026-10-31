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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::LicenseType::LicenseType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseTypeFactory {
    pub UNKNOWN: Option<Rc<RefCell<LicenseType>>>,
    pub DEMO: Option<Rc<RefCell<LicenseType>>>,
    pub NORMAL: Option<Rc<RefCell<LicenseType>>>,
}

thread_local! { pub static LicenseTypeFactory_instance: Option<Rc<RefCell<LicenseTypeFactory>>> = LicenseTypeFactory::new(); }

impl LicenseTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            UNKNOWN: None,
            DEMO: None,
            NORMAL: None,
        };
        this.UNKNOWN = LicenseType::new(jcall!(CommonStrings::getInstance()).UNKNOWN.clone());
        this.DEMO = LicenseType::new(String::from("Demo"));
        this.NORMAL = LicenseType::new(String::from("Normal"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LicenseTypeFactory>>> {
        return LicenseTypeFactory_instance.with(Clone::clone).clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut name: String) -> Option<Rc<RefCell<LicenseType>>> {
        if jcall!(jcall!(self.DEMO).getName()).compareTo(name.clone()) == 0 {
            return self.DEMO.clone();
        } else if jcall!(jcall!(self.NORMAL).getName()).compareTo(name.clone()) == 0 {
            return self.NORMAL.clone();
        }
        return self.UNKNOWN.clone();
    }
}

