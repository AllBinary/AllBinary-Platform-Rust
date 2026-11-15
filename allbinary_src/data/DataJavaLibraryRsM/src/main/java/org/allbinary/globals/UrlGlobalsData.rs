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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UrlGlobalsData {
}

pub static UrlGlobalsData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("URLGLOBALS_NAME"));
pub static UrlGlobalsData_WEBAPPPATH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("URLGLOBALS_WEBAPPPATH"));
pub static UrlGlobalsData_TESTINGMODE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("URLGLOBALS_TESTINGMODE"));
pub static UrlGlobalsData_TESTINGHTMLPATH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("URLGLOBALS_TESTINGHTMLPATH"));
pub static UrlGlobalsData_MAINPATH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("URLGLOBALS_MAINPATH"));

impl UrlGlobalsData {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

