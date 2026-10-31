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
// package org.allbinary.globals
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::UrlGlobalsInterface::UrlGlobalsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct URLGLOBALS {
}

pub static mut URLGLOBALS_urlGlobalsInterface: std::cell::LazyCell<Option<Rc<RefCell<dyn UrlGlobalsInterface>>>> = std::cell::LazyCell::new(|| AppUrlGlobals_NULL_APP_URL_GLOBALS.with(Clone::clone).clone());

impl URLGLOBALS {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(mut urlGlobalsInterface: Option<Rc<RefCell<dyn UrlGlobalsInterface>>>) {
        unsafe { *URLGLOBALS_urlGlobalsInterface = urlGlobalsInterface.clone() };
    }
    

    pub fn isTestingMode() -> bool {
        return jcall!(unsafe { URLGLOBALS_urlGlobalsInterface.clone() }).isTestingMode();
    }
    

    pub fn getWebappPath() -> String {
        return jcall!(unsafe { URLGLOBALS_urlGlobalsInterface.clone() }).getWebappPath();
    }
    

    pub fn getTestHtmlPath() -> String {
        return jcall!(unsafe { URLGLOBALS_urlGlobalsInterface.clone() }).getTestHtmlPath();
    }
    

    // throws Exception
    pub fn getMainPath() -> String {
        return jcall!(unsafe { URLGLOBALS_urlGlobalsInterface.clone() }).getMainPath();
    }
}

