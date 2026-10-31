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
// package org.allbinary.logic.visual.transform.template
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

pub struct XslData {
    pub NAME: String,
    pub ROOT_IMPORT_URI: String,
}

thread_local! { pub static XslData_instance: Option<Rc<RefCell<XslData>>> = XslData::new(); }

impl XslData {
    

    pub fn getInstance() -> Option<Rc<RefCell<XslData>>> {
        return XslData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            ROOT_IMPORT_URI: String::new(),
        };
        this.NAME = String::from("XSL_NAME");
        this.ROOT_IMPORT_URI = String::from("ROOT_IMPORT_URI");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

