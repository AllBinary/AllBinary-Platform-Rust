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
// package org.allbinary.business.context.modules.storefront
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

pub struct StoreFrontsData {
    pub TYPE: String,
    pub RETAIL: String,
    pub SUBSCRIPTION: String,
    pub AUCTION: String,
    pub NEWS: String,
    pub BLOG: String,
    pub FORUM: String,
}

thread_local! { pub static StoreFrontsData_instance: Option<Rc<RefCell<StoreFrontsData>>> = StoreFrontsData::new(); }

impl StoreFrontsData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            TYPE: String::new(),
            RETAIL: String::new(),
            SUBSCRIPTION: String::new(),
            AUCTION: String::new(),
            NEWS: String::new(),
            BLOG: String::new(),
            FORUM: String::new(),
        };
        this.TYPE = String::from("STOREFRONTS_TYPE");
        this.RETAIL = String::from("STOREFRONTS_RETAIL");
        this.SUBSCRIPTION = String::from("STOREFRONTS_SUBSCRIPTION");
        this.AUCTION = String::from("STOREFRONTS_AUCTION");
        this.NEWS = String::from("STOREFRONTS_NEWS");
        this.BLOG = String::from("STOREFRONTS_BLOG");
        this.FORUM = String::from("STOREFRONTS_FORUM");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<StoreFrontsData>>> {
        return StoreFrontsData_instance.with(Clone::clone).clone();
    }
}

