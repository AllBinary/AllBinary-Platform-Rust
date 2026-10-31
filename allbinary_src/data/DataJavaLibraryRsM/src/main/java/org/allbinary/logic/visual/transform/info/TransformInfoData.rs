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
// package org.allbinary.logic.visual.transform.info
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

pub struct TransformInfoData {
    pub PARENT: String,
    pub MAPPED: String,
    pub NAME: String,
    pub DESCRIPTION: String,
    pub CATEGORY: String,
    pub OBJECTFILENAME: String,
    pub OBJECT: String,
    pub OBJECTCONFIGFILENAME: String,
    pub OBJECTCONFIG: String,
    pub TEMPLATEFILENAME: String,
    pub TEMPLATE: String,
    pub DATAFILENAME: String,
    pub DATA: String,
    pub LABEL: String,
    pub OWNER: String,
    pub PARTIAL: String,
}

thread_local! { pub static TransformInfoData_instance: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::new(); }

impl TransformInfoData {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformInfoData>>> {
        return TransformInfoData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            PARENT: String::new(),
            MAPPED: String::new(),
            NAME: String::new(),
            DESCRIPTION: String::new(),
            CATEGORY: String::new(),
            OBJECTFILENAME: String::new(),
            OBJECT: String::new(),
            OBJECTCONFIGFILENAME: String::new(),
            OBJECTCONFIG: String::new(),
            TEMPLATEFILENAME: String::new(),
            TEMPLATE: String::new(),
            DATAFILENAME: String::new(),
            DATA: String::new(),
            LABEL: String::new(),
            OWNER: String::new(),
            PARTIAL: String::new(),
        };
        this.PARENT = String::from("TRANSFORM_INFO_PARENT");
        this.MAPPED = String::from("TRANSFORM_INFO_MAPPED");
        this.NAME = String::from("TRANSFORM_INFO_NAME");
        this.DESCRIPTION = String::from("TRANSFORM_INFO_DESCRIPTION");
        this.CATEGORY = String::from("TRANSFORM_INFO_CATEGORY");
        this.OBJECTFILENAME = String::from("TRANSFORM_INFO_OBJECTFILENAME");
        this.OBJECT = String::from("TRANSFORM_INFO_OBJECT");
        this.OBJECTCONFIGFILENAME = String::from("TRANSFORM_INFO_OBJECTCONFIGFILENAME");
        this.OBJECTCONFIG = String::from("TRANSFORM_INFO_OBJECTCONFIG");
        this.TEMPLATEFILENAME = String::from("TRANSFORM_INFO_TEMPLATEFILENAME");
        this.TEMPLATE = String::from("TRANSFORM_INFO_TEMPLATE");
        this.DATAFILENAME = String::from("TRANSFORM_INFO_DATAFILENAME");
        this.DATA = String::from("TRANSFORM_INFO_DATA");
        this.LABEL = String::from("TRANSFORM_INFO_LABEL");
        this.OWNER = format!("{}{}", String::from("OWNER_"), this.NAME);
        this.PARTIAL = format!("{}{}", String::from("PARTIAL_"), this.NAME);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

