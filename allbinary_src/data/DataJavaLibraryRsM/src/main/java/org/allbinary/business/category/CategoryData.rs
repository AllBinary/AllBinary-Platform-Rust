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
// package org.allbinary.business.category
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::CommonDataFileStrings::CommonDataFileStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryData {
    pub PARENT: String,
    pub NAME: String,
    pub PATH: String,
    pub LABEL: String,
    pub VIEW: String,
    pub LEVEL: String,
    pub REQUEST: String,
    pub ROOTCATEGORY: String,
    pub UNCRYPTED_EXTENSION: String,
    pub ENCRYPTED_EXTENSION: String,
}

thread_local! { pub static CategoryData_instance: Option<Rc<RefCell<CategoryData>>> = CategoryData::new(); }

impl CategoryData {
    

    pub fn getInstance() -> Option<Rc<RefCell<CategoryData>>> {
        return CategoryData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            PARENT: String::new(),
            NAME: String::new(),
            PATH: String::new(),
            LABEL: String::new(),
            VIEW: String::new(),
            LEVEL: String::new(),
            REQUEST: String::new(),
            ROOTCATEGORY: String::new(),
            UNCRYPTED_EXTENSION: String::new(),
            ENCRYPTED_EXTENSION: String::new(),
        };
        this.PARENT = String::from("CATEGORY_PARENT");
        this.NAME = String::from("CATEGORY_NAME");
        this.PATH = String::from("CATEGORY_PATH");
        this.LABEL = String::from("CATEGORY_LABEL");
        this.VIEW = String::from("CATEGORY_VIEW_CATEGORY");
        this.LEVEL = String::from("CATEGORY_LEVEL");
        this.REQUEST = String::from("request");
        this.ROOTCATEGORY = String::from("Category");
        this.UNCRYPTED_EXTENSION = jcall!(CommonDataFileStrings::getInstance()).UNCRYPTED_EXTENSION.clone();
        this.ENCRYPTED_EXTENSION = jcall!(CommonDataFileStrings::getInstance()).ENCRYPTED_EXTENSION.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

