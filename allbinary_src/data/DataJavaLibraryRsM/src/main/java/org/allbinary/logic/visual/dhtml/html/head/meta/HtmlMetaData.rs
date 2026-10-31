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
// package org.allbinary.logic.visual.dhtml.html.head.meta
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

pub struct HtmlMetaData {
    pub NAME: String,
    pub LABEL: String,
    pub VALUE: String,
}

thread_local! { pub static HtmlMetaData_instance: Option<Rc<RefCell<HtmlMetaData>>> = HtmlMetaData::new(); }

impl HtmlMetaData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            LABEL: String::new(),
            VALUE: String::new(),
        };
        this.NAME = String::from("HTMLMETA_NAME");
        this.LABEL = String::from("HTMLMETA_LABEL");
        this.VALUE = String::from("meta");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HtmlMetaData>>> {
        return HtmlMetaData_instance.with(Clone::clone).clone();
    }
}

