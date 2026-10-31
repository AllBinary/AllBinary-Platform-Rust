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
// package org.allbinary.logic.visual.dhtml.style.css.property
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

pub struct CssPropertyData {
    pub NAME: String,
    pub VALUES: String,
}

thread_local! { pub static CssPropertyData_instance: Option<Rc<RefCell<CssPropertyData>>> = CssPropertyData::new(); }

impl CssPropertyData {
    

    pub fn getInstance() -> Option<Rc<RefCell<CssPropertyData>>> {
        return CssPropertyData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            VALUES: String::new(),
        };
        this.NAME = String::from("CSS_PROPERTY_NAME");
        this.VALUES = String::from("CSS_PROPERTY_VALUES");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

