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

pub struct HtmlMetaAttributeValuesData {
    pub ABSTRACT: String,
    pub DESCRIPTION: String,
    pub KEYWORDS: String,
}

thread_local! { pub static HtmlMetaAttributeValuesData_instance: Option<Rc<RefCell<HtmlMetaAttributeValuesData>>> = HtmlMetaAttributeValuesData::new(); }

impl HtmlMetaAttributeValuesData {
    

    pub fn getInstance() -> Option<Rc<RefCell<HtmlMetaAttributeValuesData>>> {
        return HtmlMetaAttributeValuesData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ABSTRACT: String::new(),
            DESCRIPTION: String::new(),
            KEYWORDS: String::new(),
        };
        this.ABSTRACT = String::from("abstract");
        this.DESCRIPTION = String::from("description");
        this.KEYWORDS = String::from("keywords");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

