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
// package org.allbinary.logic.io
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

pub struct InputOutputTypeData {
    pub NAME: String,
    pub FILE: String,
    pub RESPONSE: String,
    pub DB: String,
    pub JSP: String,
    pub JSP_FRAGMENT: String,
    pub PHP: String,
    pub ASP: String,
    pub PL: String,
    pub DEFAULT: String,
    pub DEFAULT_FRAGMENT: String,
}

thread_local! { pub static InputOutputTypeData_instance: Option<Rc<RefCell<InputOutputTypeData>>> = InputOutputTypeData::new(); }

impl InputOutputTypeData {
    

    pub fn getInstance() -> Option<Rc<RefCell<InputOutputTypeData>>> {
        return InputOutputTypeData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            FILE: String::new(),
            RESPONSE: String::new(),
            DB: String::new(),
            JSP: String::new(),
            JSP_FRAGMENT: String::new(),
            PHP: String::new(),
            ASP: String::new(),
            PL: String::new(),
            DEFAULT: String::new(),
            DEFAULT_FRAGMENT: String::new(),
        };
        this.NAME = String::from("INPUT_OUTPUT_TYPE_NAME");
        this.FILE = String::from("INPUT_OUTPUT_TYPE_FILE");
        this.RESPONSE = String::from("INPUT_OUTPUT_TYPE_RESPONSE");
        this.DB = String::from("INPUT_OUTPUT_TYPE_DB");
        this.JSP = String::from("jsp");
        this.JSP_FRAGMENT = String::from("jspf");
        this.PHP = String::from("php");
        this.ASP = String::from("asp");
        this.PL = String::from("pl");
        this.DEFAULT = this.JSP.clone();
        this.DEFAULT_FRAGMENT = this.JSP_FRAGMENT.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

