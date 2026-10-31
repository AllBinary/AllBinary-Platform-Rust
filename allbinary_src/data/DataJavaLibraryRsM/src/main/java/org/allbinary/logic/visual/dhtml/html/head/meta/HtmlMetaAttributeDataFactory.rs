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
use super::HtmlMetaAttributeData::HtmlMetaAttributeData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlMetaAttributeDataFactory {
    pub NAME: Option<Rc<RefCell<HtmlMetaAttributeData>>>,
    pub HTTP_EQUIV: Option<Rc<RefCell<HtmlMetaAttributeData>>>,
    pub CONTENT: Option<Rc<RefCell<HtmlMetaAttributeData>>>,
}

thread_local! { pub static HtmlMetaAttributeDataFactory_instance: Option<Rc<RefCell<HtmlMetaAttributeDataFactory>>> = HtmlMetaAttributeDataFactory::new(); }

impl HtmlMetaAttributeDataFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<HtmlMetaAttributeDataFactory>>> {
        return HtmlMetaAttributeDataFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: None,
            HTTP_EQUIV: None,
            CONTENT: None,
        };
        this.NAME = HtmlMetaAttributeData::new(0, String::from("HTMLMETA_ATTRIBUTE_NAME"));
        this.HTTP_EQUIV = HtmlMetaAttributeData::new(1, String::from("HTMLMETA_ATTRIBUTE_HTTPEQUIV"));
        this.CONTENT = HtmlMetaAttributeData::new(2, String::from("HTMLMETA_ATTRIBUTE_CONTENT"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

